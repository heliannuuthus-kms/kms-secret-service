use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};
use utoipa::{IntoParams, ToSchema};

use crate::common::encrypto::types::{
    KeyOrigin, KeySpec, KeyUsage, WrappingKeyAlgorithm, WrappingKeySpec,
};
#[serde_as]
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct KeyCreateForm {
    pub kms_id: String,
    pub description: Option<String>,
    #[serde(rename = "key_usage")]
    pub usage: KeyUsage,
    pub origin: KeyOrigin,
    #[serde(rename = "key_spec")]
    pub spec: KeySpec,
    pub enable_automatic_rotation: bool,
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    pub rotation_interval: Option<Duration>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, IntoParams)]
pub struct KeyImportParamsQuery {
    pub key_id: String,
    pub wrapping_algorithm: WrappingKeyAlgorithm,
    pub wrapping_key_spec: WrappingKeySpec,
}

#[serde_as]
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct KeyImportForm {
    pub key_id: String,
    pub encrypted_key_material: String,
    pub import_token: String,
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    pub key_material_expire_in: Option<Duration>,
}
