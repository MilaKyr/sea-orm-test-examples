use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "procedure")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub dog_id: Uuid,
    // Should be another table, but used String here for simplicity
    pub procedure_description: String,
    pub datetime: TimeDateTime,
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
