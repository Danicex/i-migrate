pub mod database_migrate;
pub mod migrate_status;
pub mod databases;
pub mod sign_user;
pub mod help;
pub mod init_imigrate;

#[allow(dead_code)]
pub struct Command{
    pub prompt: &'static str,
    pub action: fn(),
}

//pub fn get_commands()->  Vec<Command>{}