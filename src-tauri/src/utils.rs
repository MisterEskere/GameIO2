use serde_json::Value;

pub async fn get_request(url: &str) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;

    // Directly parse the response body as JSON
    let json = res.json::<Value>().await?;

    Ok(json)
}

pub async fn get_api_key() -> String {
    
    // Create the database.sqlite file
    create_database_sqlite().await;

    // query the database for the API_KEY
    let conn = rusqlite::Connection::open("database.sqlite").unwrap();
    let api_key: String = conn.query_row(
        "SELECT api_key FROM settings LIMIT 1",
        [],
        |row| row.get(0),
    ).unwrap();

    // Return the API_KEY
    api_key
}

pub async fn set_api_key(api_key: &str) {

    // Create the database.sqlite file
    create_database_sqlite().await;

    // Update the database with the new API_KEY
    let conn = rusqlite::Connection::open("database.sqlite").unwrap();
    conn.execute(
        "INSERT OR REPLACE INTO settings (api_key) VALUES (?)",
        [api_key],
    ).unwrap();

}



/// Function to create a database.sqlite file. This will be used for persistent storage.
/// Table: Settings (1 row)
/// Columns: api_key TEXT
async fn create_database_sqlite() {
    // Check if the database file already exists
    if !std::path::Path::new("database.sqlite").exists() {
        // Create the database file
        let conn = rusqlite::Connection::open("database.sqlite").unwrap();

        // Create the settings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                api_key TEXT
            )",
            [],
        )
        .unwrap();
    }
}