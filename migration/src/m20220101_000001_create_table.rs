use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alert::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alert::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alert::Ticker).string().not_null())
                    .col(ColumnDef::new(Alert::Signal).string().not_null())
                    .col(ColumnDef::new(Alert::Category).string().not_null())
                    .col(ColumnDef::new(Alert::Timestamp).timestamp().not_null())
                    .col(ColumnDef::new(Alert::Interval).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alert::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Alert {
    Table,
    Id,
    Ticker,
    Timestamp,
    Signal,
    Category,
    Interval,
}
