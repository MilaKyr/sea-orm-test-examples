use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "owner")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    #[sea_orm(has_many)]
    pub dogs: HasMany<crate::dog::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
