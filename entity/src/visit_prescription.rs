use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "visit")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub visit_id: Uuid,
    pub prescription_id: Uuid,
    pub datetime: TimeDateTime,
    #[sea_orm(
        belongs_to,
        from = "visit_id",
        to = "id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    pub visit: HasOne<crate::visit::Entity>,
    #[sea_orm(
        belongs_to,
        from = "prescription_id",
        to = "id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    pub prescription: HasOne<crate::prescription::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
