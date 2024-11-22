use sea_orm_migration::prelude::*;

use crate::{m20241122_085421_create_users::Users, m20241122_125531_create_posts::Posts};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .col(ColumnDef::new(Comments::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Comments::Content).text().not_null())
                    .col(
                        ColumnDef::new(Comments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(Comments::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(ColumnDef::new(Comments::PostId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comments::Table, Comments::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Comments::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comments::Table, Comments::UserId)
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
                    .table(Comments::Table)
                    .col(Comments::CreatedAt)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_index(
                Index::create()
                    .table(Comments::Table)
                    .col(Comments::UpdatedAt)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_index(
                Index::create()
                    .table(Comments::Table)
                    .col(Comments::PostId)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_index(
                Index::create()
                    .table(Comments::Table)
                    .col(Comments::UserId)
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
pub enum Comments {
    Table,
    Id,
    Content,
    CreatedAt,
    UpdatedAt,
    PostId,
    UserId,
}
