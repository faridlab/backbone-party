//! Integration Tests for party
//!
//! User-owned module stub: declares the generated integration scaffold and carries the
//! crate-root lint expectation covering it (the scaffold itself stays generator-owned).
//!
//! Run with: cargo test --package backbone-party --test integration_tests

#![recursion_limit = "512"]
#![expect(clippy::expect_used, reason = "test harness: a panic here names the setup failure precisely")]

mod integration;

use integration::tests::*;

#[tokio::test]
async fn test_party_api() {
    let mut test = PartyApiTest::new();
    let results = test.run_all().await;

    let failed: Vec<_> = results.iter().filter(|r| !r.success).collect();
    if !failed.is_empty() {
        for f in &failed {
            eprintln!("FAILED: {} - {}", f.test_name, f.details);
        }
        panic!("{} tests failed", failed.len());
    }
}

#[tokio::test]
async fn test_party_address_api() {
    let mut test = PartyAddressApiTest::new();
    let results = test.run_all().await;

    let failed: Vec<_> = results.iter().filter(|r| !r.success).collect();
    if !failed.is_empty() {
        for f in &failed {
            eprintln!("FAILED: {} - {}", f.test_name, f.details);
        }
        panic!("{} tests failed", failed.len());
    }
}

#[tokio::test]
async fn test_party_contact_api() {
    let mut test = PartyContactApiTest::new();
    let results = test.run_all().await;

    let failed: Vec<_> = results.iter().filter(|r| !r.success).collect();
    if !failed.is_empty() {
        for f in &failed {
            eprintln!("FAILED: {} - {}", f.test_name, f.details);
        }
        panic!("{} tests failed", failed.len());
    }
}

#[tokio::test]
async fn test_party_email_api() {
    let mut test = PartyEmailApiTest::new();
    let results = test.run_all().await;

    let failed: Vec<_> = results.iter().filter(|r| !r.success).collect();
    if !failed.is_empty() {
        for f in &failed {
            eprintln!("FAILED: {} - {}", f.test_name, f.details);
        }
        panic!("{} tests failed", failed.len());
    }
}

#[tokio::test]
async fn test_party_phone_api() {
    let mut test = PartyPhoneApiTest::new();
    let results = test.run_all().await;

    let failed: Vec<_> = results.iter().filter(|r| !r.success).collect();
    if !failed.is_empty() {
        for f in &failed {
            eprintln!("FAILED: {} - {}", f.test_name, f.details);
        }
        panic!("{} tests failed", failed.len());
    }
}
