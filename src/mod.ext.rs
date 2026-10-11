// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

// The hand-owned request-pool shim (the composing service's tenant pool
// resolution): a generated-tree declaration the regenerator drops, so it
// lives in the preserved block (#447 cause-2 class).
pub mod request_pool;
pub use application::service::{
    validate_nik, validate_npwp, NewAddress, NewContact, NewEmail, NewParty, NewPhone,
    PartyWriteError, PartyWriteService,
};
pub use application::service::party_vat_validation::{
    normalize_vat, validate_vat, validate_vat_with, VatError, VatValidationPolicy,
    ALLOW_UNKNOWN_COUNTRIES_ENV, NO_VAT_SENTINEL,
};
pub use presentation::http::create_guarded_party_routes;
