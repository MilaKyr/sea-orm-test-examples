use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "prescription")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    // Should be another table, but used String here for simplicity
    pub medication: String,
    pub quantity: String,
    pub duration_in_days: u32,
    #[sea_orm(has_many)]
    pub visit_prescriptions: HasMany<crate::visit_prescription::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
