//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use std::fmt::Debug;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Default,
)]
#[sea_orm(table_name = "t_kms_aksk")]
pub struct Model {
    #[sea_orm(column_name = "_id", primary_key)]
    #[serde(skip)]
    pub id: i64,
    #[sea_orm(unique)]
    pub kms_id: String,
    #[sea_orm(unique)]
    pub access_key: String,
    pub secret_key: Json,
    pub expired_at: Option<DateTimeWithTimeZone>,
}

impl Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model")
            .field("kms_id", &self.kms_id)
            .field("access_key", &self.access_key)
            .field("expired_at", &self.expired_at)
            .finish()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
