pub mod db_context;
pub mod repository;
pub mod todo_repository;

use crate::Error;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.

    let mut connection = db_context::get_pool().get().unwrap();
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
