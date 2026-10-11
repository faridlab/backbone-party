// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

pub mod party_vat_validation;
pub mod party_write_service;
pub use party_vat_validation::{
    normalize_vat, validate_vat, validate_vat_with, VatError, VatValidationPolicy,
    ALLOW_UNKNOWN_COUNTRIES_ENV, NO_VAT_SENTINEL,
};
pub use party_write_service::{
    validate_nik, validate_npwp, NewAddress, NewContact, NewEmail, NewParty, NewPhone,
    PartyWriteError, PartyWriteService,
};
