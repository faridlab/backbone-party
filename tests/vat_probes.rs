//! VAT validation probes for the party write path (hand-authored, user-owned).
//!
//! Proves the fail-closed posture end-to-end against a migrated database:
//!   - a valid country-prefixed VAT number is accepted and stored in canonical form;
//!   - the Belgian mod-97 checksum is enforced;
//!   - an UNKNOWN-country prefix is refused with the distinct `vat_unknown_country` error
//!     and NOTHING is written (the fail-closed inversion: Odoo's base_vat accepts these);
//!   - the '/' no-VAT sentinel is accepted and stored verbatim;
//!   - the NAMED configuration escape (`VatValidationPolicy::ALLOW_UNKNOWN_COUNTRIES`)
//!     accepts unknown-country numbers — and still refuses malformed known-country ones;
//!   - a malformed known-country number and a prefix-less number are both refused.
//!
//! Requires DATABASE_URL (defaults to local dev Postgres on :5433/backbone_party).

#![expect(clippy::expect_used, reason = "test harness: a panic here names the setup failure precisely")]
use sqlx::PgPool;
use uuid::Uuid;

use backbone_party::{
    NewParty, PartyWriteError, PartyWriteService, VatValidationPolicy, NO_VAT_SENTINEL,
};

async fn pool() -> PgPool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5433/backbone_party".to_string());
    PgPool::connect(&url).await.unwrap()
}

fn uq(p: &str) -> String {
    format!("{p}-{}", &Uuid::new_v4().simple().to_string()[..8])
}

fn party_with_vat(code: &str, vat: &str) -> NewParty {
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
        vat: Some(vat.into()),
    }
}

async fn stored_vat(pool: &PgPool, id: Uuid) -> Option<String> {
    let row: (Option<String>,) = sqlx::query_as("SELECT vat FROM party.parties WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap();
    row.0
}

// VAT-P1: a valid VAT number is accepted and stored in canonical (uppercased, separator-free) form.
#[tokio::test]
async fn valid_vat_accepted_and_canonicalized() {
    let pool = pool().await;
    let svc = PartyWriteService::new(pool.clone());
    let id = svc
        .create_party(party_with_vat(&uq("VAT"), "de 123.456-789"))
        .await
        .expect("valid VAT must be accepted");
    assert_eq!(stored_vat(&pool, id).await.as_deref(), Some("DE123456789"));
}

// VAT-P2: the Belgian mod-97 checksum is enforced — one digit off is refused, nothing written.
#[tokio::test]
async fn belgian_mod97_checksum_enforced() {
    let pool = pool().await;
    let svc = PartyWriteService::new(pool.clone());
    let id = svc
        .create_party(party_with_vat(&uq("BEG"), "BE0428759497"))
        .await
        .expect("checksum-valid BE number accepted");
    assert_eq!(stored_vat(&pool, id).await.as_deref(), Some("BE0428759497"));

    let err = svc
        .create_party(party_with_vat(&uq("BEB"), "BE0428759498"))
        .await
        .unwrap_err();
    assert!(matches!(err, PartyWriteError::InvalidVat(_)));
    let n: (i64,) = sqlx::query_as("SELECT count(*) FROM party.parties WHERE vat='BE0428759498'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(n.0, 0, "a refused VAT number must not be written");
}

// VAT-P3: an unknown-country prefix is refused fail-closed with the distinct error code,
// and no row is written. (Odoo's base_vat accepts these after a generic check — inverted here.)
#[tokio::test]
async fn unknown_country_refused_loudly_fail_closed() {
    let pool = pool().await;
    let svc = PartyWriteService::new(pool.clone());
    let err = svc
        .create_party(party_with_vat(&uq("XX"), "XX123456789"))
        .await
        .unwrap_err();
    match &err {
        PartyWriteError::VatUnknownCountry(country) => assert_eq!(country, "XX"),
        other => panic!("expected VatUnknownCountry, got {other:?}"),
    }
    let n: (i64,) = sqlx::query_as(
        "SELECT count(*) FROM party.parties WHERE party_code LIKE 'XX-%'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(n.0, 0, "a refused party must not be written");
    // Distinct machine code so the HTTP surface can tell escape-able refusals from malformed ones.
    assert_eq!(err.code(), "vat_unknown_country");
    assert_eq!(err.http_status(), 422);
}

// VAT-P4: the '/' no-VAT sentinel is accepted and stored verbatim (explicit "no VAT number").
#[tokio::test]
async fn no_vat_sentinel_accepted() {
    let pool = pool().await;
    let svc = PartyWriteService::new(pool.clone());
    let code = uq("SNT");
    let id = svc
        .create_party(party_with_vat(&code, NO_VAT_SENTINEL))
        .await
        .expect("the no-VAT sentinel must be accepted");
    assert_eq!(stored_vat(&pool, id).await.as_deref(), Some("/"));
}

// VAT-P5: the NAMED escape accepts unknown-country numbers; malformed known-country numbers
// stay refused even under the escape (the escape widens country coverage, never correctness).
#[tokio::test]
async fn named_escape_honored() {
    let pool = pool().await;
    let escaped = PartyWriteService::with_vat_policy(
        pool.clone(),
        VatValidationPolicy::ALLOW_UNKNOWN_COUNTRIES,
    );
    let id = escaped
        .create_party(party_with_vat(&uq("ESC"), "XX123456789"))
        .await
        .expect("the named escape must accept an unknown-country number");
    assert_eq!(stored_vat(&pool, id).await.as_deref(), Some("XX123456789"));
    assert!(matches!(
        escaped.create_party(party_with_vat(&uq("ESD"), "DE123")).await.unwrap_err(),
        PartyWriteError::InvalidVat(_)
    ));
}

// VAT-P6: malformed known-country numbers and prefix-less numbers are refused.
#[tokio::test]
async fn malformed_and_prefixless_refused() {
    let pool = pool().await;
    let svc = PartyWriteService::new(pool.clone());
    for bad in ["DE123", "123456789", "IEX", "BE04287594"] {
        let err = svc
            .create_party(party_with_vat(&uq("BAD"), bad))
            .await
            .unwrap_err();
        assert!(
            matches!(err, PartyWriteError::InvalidVat(_) | PartyWriteError::VatUnknownCountry(_)),
            "expected a VAT refusal for {bad}, got {err:?}"
        );
    }
}
