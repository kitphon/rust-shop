use sea_orm::entity::prelude::*;
use crate::order_status::OrderStatus;
use crate::order_items;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub customer_id: i64,
    pub status: Option<OrderStatus>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "order_items::Entity")]
    OrderItems,
}

impl Related<order_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
