use sqlx::MySqlPool;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPool::connect("mysql://root:root@localhost:3306/midgard").await?;
    Ok(pool)
}