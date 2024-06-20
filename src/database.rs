use tokio_postgres::{NoTls, Error};
use crate::config::Config;
use std::sync::Arc;

async fn create_tables(config: Arc<Config>) -> Result<(), Error> {
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        config.db_host,
        config.db_port,
        config.db_user,
        config.db_password,
        config.db_name,
    );

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            user_data JSON NOT NULL,
            chat_data JSON NOT NULL,
            epoch_time INTEGER NOT NULL
        )
        ",
        &[],
    ).await?;

    client.execute(
        "
        CREATE TABLE IF NOT EXISTS requests (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES users(id),
            command TEXT NOT NULL,
            response TEXT NOT NULL,
            request_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        ",
        &[],
    ).await?;

    Ok(())
}

async fn create_user(client: &tokio_postgres::Client, user_data: &str, chat_data: &str, epoch_time: i32) -> Result<u64, Error> {
    let stmt = "
        INSERT INTO users (user_data, chat_data, epoch_time) 
        VALUES ($1, $2, $3)
    ";
    let rows_affected = client.execute(stmt, &[&user_data, &chat_data, &epoch_time]).await?;
    Ok(rows_affected)
}

async fn create_request(client: &tokio_postgres::Client, user_id: i32, command: &str, response: &str) -> Result<u64, Error> {
    let stmt = "
        INSERT INTO requests (user_id, command, response) 
        VALUES ($1, $2, $3)
    ";
    let rows_affected = client.execute(stmt, &[&user_id, &command, &response]).await?;
    Ok(rows_affected)
}

async fn find_user_by_id(client: &tokio_postgres::Client, user_id: &str) -> Result<Option<i32>, Error> {
    let stmt = "
        SELECT id FROM users WHERE user_data->>'id' = $1
    ";
    let row = client.query_opt(stmt, &[&user_id]).await?;
    if let Some(row) = row {
        Ok(Some(row.get(0)))
    } else {
        Ok(None)
    }
}