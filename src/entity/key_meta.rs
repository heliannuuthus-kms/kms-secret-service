//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::encrypto::types::{KeyOrigin, KeySpec, KeyState, KeyUsage};

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    Default,
)]
#[sea_orm(table_name = "t_key_meta")]
pub struct Model {
    #[sea_orm(column_name = "_id", primary_key)]
    #[serde(skip)]
    pub id: i64,
    #[sea_orm(unique)]
    pub key_id: String,
    pub spec: KeySpec,
    pub origin: KeyOrigin,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub version: String,
    pub primary_version: String,
    pub state: KeyState,
    pub usage: KeyUsage,
    pub rotation_interval: i64,
    pub creator: String,
    pub material_expire_at: Option<DateTimeWithTimeZone>,
    pub last_rotation_at: Option<DateTimeWithTimeZone>,
    pub deletion_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
