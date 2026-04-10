use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "visit")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub dog_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub diagnosis: String,
    pub datetime: TimeDateTime,
    #[sea_orm(
        belongs_to,
        from = "dog_id",
        to = "id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    pub dog: HasOne<crate::dog::Entity>,
    #[sea_orm(has_many)]
    pub symptoms: HasMany<crate::symptom::Entity>,
    #[sea_orm(has_many)]
    pub visit_prescriptions: HasMany<crate::visit_prescription::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
