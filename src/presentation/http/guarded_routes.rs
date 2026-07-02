//! Guarded route composition — the RECOMMENDED way to mount the party module.
//!
//! Hand-authored (user-owned; see `metaphor.codegen.yaml`). Closes the CRUD-bypass: the generated
//! 12-endpoint CRUD writes rows with no domain validation. Here every entity is READ + **validated
//! create** via `PartyWriteService` (NPWP/NIK format + uniqueness on the party; party existence on
//! each child). Generic update/delete/upsert/bulk are not mounted.

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::service::party_write_service::{
    NewAddress, NewContact, NewEmail, NewParty, NewPhone, PartyWriteError, PartyWriteService,
};
use crate::PartyModule;

use super::{
    create_party_address_read_routes, create_party_contact_read_routes,
    create_party_email_read_routes, create_party_phone_read_routes, create_party_read_routes,
};

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}
#[derive(Debug, Serialize)]
struct IdResponse {
    id: Uuid,
}
fn err_response(e: PartyWriteError) -> axum::response::Response {
    let status = StatusCode::from_u16(e.http_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(ErrorBody { error: e.code(), message: e.to_string() })).into_response()
}

// ── Party ───────────────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatePartyBody {
    party_code: String,
    #[serde(default)]
    party_kind: Option<String>,
    name: String,
    #[serde(default)]
    legal_name: Option<String>,
    #[serde(default)]
    first_name: Option<String>,
    #[serde(default)]
    last_name: Option<String>,
    #[serde(default)]
    npwp: Option<String>,
    #[serde(default)]
    nik: Option<String>,
}

async fn create_party(
    State(svc): State<Arc<PartyWriteService>>,
    Json(b): Json<CreatePartyBody>,
) -> axum::response::Response {
    match svc
        .create_party(NewParty {
            party_code: b.party_code,
            party_kind: b.party_kind,
            name: b.name,
            legal_name: b.legal_name,
            first_name: b.first_name,
            last_name: b.last_name,
            npwp: b.npwp,
            nik: b.nik,
        })
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(IdResponse { id })).into_response(),
        Err(e) => err_response(e),
    }
}

// ── Address ─────────────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddAddressBody {
    party_id: Uuid,
    #[serde(default)]
    address_type: Option<String>,
    #[serde(default)]
    label: Option<String>,
    line1: String,
    #[serde(default)]
    line2: Option<String>,
    #[serde(default)]
    country_id: Option<Uuid>,
    #[serde(default)]
    province_id: Option<Uuid>,
    #[serde(default)]
    city_id: Option<Uuid>,
    #[serde(default)]
    district_id: Option<Uuid>,
    #[serde(default)]
    subdistrict_id: Option<Uuid>,
    #[serde(default)]
    postal_code: Option<String>,
    #[serde(default)]
    latitude: Option<Decimal>,
    #[serde(default)]
    longitude: Option<Decimal>,
    #[serde(default)]
    is_primary: bool,
    #[serde(default)]
    is_billing: bool,
    #[serde(default)]
    is_shipping: bool,
}

async fn add_address(
    State(svc): State<Arc<PartyWriteService>>,
    Json(b): Json<AddAddressBody>,
) -> axum::response::Response {
    match svc
        .add_address(NewAddress {
            party_id: b.party_id,
            address_type: b.address_type,
            label: b.label,
            line1: b.line1,
            line2: b.line2,
            country_id: b.country_id,
            province_id: b.province_id,
            city_id: b.city_id,
            district_id: b.district_id,
            subdistrict_id: b.subdistrict_id,
            postal_code: b.postal_code,
            latitude: b.latitude,
            longitude: b.longitude,
            is_primary: b.is_primary,
            is_billing: b.is_billing,
            is_shipping: b.is_shipping,
        })
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(IdResponse { id })).into_response(),
        Err(e) => err_response(e),
    }
}

// ── Contact / Email / Phone ───────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddContactBody {
    party_id: Uuid,
    name: String,
    #[serde(default)]
    job_title: Option<String>,
    #[serde(default)]
    department: Option<String>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    phone: Option<String>,
    #[serde(default)]
    is_primary: bool,
}
async fn add_contact(
    State(svc): State<Arc<PartyWriteService>>,
    Json(b): Json<AddContactBody>,
) -> axum::response::Response {
    match svc
        .add_contact(NewContact {
            party_id: b.party_id,
            name: b.name,
            job_title: b.job_title,
            department: b.department,
            email: b.email,
            phone: b.phone,
            is_primary: b.is_primary,
        })
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(IdResponse { id })).into_response(),
        Err(e) => err_response(e),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddEmailBody {
    party_id: Uuid,
    #[serde(default)]
    label: Option<String>,
    email: String,
    #[serde(default)]
    is_primary: bool,
}
async fn add_email(
    State(svc): State<Arc<PartyWriteService>>,
    Json(b): Json<AddEmailBody>,
) -> axum::response::Response {
    match svc
        .add_email(NewEmail { party_id: b.party_id, label: b.label, email: b.email, is_primary: b.is_primary })
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(IdResponse { id })).into_response(),
        Err(e) => err_response(e),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddPhoneBody {
    party_id: Uuid,
    #[serde(default)]
    label: Option<String>,
    phone: String,
    #[serde(default)]
    is_primary: bool,
}
async fn add_phone(
    State(svc): State<Arc<PartyWriteService>>,
    Json(b): Json<AddPhoneBody>,
) -> axum::response::Response {
    match svc
        .add_phone(NewPhone { party_id: b.party_id, label: b.label, phone: b.phone, is_primary: b.is_primary })
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(IdResponse { id })).into_response(),
        Err(e) => err_response(e),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SetPrimaryBody {
    party_id: Uuid,
    kind: String, // address | contact | email | phone
    child_id: Uuid,
}
async fn set_primary(
    State(svc): State<Arc<PartyWriteService>>,
    Json(b): Json<SetPrimaryBody>,
) -> axum::response::Response {
    match svc.set_primary(b.party_id, &b.kind, b.child_id).await {
        Ok(()) => (StatusCode::OK, Json(IdResponse { id: b.child_id })).into_response(),
        Err(e) => err_response(e),
    }
}

fn create_party_write_routes(svc: Arc<PartyWriteService>) -> Router {
    Router::new()
        .route("/parties", post(create_party))
        .route("/party-addresses", post(add_address))
        .route("/party-contacts", post(add_contact))
        .route("/party-emails", post(add_email))
        .route("/party-phones", post(add_phone))
        .route("/party-set-primary", post(set_primary))
        .with_state(svc)
}

/// Mount the party module with write paths locked to validated services.
/// **Prefer this over `PartyModule::all_crud_routes()` for any real deployment.**
pub fn create_guarded_party_routes(m: &PartyModule) -> Router {
    Router::new()
        .merge(create_party_read_routes(m.party_service.clone()))
        .merge(create_party_address_read_routes(m.party_address_service.clone()))
        .merge(create_party_contact_read_routes(m.party_contact_service.clone()))
        .merge(create_party_email_read_routes(m.party_email_service.clone()))
        .merge(create_party_phone_read_routes(m.party_phone_service.clone()))
        .merge(create_party_write_routes(m.party_write_service.clone()))
}
