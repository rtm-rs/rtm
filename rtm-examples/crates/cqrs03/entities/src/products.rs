//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "products"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub name: Option<String>,
    pub price: Option<Decimal>,
    pub id: Uuid,
    pub stock_level: Option<i32>,
    pub registered_at: Option<DateTimeUtc>,
    pub vat_rate_code: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Name,
    Price,
    Id,
    StockLevel,
    RegisteredAt,
    VatRateCode,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Name => ColumnType::String(None).def().null(),
            Self::Price => ColumnType::Decimal(Some((8u32, 2u32))).def().null(),
            Self::Id => ColumnType::Uuid.def(),
            Self::StockLevel => ColumnType::Integer.def().null(),
            Self::RegisteredAt => ColumnType::Timestamp.def().null(),
            Self::VatRateCode => ColumnType::String(None).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
