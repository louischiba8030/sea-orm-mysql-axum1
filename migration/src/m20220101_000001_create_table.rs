use entity::post::*;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
			manager
				.create_table(
					Table::create()
						.table(Entity)
						.if_not_exists()
						.col(ColumnDef::new(Column::Id).integer().not_null().auto_increment().primary_key())
						.col(ColumnDef::new(Column::KanjiName).string().not_null())
						.col(ColumnDef::new(Column::EngName).string().not_null())
						.col(ColumnDef::new(Column::Age).integer().not_null())
						.col(ColumnDef::new(Column::Bloodtype).string().not_null())
						.col(ColumnDef::new(Column::Birthplace).string().not_null())
						.to_owned()
				).await
		}

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
			manager
				.drop_table(Table::drop().table(Entity).to_owned())
				.await
		}
}
