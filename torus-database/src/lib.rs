#[macro_use]
extern crate diesel;
extern crate core;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;
use std::process;

pub mod models;
pub mod sandbox_repository;
pub mod schema;

#[derive(Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    name: String,
}

pub fn establish_connection() -> PgConnection {
    let config = match envy::prefixed("DB_").from_env::<DatabaseConfig>() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.name
    );
    PgConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}
