pub use sea_orm_migration::prelude::*;

mod m20240415_000001_create_post_table;
mod m20240416_000001_create_user_table;
mod profile;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator
{
	fn migrations() -> Vec<Box<dyn MigrationTrait>>
	{
		vec![
		     Box::new(m20240415_000001_create_post_table::Migration),
		     Box::new(m20240416_000001_create_user_table::Migration),
		     Box::new(profile::m20240423_000001_create_anniversary_table::Migration),
		     Box::new(profile::m20240424_000001_create_location_table::Migration),
		     Box::new(profile::m20240425_000001_create_pronoun_table::Migration),
		     Box::new(profile::m20240426_000001_create_role_table::Migration),
		     Box::new(profile::m20240427_000001_create_orientation_table::Migration),
		     Box::new(profile::m20240428_000001_create_gender_table::Migration),
		     Box::new(profile::m20240429_000001_create_biography_table::Migration),
		     Box::new(profile::m20240430_000001_create_relationship_table::Migration),
		]
	}
}
