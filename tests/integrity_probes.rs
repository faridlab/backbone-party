//! Council integrity probes — route-level. The guarded composition locks generic writes and
//! enforces validation on the sanctioned create path. Hits routes via tower oneshot.
//! Requires DATABASE_URL (defaults to local dev Postgres on :5433).

use axum::body::Body;
use axum::http::{Request, StatusCode};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

use backbone_party::{create_guarded_party_routes, PartyModule};

async fn pool() -> PgPool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5433/backbone_party".to_string());
    PgPool::connect(&url).await.unwrap()
}
async fn module(pool: &PgPool) -> PartyModule {
    PartyModule::builder().with_database(pool.clone()).build().unwrap()
}
async fn post(app: axum::Router, uri: &str, body: String) -> StatusCode {
    app.oneshot(
        Request::builder().method("POST").uri(uri)
            .header("content-type", "application/json").body(Body::from(body)).unwrap(),
    ).await.unwrap().status()
}
fn uq(p: &str) -> String { format!("{p}-{}", &Uuid::new_v4().simple().to_string()[..8]) }

// IGC-1: generic bulk party create is not exposed on the guarded surface.
#[tokio::test]
async fn guarded_routes_lock_generic_party_bulk() {
    let pool = pool().await;
    let body = format!(r#"{{"partyCode":"{}","partyKind":"organization","name":"X","status":"active"}}"#, uq("BYP"));
    let status = post(create_guarded_party_routes(&module(&pool).await), "/parties/bulk", body).await;
    assert!(
        status == StatusCode::METHOD_NOT_ALLOWED || status == StatusCode::NOT_FOUND,
        "generic bulk party create must not be exposed; got {status}"
    );
}

// IGC-2: validated party create rejects a malformed NPWP.
#[tokio::test]
async fn guarded_party_rejects_bad_npwp() {
    let pool = pool().await;
    let body = format!(r#"{{"partyCode":"{}","name":"PT X","npwp":"123"}}"#, uq("SKU"));
    let status = post(create_guarded_party_routes(&module(&pool).await), "/parties", body).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

// IGC-3: adding an address to a non-existent party is rejected.
#[tokio::test]
async fn guarded_address_rejects_missing_party() {
    let pool = pool().await;
    let body = format!(r#"{{"partyId":"{}","line1":"Jl. Test"}}"#, Uuid::new_v4());
    let status = post(create_guarded_party_routes(&module(&pool).await), "/party-addresses", body).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

// IGC-4: valid party + geo-linked address succeed through the guarded surface.
#[tokio::test]
async fn guarded_valid_writes_succeed() {
    let pool = pool().await;
    let code = uq("OK");
    let ps = post(create_guarded_party_routes(&module(&pool).await), "/parties",
        format!(r#"{{"partyCode":"{code}","name":"PT Ok","legalName":"PT Ok Indonesia"}}"#)).await;
    assert_eq!(ps, StatusCode::CREATED);

    let pid: Uuid = sqlx::query_scalar("SELECT id FROM party.parties WHERE party_code=$1")
        .bind(&code).fetch_one(&pool).await.unwrap();
    let geo_sub = Uuid::new_v4();
    let abody = format!(
        r#"{{"partyId":"{pid}","addressType":"billing","line1":"Jl. Merdeka 1","subdistrictId":"{geo_sub}","postalCode":"10110","isPrimary":true}}"#
    );
    let ascode = post(create_guarded_party_routes(&module(&pool).await), "/party-addresses", abody).await;
    assert_eq!(ascode, StatusCode::CREATED);
}

// ── Council 2026-07-02 fixes: one-primary + kind/field consistency ─────────────────

async fn mk_party(pool: &PgPool, code: &str, extra: &str) -> Uuid {
    let body = format!(r#"{{"partyCode":"{code}","name":"PT X","legalName":"PT X Indonesia"{extra}}}"#);
    let s = post(create_guarded_party_routes(&module(pool).await), "/parties", body).await;
    assert_eq!(s, StatusCode::CREATED, "seed party create");
    sqlx::query_scalar("SELECT id FROM party.parties WHERE party_code=$1").bind(code).fetch_one(pool).await.unwrap()
}

// IGC-5: a second primary of the same kind is rejected (one-primary-per-party).
#[tokio::test]
async fn guarded_rejects_second_primary_address() {
    let pool = pool().await;
    let pid = mk_party(&pool, &uq("PRM"), "").await;
    let a1 = format!(r#"{{"partyId":"{pid}","line1":"Jl. A","isPrimary":true}}"#);
    let s1 = post(create_guarded_party_routes(&module(&pool).await), "/party-addresses", a1).await;
    assert_eq!(s1, StatusCode::CREATED);
    let a2 = format!(r#"{{"partyId":"{pid}","line1":"Jl. B","isPrimary":true}}"#);
    let s2 = post(create_guarded_party_routes(&module(&pool).await), "/party-addresses", a2).await;
    assert_eq!(s2, StatusCode::UNPROCESSABLE_ENTITY, "second primary address must be rejected");
    // non-primary is fine
    let a3 = format!(r#"{{"partyId":"{pid}","line1":"Jl. C"}}"#);
    let s3 = post(create_guarded_party_routes(&module(&pool).await), "/party-addresses", a3).await;
    assert_eq!(s3, StatusCode::CREATED, "additional non-primary address is allowed");
}

// IGC-6: kind/field consistency — person needs a name part; org cannot carry a NIK.
#[tokio::test]
async fn guarded_enforces_kind_field_consistency() {
    let pool = pool().await;
    // person with no first/last name → 422
    let person = format!(r#"{{"partyCode":"{}","partyKind":"person","name":"x"}}"#, uq("PER"));
    let s1 = post(create_guarded_party_routes(&module(&pool).await), "/parties", person).await;
    assert_eq!(s1, StatusCode::UNPROCESSABLE_ENTITY, "person without name parts must be rejected");
    // organization carrying a NIK → 422
    let org = format!(r#"{{"partyCode":"{}","name":"PT Y","legalName":"PT Y","nik":"3201234567890123"}}"#, uq("ORG"));
    let s2 = post(create_guarded_party_routes(&module(&pool).await), "/parties", org).await;
    assert_eq!(s2, StatusCode::UNPROCESSABLE_ENTITY, "org carrying a NIK must be rejected");
}

// IGC-7: set-primary switches which child is primary (keeps the invariant switchable).
#[tokio::test]
async fn guarded_set_primary_switches() {
    let pool = pool().await;
    let pid = mk_party(&pool, &uq("SW"), "").await;
    // one primary + one non-primary
    post(create_guarded_party_routes(&module(&pool).await), "/party-addresses",
        format!(r#"{{"partyId":"{pid}","line1":"Jl. First","isPrimary":true}}"#)).await;
    post(create_guarded_party_routes(&module(&pool).await), "/party-addresses",
        format!(r#"{{"partyId":"{pid}","line1":"Jl. Second"}}"#)).await;
    let second: Uuid = sqlx::query_scalar("SELECT id FROM party.party_addresses WHERE party_id=$1 AND line1='Jl. Second'")
        .bind(pid).fetch_one(&pool).await.unwrap();
    // promote the second → succeeds (old primary cleared in one tx)
    let body = format!(r#"{{"partyId":"{pid}","kind":"address","childId":"{second}"}}"#);
    let s = post(create_guarded_party_routes(&module(&pool).await), "/party-set-primary", body).await;
    assert_eq!(s, StatusCode::OK);
    let n_primary: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM party.party_addresses WHERE party_id=$1 AND is_primary")
        .bind(pid).fetch_one(&pool).await.unwrap();
    assert_eq!(n_primary, 1, "exactly one primary after switch");
    let is_second_primary: bool = sqlx::query_scalar("SELECT is_primary FROM party.party_addresses WHERE id=$1")
        .bind(second).fetch_one(&pool).await.unwrap();
    assert!(is_second_primary, "the promoted address is now primary");
}
