//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "transaction_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub tx_hash: String,
    pub operator: String,
    pub from: String,
    pub to: String,
    pub token_id: String,
    pub value: String,
    pub status: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

pub type TransactionEventModel = Model;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
