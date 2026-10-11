// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

// The hand-written party SQL's parameter/projection types. Every repository listed here is declared
// `user_owned` in metaphor.codegen.yaml — see party_write_service, which orchestrates them.
pub use party_repository::NewPartyRow;
pub use party_address_repository::NewPartyAddressRow;
pub use party_contact_repository::NewPartyContactRow;
pub use party_email_repository::NewPartyEmailRow;
pub use party_phone_repository::NewPartyPhoneRow;
