// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod env;
mod database;
mod api;
mod scrapers;
mod torrent;

use std::thread;

/********************************************************************************************************************/
/// This function will be used to start all the torrents previously started. 
/// This will be called at the beginning of the application.
async fn start_torrents() {
    let torrents = database::get_downloads().await.unwrap();

    for torrent in torrents {
        let download_path = torrent["path"].as_str().unwrap().to_string(); // Convert to owned String
        let magnet_link: String = torrent["link"].as_str().unwrap().to_string(); // Convert to owned String

        thread::spawn(move || {
            torrent::download_torrent(&download_path, &magnet_link); // Directly use the owned Strings
        });
    }
}
/********************************************************************************************************************/

/********************************************************************************************************************/
/// Makes a GET request to "https://rawg.io/api/games?page=1&page_size=10&search=NAME_OF_GAME&parent_platforms=1,6,5&stores=1,5,11"
#[tauri::command]
async fn games_list(game_name: &str) -> Result<Vec<serde_json::Value>, String> {
    
    // Retrieve the API_KEY from the .env file
    let api_key = match env::get_id_client().await {
        Ok(key) => key,
        Err(e) => return Err(format!("Failed to get API key: {}", e)),
    };

    // use the api::games_list function to get the list of games
    let games = match api::games_list(game_name, &api_key).await {
        Ok(games) => games,
        Err(e) => return Err(e),
    };

    // Return the list of games
    Ok(games)
}
/********************************************************************************************************************/

/********************************************************************************************************************/
/// Makes a GET request to "https://rawg.io/api/games/ID_OF_GAME?key=API_KEY"
#[tauri::command]
async fn game_details(game_id: i64) -> Result<serde_json::Value, String> {

    // Retrieve the API_KEY from the
    let api_key = match env::get_id_client().await {
        Ok(key) => key,
        Err(e) => return Err(format!("Failed to get API key: {}", e)),
    };

    // use the api::game_details function to get the details of the game
    let game = match api::game_details(game_id, &api_key).await {
        Ok(game) => game,
        Err(e) => return Err(e),
    };

    // Return the details of the game
    Ok(game)
}
/********************************************************************************************************************/

/********************************************************************************************************************/
/// Gets the list of torrents for a game
#[tauri::command]
async fn get_torrents(game_name: &str) -> Result<Vec<(String, String)>, String> {
    let torrents = scrapers::get_torrents(game_name).await.unwrap();
    Ok(torrents)
}
/********************************************************************************************************************/

/********************************************************************************************************************/
/// Gets the magnet link of a torrent and downloads it
#[tauri::command]
async fn download_torrent(name : &str, game: &str, url: &str, uploader: &str) -> Result<(), String> {

    // Get the magnet link of the torrent
    print!("Getting magnet link... of {}", url);
    let magnet_link: String = scrapers::get_magnet_link(url).await.unwrap();
    print!("{}", magnet_link);

    // Add the download to the database
    let name = name;
    let game = game;
    let link = url;
    let uploader = uploader;
    let path = env::get_download_path().await.unwrap();
    database::add_download(name, game, &link, uploader, &path).await.unwrap();

    // Get the download path
    let download_path = env::get_download_path().await.unwrap();

    // Start the torrent download as a new thread
    thread::spawn(move || {
        torrent::download_torrent(&download_path, &magnet_link); // Directly use the owned Strings
    });

    Ok(())
}
/********************************************************************************************************************/

/********************************************************************************************************************/
/// Update the downloaded path
#[tauri::command]
async fn set_downloaded_path(path: &str) -> Result<(), String> {
    env::set_download_path(path).await.unwrap();
    Ok(())
}
/********************************************************************************************************************/


fn main() {

    // Create the logger
    env_logger::init();

    // Create the .env file
    env::create_env_file().unwrap();

    // Attempt to create the database.sqlite file
    database::create_database_sqlite();

    // Call the start_torrents function
    tokio::runtime::Runtime::new().unwrap().block_on(start_torrents());

    // Run the tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            games_list,
            game_details,
            get_torrents,
            download_torrent,
            set_downloaded_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
