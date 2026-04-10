use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "dog")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub chip_id: String,
    #[sea_orm(
        belongs_to,
        from = "owner_id",
        to = "id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    pub owner: HasOne<crate::owner::Entity>,
    #[sea_orm(has_many)]
    pub procedures: HasMany<crate::procedure::Entity>,
    #[sea_orm(has_many)]
    pub visits: HasMany<crate::visit::Entity>,
    #[sea_orm(has_many)]
    pub vaccinations: HasMany<crate::vaccination::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
