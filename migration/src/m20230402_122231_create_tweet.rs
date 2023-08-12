use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tweet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tweet::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tweet::UserId).integer().not_null())
                    .col(ColumnDef::new(Tweet::Content).text().not_null())
                    .col(ColumnDef::new(Tweet::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Tweet::UpdatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_tweets_user_id_users_id")
                            .from(Tweet::Table, Tweet::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tweet::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Tweet {
    Table,
    Id,
    UserId,
    Content,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
