use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use log::info;

use common::functions::get_env_or;

pub fn postgresql_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    info!("Connecting postgresql...");

    let manager = ConnectionManager::<PgConnection>::new(get_env_or(
        "DATABASE_URL",
        "postgres://postgres:postgres@localhost:7004/postgres",
    ));

    let pool = Pool::builder().build(manager).expect("Failed to create pool.");

    println!("[✅] Postgresql connected successfully!");

    pool
}
