/*
This file will be used to interact with the database.sqlite file.
The database.sqlite file will be used to store the data about the downloads and the library of the user.
*/

use rusqlite::Error as RusqliteError;
use serde_json::{json, Value};

/// Function to create a database.sqlite file.
/// It will create 2 tables:
///    - Downloads:
///       - name TEXT (name of the torrent)
///       - game TEXT (name of the game)
///       - path TEXT (path to the game)
///       - link TEXT (magnet link of the torrent)
///       - uploader TEXT (name of the uploader, will be used to search the install instructions)
///
///     - Library:
///       - name TEXT (name of the game)
///       - path TEXT (path to the game)
///       - executable TEXT (path to the executable)
///

pub fn create_database_sqlite() {
    // Check if the database file already exists if yes return
    if std::path::Path::new("database.sqlite").exists() {
        return;
    }

    // Attempt to create the database file
    let conn = match rusqlite::Connection::open("database.sqlite") {
        Ok(conn) => conn,
        Err(_) => return,
    };

    // Attempt to create the downloads table
    if conn.execute(
        "CREATE TABLE IF NOT EXISTS downloads (
            name TEXT,
            game TEXT,
            link TEXT,
            uploader TEXT
        )",
        [],
    ).is_err() {
        return;
    }

    // Attempt to create the library table
    if conn.execute(
        "CREATE TABLE IF NOT EXISTS library (
            name TEXT,
            game TEXT,
            path TEXT,
            executable TEXT
        )",
        [],
    ).is_err() {
        return;
    }
}

/// Function to get the all the downloads from the database.
/// It will return a List of JSON objects containing the name, link and uploader of the torrent.
/// Returns a Result<Vec<Value>, RusqliteError> to handle potential SQL execution errors.
///
/// # Example
/// ```rust
/// let downloads = get_downloads().await.unwrap();
/// ```
///
/// # Returns
/// ```json
/// [
///     {
///         "name": "Zelda",
///         "link": "magnet:?xt=urn:btih:...",
///         "uploader": "Noidea"
///     },
///     {
///         "name": "Mario",
///         "link": "magnet:?xt=urn:btih:...",
///         "uploader": "Noidea"
///     }
/// ]
///
pub async fn get_downloads() -> Result<Vec<Value>, RusqliteError> {

    // Attempt to query the database for the downloads
    let conn = rusqlite::Connection::open("database.sqlite")?;
    let mut stmt = conn.prepare("SELECT name, game, link, uploader FROM downloads")?;
    let downloads = stmt
        .query_map([], |row| {
            Ok(json!({
                "name": row.get::<_, String>(0)?,
                "game": row.get::<_, String>(1)?,
                "link": row.get::<_, String>(2)?,
                "uploader": row.get::<_, String>(3)?
            }))
        })?
        .collect::<Result<Vec<Value>, RusqliteError>>()?;

    Ok(downloads)
}

/// Function to add a download to the database.
/// It will take the name, link and uploader of the torrent and add it to the downloads table.
/// It will return a Result<(), RusqliteError> to handle potential SQL execution errors.
///
/// # Arguments
/// - name: &str (name of the torrent)
/// - link: &str (magnet link of the torrent)
/// - uploader: &str (name of the uploader)
///
/// # Example
/// ```rust
/// add_download("Zelda", "magnet:?xt=urn:btih:...", "Noidea").await.unwrap();
/// ```
///
/// # Returns
/// ```bool
/// true // If the download was added successfully
/// false // If the download was not added successfully
/// ```
///
pub async fn add_download(
    name: &str,
    game: &str,
    link: &str,
    uploader: &str,
) -> Result<bool, RusqliteError> {

    // Attempt to open a connection to the database
    let conn = rusqlite::Connection::open("database.sqlite")?;

    // Attempt to insert the download into the downloads table
    conn.execute(
        "INSERT INTO downloads (name, game, link, uploader) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![name, game, link, uploader],
    )?;

    Ok(true)
}

/// Function to remove a download from the database.
/// It will take the name of the torrent and remove it from the downloads table.
/// It will return a Result<(), RusqliteError> to handle potential SQL execution errors.
///
/// # Arguments
/// - name: &str (name of the torrent)
///
/// # Example
/// ```rust
/// remove_download("Zelda").await.unwrap();
///
/// # Returns
/// ```bool
/// true // If the download was removed successfully
/// false // If the download was not removed successfully
/// ```
///

pub async fn remove_download(name: &str) -> Result<bool, RusqliteError> {

    // Attempt to open a connection to the database
    let conn = rusqlite::Connection::open("database.sqlite")?;

    // Attempt to remove the download from the downloads table
    conn.execute(
        "DELETE FROM downloads WHERE name = ?1",
        rusqlite::params![name],
    )?;

    Ok(true)
}

// TODO: Add library support
