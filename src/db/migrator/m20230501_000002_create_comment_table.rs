use sea_orm_migration::prelude::*;

use super::m20230501_000001_create_post_table::Post;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comment::Text).string().not_null())
                    .col(ColumnDef::new(Comment::PostId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk-comment-post_id")
                            .from(Comment::Table, Comment::PostId)
                            .to(Post::Table, Post::Id)
                    )
                    .col(
                        ColumnDef::new(Comment::CreatedAt)
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Comment::UpdatedAt)
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Comment {
    Table,
    Id,
    Text,
    PostId,
    CreatedAt,
    UpdatedAt,
}
