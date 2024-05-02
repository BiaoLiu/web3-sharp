//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "transaction_order")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub tx_id: i64,
    pub user_id: i64,
    pub ref_id: i64,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub amount: Decimal,
    pub tx_type: String,
    pub payment_type: Option<String>,
    pub status: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub payment_tx_hash: Option<String>,
    pub mint_tx_hash: Option<String>,
    pub transfer_tx_hash: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

pub type TransactionOrderModel = Model;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}