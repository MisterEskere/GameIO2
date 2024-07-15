use serde_json::Value;
use rusqlite::Error as RusqliteError;

pub async fn get_request(url: &str) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;

    // Directly parse the response body as JSON
    let json = res.json::<Value>().await?;

    Ok(json)
}

pub async fn get_api_key() -> Result<String, RusqliteError> {
    
    // Attempt to create the database.sqlite file
    create_database_sqlite().await?;

    // Attempt to query the database for the API_KEY
    let conn = rusqlite::Connection::open("database.sqlite")?;
    let api_key: String = conn.query_row(
        "SELECT api_key FROM settings LIMIT 1",
        [],
        |row| row.get(0),
    )?;

    // Return the API_KEY
    Ok(api_key)
}

pub async fn set_api_key(api_key: &str) -> Result<(), RusqliteError> {

    // Attempt to create the database.sqlite file
    create_database_sqlite().await?;

    // Attempt to update the database with the new API_KEY
    let conn = rusqlite::Connection::open("database.sqlite")?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (api_key) VALUES (?)",
        [api_key],
    )?;

    Ok(())
}

/// Function to create a database.sqlite file. This will be used for persistent storage.
/// Table: Settings (1 row)
/// Columns: api_key TEXT
/// Returns a Result<(), IoError> to handle potential file creation or SQL execution errors.
async fn create_database_sqlite() -> Result<(), RusqliteError> {
    // Check if the database file already exists
    if !std::path::Path::new("database.sqlite").exists() {
        // Attempt to create the database file
        let conn = rusqlite::Connection::open("database.sqlite")?;

        // Attempt to create the settings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                api_key TEXT
            )",
            [],
        )?;
    }
    Ok(())
}