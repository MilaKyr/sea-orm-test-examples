use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "medication")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    // Some fields would be there, omitting for simplicity
    #[sea_orm(has_many)]
    pub visit_prescriptions: HasMany<crate::visit_prescription::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
