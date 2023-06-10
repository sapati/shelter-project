use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Dog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Dog::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Dog::Name).string().not_null())
                    .col(ColumnDef::new(Dog::Description).text().not_null())
                    .col(ColumnDef::new(Dog::DateOfBirth).date().not_null())
                    .col(ColumnDef::new(Dog::DateOfVaccination).date())
                    .col(ColumnDef::new(Dog::ChipNumber).string().not_null())
                    .col(ColumnDef::new(Dog::Gender).string().not_null())
                    .col(ColumnDef::new(Dog::IsSterilized).boolean().not_null())
                    .col(ColumnDef::new(Dog::Breed).string().not_null())
                    .col(ColumnDef::new(Dog::Size).string().not_null())
                    .col(ColumnDef::new(Dog::Weight).integer())
                    .col(ColumnDef::new(Dog::Hair).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Dog::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Password,
}

#[derive(Iden)]
enum Dog {
    Table,
    Id,
    Name,
    Description,
    DateOfBirth,
    DateOfVaccination,
    ChipNumber,
    Gender,
    IsSterilized,
    Breed,
    Size,
    Weight,
    Hair,
}
