// modules that holds all database functions
use super::databases::{postgres, mysql, sqlite, mongodb, cockroach, mariadb };
//modules to destructure and read config file
use serde::Deserialize;
use std::fs;
use std::error::Error;

//define enums for database type
#[derive(Debug)]
enum DbType {
    PostgreSQL,
    MySQL,
    Sqlite,
    Mongodb,
    CockroachDb,
    Mariadb,
}
//attach the imported functions to the DbType enum
impl DbType {
    fn from_str(s: &str) -> Option<DbType> {
        let lower = s.to_lowercase();
        println!("Parsing db_type: {}", lower);
        match lower.as_str() {
            "postgresql" | "postgres" | "pg" => Some(DbType::PostgreSQL),
            "mysql" => Some(DbType::MySQL),
            "sqlite" => Some(DbType::Sqlite),
            "mongodb" => Some(DbType::Mongodb),
            "cockroachdb" => Some(DbType::CockroachDb),
            "mariadb" => Some(DbType::Mariadb),
            _ => None,
        }
    }
}


//store config value in struct
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DatabaseConfig {
    pub db_type: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub database_url: String,
    pub ssl_enabled: bool,
}

// Assigning database type to the config type
pub async fn assign_db(path: &str) -> Result<(), Box<dyn Error>> {
    // Read db_config file
    let data = fs::read_to_string(path)?;
    let config: DatabaseConfig = serde_json::from_str(&data)?;

    // Convert config string to enum and call the appropriate function
    match DbType::from_str(&config.db_type) {
        Some(DbType::PostgreSQL) => {
            postgres::postgres_db().await?;
            Ok(())
        }
        Some(DbType::MySQL) => {
            mysql::mysql_db().await?;
            Ok(())
        }
        Some(DbType::Sqlite) => {
            sqlite::sqlite_db().await?;
            Ok(())
        }
        Some(DbType::Mongodb) => {
            mongodb::mongo_db().await?;
            Ok(())
        }
        Some(DbType::CockroachDb) => {
            cockroach::cockroach_db().await?;
            Ok(())
        }
        Some(DbType::Mariadb) => {
            mariadb::maria_db().await?;
            Ok(())
        }
        None => {
            Err(format!("Unsupported or invalid DB type: {}", config.db_type).into())
        }
    }
}