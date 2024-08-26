use dotenv::dotenv;
use log::info;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use common::functions::get_env_or;

pub async fn surrealdb_pool() -> surrealdb::Result<Surreal<Client>> {
    dotenv().ok();

    info!("Connecting surrealdb...");
    let surrealdb = Surreal::new::<Ws>(get_env_or("SURREALDB_ADDRESS", "127.0.0.1:7002")).await?;

    info!("Signing in...");
    surrealdb
        .signin(Root {
            username: get_env_or("SURREALDB_USERNAME", "surrealdb_username").as_str(),
            password: get_env_or("SURREALDB_PASSWORD", "surrealdb_password").as_str(),
        })
        .await?;

    surrealdb
        .use_ns(get_env_or("SURREALDB_NAMESPACE", "surrealdb_namespace"))
        .use_db(get_env_or("SURREALDB_DATABASE", "surrealdb_database"))
        .await?;
    println!("[✅] Surrealdb connected successfully!");
    Ok(surrealdb)
}
