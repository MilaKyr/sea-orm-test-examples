use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "vaccination")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub dog_id: Uuid,
    // Should be another table, but leaving String for simplicity
    pub vaccine_sku: String,
    pub date: TimeDate,
    #[sea_orm(
        belongs_to,
        from = "dog_id",
        to = "id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    pub dog: HasOne<crate::dog::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
