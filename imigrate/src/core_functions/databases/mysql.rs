// postgres.rs
use std::collections::HashMap;

pub async fn mysql_db() -> Result<(), String> {
        connect_db().await;
        let _structure = get_db_struct().await?;
        migrate_to_db().await;
         println!("Successfully migrated data");
    Ok(())
}

async fn connect_db() {
    println!("Connecting to Mysql...");
}

async fn get_db_struct() -> Result<HashMap<String, Vec<String>>, String> {
    Ok(HashMap::new())
}

async fn migrate_to_db() {
    println!("Migrating to Mysql...");
}
