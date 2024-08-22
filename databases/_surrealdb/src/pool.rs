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
    let surrealdb = Surreal::new::<Ws>(get_env_or("SURREALDB_LOCATION", "127.0.0.1:7002")).await?;

    info!("Signing in...");
    surrealdb
        .signin(Root {
            username: get_env_or("SURREALDB_NAME", "root").as_str(),
            password: get_env_or("SURREALDB_PASSWORD", "root").as_str(),
        })
        .await?;

    surrealdb
        .use_ns(get_env_or("SURREALDB_NAMESPACE", "basic_surrealdb"))
        .use_db(get_env_or("SURREALDB_DATABASE", "basic_surrealdb"))
        .await?;
    println!("[✅] Surrealdb connected successfully!");
    Ok(surrealdb)
}
