use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use dotenv::dotenv;
use std::env;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = MySqlPool::connect(&database_url).await?;
    println!("Connected to database {}", &database_url);
    Ok(pool)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}

pub fn update_config<P: AsRef<Path>>(
    path: P,
    new_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = read_config(&path)?;
    println!("Updating config with new url: {}", new_url);
    config.api_url = new_url.to_string();

    let config_str = serde_json::to_string(&config)?;
    let mut file = File::create(path)?;
    file.write_all(config_str.as_bytes())?;
    Ok(())
}
