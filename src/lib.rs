use sea_orm::ColumnTrait;
use sea_orm::{PaginatorTrait, QueryFilter};
use entity::{dog, owner};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, DerivePartialModel, EntityTrait, QuerySelect};
use uuid::Uuid;

#[derive(Debug, DerivePartialModel, PartialEq)]
#[sea_orm(entity = "dog::Entity")]
pub struct DogIdentification {
    pub name: String,
    pub chip_id: String,
}

#[derive(Debug, DerivePartialModel, PartialEq)]
#[sea_orm(entity = "owner::Entity")]
pub struct OwnerPartialModel {
    pub name: String,
    pub surname: String,
}

#[derive(Debug, DerivePartialModel, PartialEq)]
#[sea_orm(entity = "dog::Entity")]
pub struct DogWithOwner {
    pub name: String,
    pub chip_id: String,
    #[sea_orm(nested)]
    pub owner: OwnerPartialModel,
}

pub async fn update_dog_chip(
    id: Uuid,
    chip_id: String,
    conn: &DatabaseConnection,
) -> Result<(), DbErr> {
    if let Some(dog) = dog::Entity::find_by_id(id).one(conn).await? {
        let mut active_model: dog::ActiveModel = dog.into();
        active_model.chip_id = ActiveValue::Set(chip_id);
        active_model.save(conn).await?;
        // Save here will default to update as the ID is already set
        // but the mocking logic is same
        // active.update(conn).await?;
    }
    Ok(())
}

pub async fn get_dog(id: Uuid, conn: &DatabaseConnection) -> Option<DogIdentification> {
    dog::Entity::find_by_id(id)
        .into_partial_model()
        .one(conn)
        .await
        .ok()
        .flatten()
}

pub async fn get_dog_with_owner(id: Uuid, conn: &DatabaseConnection) -> Option<DogWithOwner> {
    dog::Entity::find_by_id(id)
        .left_join(owner::Entity)
        .into_partial_model()
        .one(conn)
        .await
        .ok()
        .flatten()
}

pub async fn get_owner_name(dog_id: Uuid, conn: &DatabaseConnection) -> Option<String> {
    dog::Entity::find_by_id(dog_id)
        .left_join(owner::Entity)
        .select_only()
        .column(owner::Column::Name)
        .into_tuple::<String>()
        .one(conn)
        .await
        .ok()
        .flatten()
}

pub async fn get_number_of_dogs(user_id: Uuid, conn: &DatabaseConnection) -> Option<u64> {
    dog::Entity::find()
        .filter(dog::Column::OwnerId.eq(user_id))
        .select_only()
        .count(conn)
        .await
        .ok()
}

pub async fn delete_dog(dog_id: Uuid, conn: &DatabaseConnection) -> Result<(), DbErr> {
    if let Some(dog) = dog::Entity::find_by_id(dog_id).one(conn).await? {
        dog.cascade_delete(conn).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use entity::{procedure, symptom, vaccination, visit, visit_prescription};
    use sea_orm::{DatabaseBackend, IntoMockRow, MockDatabase, MockExecResult, Value};
    use std::collections::BTreeMap;
    use time::macros::{date, datetime};

    #[tokio::test]
    async fn test_update_dog_chip() -> Result<(), DbErr> {
        let conn = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results([
                [dog::Model {
                    chip_id: "chip_1".to_string(),
                    ..Default::default()
                }],
                [dog::Model {
                    chip_id: "chip_2".to_string(),
                    ..Default::default()
                }],
            ])
            .into_connection();
        update_dog_chip(Uuid::default(), "chip_2".to_string(), &conn).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_dog() -> Result<(), DbErr> {
        let dog_name = "Buddy".to_string();
        let chip_id = "chip_1".to_string();
        let conn = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results([[BTreeMap::from([
                ("name", Value::String(Some(dog_name.clone()))),
                ("chip_id", Value::String(Some(chip_id.clone()))),
            ])]])
            .into_connection();
        let dog_opt = get_dog(Uuid::default(), &conn).await;
        assert!(dog_opt.is_some());
        let dog = dog_opt.unwrap();
        let expected = DogIdentification {
            name: dog_name,
            chip_id,
        };
        assert_eq!(dog, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_dog_with_owner() -> Result<(), DbErr> {
        let dog_name = "Buddy".to_string();
        let chip_id = "chip_1".to_string();
        let owner_name = "Jane".to_string();
        let owner_surname = "Doe".to_string();

        let conn = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results([[BTreeMap::from([
                ("name", Value::String(Some(dog_name.clone()))),
                ("chip_id", Value::String(Some(chip_id.clone()))),
                ("owner_name", Value::String(Some(owner_name.clone()))),
                ("owner_surname", Value::String(Some(owner_surname.clone()))),
            ])]])
            .into_connection();

        let dog_opt = get_dog_with_owner(Uuid::default(), &conn).await;
        assert!(dog_opt.is_some());
        let dog = dog_opt.unwrap();
        let expected = DogWithOwner {
            name: dog_name,
            chip_id,
            owner: OwnerPartialModel {
                name: owner_name,
                surname: owner_surname,
            },
        };
        assert_eq!(dog, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_owner_name() -> Result<(), DbErr> {
        let original_owner_name = "Jane".to_string();
        let conn = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results([[BTreeMap::from([(
                "owner_name",
                Value::String(Some(original_owner_name.clone())),
            )])]])
            .into_connection();
        let owner_name_opt = get_owner_name(Uuid::default(), &conn).await;
        assert!(owner_name_opt.is_some());
        let owner_name = owner_name_opt.unwrap();
        assert_eq!(owner_name, original_owner_name);
        Ok(())
    }

    #[tokio::test]
    async fn test_cascade_delete_dog() -> Result<(), DbErr> {
        let id = Uuid::max();
        let time = datetime!(2026-04-08 00:00:01);
        let conn = MockDatabase::new(DatabaseBackend::Sqlite)
            // First, we find the model to delete
            .append_query_results([[dog::Model {
                id,
                name: "Jack".to_string(),
                owner_id: Default::default(),
                chip_id: "chip_1".to_string(),
            }]])
            // First one-to-many relationship in the `dog` table is procedure, we must get it and then delete it
            .append_query_results([[procedure::Model {
                id,
                dog_id: Uuid::default(),
                procedure_description: "test procedure".to_string(),
                datetime: time.clone(),
            }]])
            .append_exec_results([MockExecResult::default()])
            // Next relationship in the `dog` table is visits, which in turn has few one-to-many relationships
            .append_query_results([
                // select first the visit model
                [visit::Model {
                    id,
                    dog_id: Uuid::default(),
                    diagnosis: "test diagnosis".to_string(),
                    datetime: time.clone(),
                }
                .into_mock_row()],
                // then first one-to-many relationship in the `visit` table
                [symptom::Model {
                    id,
                    visit_id: Uuid::new_v4(),
                    symptom: "Test symptom".to_string(),
                }
                .into_mock_row()],
                // then next one-to-many relationship in the `visit` table
                [visit_prescription::Model {
                    id,
                    visit_id: Uuid::new_v4(),
                    prescription_id: Default::default(),
                    datetime: time.clone(),
                }
                .into_mock_row()],
            ])
            // remove all three tables
            .append_exec_results([MockExecResult::default()])
            .append_exec_results([MockExecResult::default()])
            .append_exec_results([MockExecResult::default()])
            // The last relationship in the `dog` table is vaccinations, get the model and delete it
            .append_query_results([[vaccination::Model {
                id,
                dog_id: Uuid::default(),
                vaccine_sku: "vaccine_000".to_string(),
                date: date!(2026 - 04 - 08).into(),
            }]])
            .append_exec_results([MockExecResult::default()])
            // Finally, mock deleting the `dog` model
            .append_exec_results([MockExecResult::default()])
            .into_connection();
        let result = delete_dog(Uuid::default(), &conn).await;
        assert!(result.is_ok());
        Ok(())
    }
}
