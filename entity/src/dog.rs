use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dog")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub date_of_birth: NaiveDate,
    pub date_of_vaccination: Option<NaiveDate>,
    pub chip_number: String,
    pub gender: String,
    pub is_sterilized: bool,
    pub breed: String,
    pub size: String,
    pub weight: Option<i32>,
    pub hair: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
