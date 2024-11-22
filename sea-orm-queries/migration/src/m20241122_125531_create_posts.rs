use sea_orm_migration::prelude::*;

use crate::m20241122_085421_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .col(
                        ColumnDef::new(Posts::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(SimpleExpr::Custom("uuid_generate_v4()".to_string())),
                    )
                    .col(ColumnDef::new(Posts::Title).text().not_null())
                    .col(ColumnDef::new(Posts::Content).text().not_null())
                    .col(
                        ColumnDef::new(Posts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(Posts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(ColumnDef::new(Posts::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Posts::Table, Posts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_index(
                Index::create()
                    .table(Posts::Table)
                    .col(Posts::CreatedAt)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_index(
                Index::create()
                    .table(Posts::Table)
                    .col(Posts::UpdatedAt)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_index(
                Index::create()
                    .table(Posts::Table)
                    .col(Posts::UserId)
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Posts {
    Table,
    Id,
    Title,
    Content,
    CreatedAt,
    UpdatedAt,
    UserId,
}
