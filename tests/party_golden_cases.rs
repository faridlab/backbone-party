//! Golden-case tests for the party validated write path.
//! Proves PartyWriteService enforces NPWP/NIK format+uniqueness and party existence,
//! and that an address links to backbone-geo by logical FK (opaque uuid, no cross-module join).
//! Requires DATABASE_URL (defaults to local dev Postgres on :5433).

use sqlx::PgPool;
use uuid::Uuid;

use backbone_party::{
    validate_nik, validate_npwp, NewAddress, NewEmail, NewParty, NewPhone, PartyWriteError,
    PartyWriteService,
};

async fn pool() -> PgPool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5433/backbone_party".to_string());
    PgPool::connect(&url).await.unwrap()
}
fn uq(p: &str) -> String {
    format!("{p}-{}", &Uuid::new_v4().simple().to_string()[..8])
}
fn npwp() -> String {
    let hex = Uuid::new_v4().simple().to_string();
    let d: String = hex.chars().filter(|c| c.is_ascii_digit()).take(15).collect();
    format!("{d:0<15}")
}
fn nik() -> String {
    let hex = Uuid::new_v4().simple().to_string();
    let d: String = hex.chars().filter(|c| c.is_ascii_digit()).take(16).collect();
    format!("{d:0<16}")
}

fn party(code: &str) -> NewParty {
    NewParty {
        company_id: Uuid::nil(),
        party_code: code.to_string(),
        party_kind: Some("organization".into()),
        name: "PT Test".into(),
        legal_name: Some("PT Test Indonesia".into()),
        first_name: None,
        last_name: None,
        npwp: None,
        nik: None,
    }
}

// PGC-1: create an organization party.
#[tokio::test]
async fn create_organization_party() {
    let pool = pool().await;
    let svc = PartyWriteService::new(pool.clone());
    let code = uq("ACME");
    let mut p = party(&code);
    p.npwp = Some(npwp());
    let id = svc.create_party(p).await.expect("party");
    let (kind, status): (String, String) = sqlx::query_as(
        "SELECT party_kind::text, status::text FROM party.parties WHERE id=$1",
    )
    .bind(id).fetch_one(&pool).await.unwrap();
    assert_eq!(kind, "organization");
    assert_eq!(status, "active");
}

// PGC-2: invalid NPWP / NIK rejected, nothing written.
#[tokio::test]
async fn rejects_invalid_npwp_and_nik() {
    let pool = pool().await;
    let svc = PartyWriteService::new(pool.clone());
    let mut bad = party(&uq("BAD"));
    bad.npwp = Some("123".into());
    assert!(matches!(svc.create_party(bad).await.unwrap_err(), PartyWriteError::InvalidNpwp(_)));

    let mut bad2 = party(&uq("BAD2"));
    bad2.party_kind = Some("person".into());
    bad2.nik = Some("12345".into());
    assert!(matches!(svc.create_party(bad2).await.unwrap_err(), PartyWriteError::InvalidNik(_)));
}

// PGC-3: duplicate party_code and duplicate npwp rejected distinctly.
#[tokio::test]
async fn rejects_duplicate_code_and_npwp() {
    let pool = pool().await;
    let svc = PartyWriteService::new(pool.clone());
    let code = uq("DUP");
    svc.create_party(party(&code)).await.expect("first");
    assert!(matches!(svc.create_party(party(&code)).await.unwrap_err(), PartyWriteError::DuplicateCode(_)));

    let shared = npwp();
    let mut a = party(&uq("NPA")); a.npwp = Some(shared.clone());
    svc.create_party(a).await.expect("npwp a");
    let mut b = party(&uq("NPB")); b.npwp = Some(shared);
    assert!(matches!(svc.create_party(b).await.unwrap_err(), PartyWriteError::DuplicateNpwp(_)));
}

// PGC-4: a child (address/email/phone) requires an existing party.
#[tokio::test]
async fn children_require_existing_party() {
    let pool = pool().await;
    let svc = PartyWriteService::new(pool.clone());
    let mut addr = NewAddress::default();
    addr.party_id = Uuid::new_v4();
    addr.line1 = "Jl. Test 1".into();
    assert!(matches!(svc.add_address(addr).await.unwrap_err(), PartyWriteError::PartyNotFound(_)));

    let e = NewEmail { company_id: Uuid::nil(), party_id: Uuid::new_v4(), label: None, email: "a@b.com".into(), is_primary: true };
    assert!(matches!(svc.add_email(e).await.unwrap_err(), PartyWriteError::PartyNotFound(_)));
    let ph = NewPhone { company_id: Uuid::nil(), party_id: Uuid::new_v4(), label: None, phone: "0811".into(), is_primary: true };
    assert!(matches!(svc.add_phone(ph).await.unwrap_err(), PartyWriteError::PartyNotFound(_)));
}

// PGC-5: an address stores its geo subdistrict_id as an OPAQUE logical FK (no cross-module join).
#[tokio::test]
async fn address_links_geo_by_logical_fk() {
    let pool = pool().await;
    let svc = PartyWriteService::new(pool.clone());
    let pid = svc.create_party(party(&uq("GEOP"))).await.expect("party");
    // A geo subdistrict id from backbone-geo — party stores it verbatim, never joins to geo.
    let geo_subdistrict = Uuid::new_v4();
    let mut addr = NewAddress::default();
    addr.party_id = pid;
    addr.address_type = Some("shipping".into());
    addr.line1 = "Jl. Merdeka 10".into();
    addr.subdistrict_id = Some(geo_subdistrict);
    addr.postal_code = Some("10110".into());
    addr.is_primary = true;
    addr.is_shipping = true;
    let aid = svc.add_address(addr).await.expect("address");

    let stored: Option<Uuid> =
        sqlx::query_scalar("SELECT subdistrict_id FROM party.party_addresses WHERE id=$1")
            .bind(aid).fetch_one(&pool).await.unwrap();
    assert_eq!(stored, Some(geo_subdistrict), "geo subdistrict id stored as-is (logical FK)");
}

// PGC-6 (unit): NPWP/NIK validators.
#[test]
fn validators() {
    assert!(validate_npwp("01.234.567.8-901.000")); // 15
    assert!(validate_npwp("0123456789012345")); // 16
    assert!(!validate_npwp("12345"));
    assert!(validate_nik("3201234567890123")); // 16
    assert!(!validate_nik("12345"));
}
