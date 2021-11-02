use std::env;
use std::io;
use std::io::ErrorKind;

use sea_orm::{Database, DatabaseConnection};

const DEFAULT_DB_URL_PREFIX: &str = "sqlite:";
const DEFAULT_DB_FILE: &str = "m.db";

pub fn get_db_file() -> String {
    env::var("M_DB_PATH").unwrap_or(DEFAULT_DB_FILE.to_owned())
}

fn get_db_url() -> String {
    let file_path = env::var("M_DB_PATH").unwrap_or(DEFAULT_DB_FILE.to_owned());
    let mut url = DEFAULT_DB_URL_PREFIX.to_owned();
    url.push_str(&file_path);
    eprintln!("DB_URL = {}", url);
    url
}

pub async fn connect() -> io::Result<DatabaseConnection> {
    let db_conn = Database::connect(get_db_url()).await;
    db_conn.map_err(|db_err| io::Error::new(ErrorKind::Other, db_err))
}
