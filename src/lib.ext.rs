//! The module's extension (hand-written; ADR-0031): what the module adds beside its
//! generated services. The generated module and builder carry `ModuleExt` and
//! `ModuleBuilderExt` and dereference to them, so their fields read as `module.field`.

#[allow(unused_imports)]
use super::*;

/// State the module adds; read through the generated module's `Deref`.
pub struct ModuleExt {
    /// Validated party + child writes (NPWP/NIK format+uniqueness, party existence).
    pub party_write_service: Arc<PartyWriteService>,
}

/// State the builder adds.
pub struct ModuleBuilderExt {
}

impl Default for ModuleBuilderExt {
    fn default() -> Self {
        Self {
        }
    }
}

impl ModuleBuilderExt {
    /// Build the extension's state from what the generated build made.
    #[allow(unused_variables, clippy::redundant_clone)]
    pub(crate) fn build(self, parts: &ModuleParts<'_>) -> anyhow::Result<ModuleExt> {
        let db_pool = parts.db_pool.clone();
        // VAT posture from the named configuration escape (fail-closed unless
        // PARTY_VAT_ALLOW_UNKNOWN_COUNTRIES is explicitly armed).
        let party_write_service = Arc::new(PartyWriteService::with_vat_policy(
            db_pool.clone(),
            VatValidationPolicy::from_env(),
        ));
        Ok(ModuleExt {
            party_write_service,
        })
    }
}
