//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "invoices"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub order_uid: String,
    pub number: Option<String>,
    pub tax_id_number: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub address_line_3: Option<String>,
    pub address_line_4: Option<String>,
    pub address_present: Option<bool>,
    pub issued: Option<bool>,
    pub issue_date: Option<Date>,
    pub disposal_date: Option<Date>,
    pub payment_date: Option<Date>,
    pub total_value: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    OrderUid,
    Number,
    TaxIdNumber,
    AddressLine1,
    AddressLine2,
    AddressLine3,
    AddressLine4,
    AddressPresent,
    Issued,
    IssueDate,
    DisposalDate,
    PaymentDate,
    TotalValue,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::OrderUid => ColumnType::String(None).def(),
            Self::Number => ColumnType::String(None).def().null(),
            Self::TaxIdNumber => ColumnType::String(None).def().null(),
            Self::AddressLine1 => ColumnType::String(None).def().null(),
            Self::AddressLine2 => ColumnType::String(None).def().null(),
            Self::AddressLine3 => ColumnType::String(None).def().null(),
            Self::AddressLine4 => ColumnType::String(None).def().null(),
            Self::AddressPresent => ColumnType::Boolean.def().null(),
            Self::Issued => ColumnType::Boolean.def().null(),
            Self::IssueDate => ColumnType::Date.def().null(),
            Self::DisposalDate => ColumnType::Date.def().null(),
            Self::PaymentDate => ColumnType::Date.def().null(),
            Self::TotalValue => ColumnType::Decimal(Some((8u32, 2u32))).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
