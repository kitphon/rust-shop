use sea_orm::entity::prelude::*;
use sea_orm::EnumIter;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "order_status")]
pub enum OrderStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "paid")]
    Paid,
    #[sea_orm(string_value = "in transit")]
    InTransit,
    #[sea_orm(string_value = "received")]
    Received,
}
