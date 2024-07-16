// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

use serde_json::json;
use librqbit::*;
use tokio::runtime::Runtime;

/// Makes a GET request to "https://rawg.io/api/games?page=1&page_size=10&search=NAME_OF_GAME&parent_platforms=1,6,5&stores=1,5,11"
#[tauri::command]
async fn games_list(game_name: &str) -> Result<Vec<serde_json::Value>, String> {

    // Retrieve the API_KEY from the .env file, managing the case that get_api_key returns an error
    let api_key = match utils::get_api_key().await {
        Ok(key) => key,
        Err(e) => return Err(format!("Failed to get API key: {}", e)),
    };

    // Create the URL
    let url: String = format!("https://rawg.io/api/games?page=1&page_size=100&search={}&parent_platforms=1,6,5&stores=1,5,11&key={}", game_name, api_key);

    // Make the request
    let response = match utils::get_request(&url).await {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()), // Convert the error to a String here
    };

    // Of the response, extract the "next" field TODO da implementare
    let next = response["next"].as_str().unwrap();

    // Of the response, extract the "results" list
    let results = response["results"].as_array().unwrap();

    // Of the "results" list, extract: "slug", "name", "released", "background_image", "metacritic", "id"
    // saves them in a list of json objects
    let mut games: Vec<serde_json::Value> = Vec::new();
    for game in results {
        let slug = game["slug"].as_str().unwrap();
        let name = game["name"].as_str().unwrap();
        let background_image = game["background_image"].as_str().unwrap();
        let id = game["id"].as_i64().unwrap();

        let game = json!({
            "id": id,
            "slug": slug,
            "name": name,
            "background_image": background_image
        });

        games.push(game);
    }

    // Return the response
    Ok(games)
}

/// Makes a GET request to "https://rawg.io/api/games/ID_OF_GAME?key=API_KEY"
#[tauri::command]
async fn game_details(game_id: i64) -> Result<serde_json::Value, String> {

    // Retrieve the API_KEY from the .env file, managing the case that get_api_key returns an error
    let api_key = match utils::get_api_key().await {
        Ok(key) => key,
        Err(e) => return Err(format!("Failed to get API key: {}", e)),
    };

    // Create the URL
    let url = format!("https://rawg.io/api/games/{}?key={}", game_id, api_key);

    // Make the request
    let response = match utils::get_request(&url).await {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()), // Convert the error to a String here
    };

    // Of the response, extract the "id", "slug", "name", "name_original", "description", 
    // "metacritic", "image_background", "background_image_additional", "released", "genres"
    let id = response["id"].as_i64().unwrap();
    let slug = response["slug"].as_str().unwrap();
    let name = response["name"].as_str().unwrap();
    let description = response["description"].as_str().unwrap();
    let background_image = response["background_image"].as_str().unwrap();
    let background_image_additional = response["background_image_additional"].as_str().unwrap();
    let released = response["released"].as_str().unwrap();
    let genres = response["genres"].as_array().unwrap();

    // make a JSON object with the extracted fields
    let game: serde_json::Value = json!({
        "id": id,
        "slug": slug,
        "name": name,
        "description": description,
        "background_image": background_image,
        "background_image_additional": background_image_additional,
        "released": released,
        "genres": genres
    });

    // Return the response
    Ok(game)
}

/// Set and get the API key
#[tauri::command]
async fn set_api_key(api_key: &str) -> Result<(), String> {
    match utils::set_api_key(api_key).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn get_api_key() -> Result<String, String> {
    match utils::get_api_key().await {
        Ok(api_key) => Ok(api_key),
        Err(e) => Err(format!("Failed to get API key: {}", e)),
    }
}

/*
fn main() {

    // get the current path
    let current_path = std::env::current_dir().unwrap();
    let current_path = current_path.to_str().unwrap();

    tokio_test::block_on(async {
        let session = Session::new(current_path.into()).await.unwrap();
        let managed_torrent_handle = session.add_torrent(
           AddTorrent::from_url("magnet:?xt=urn:btih:F4232511728CEC01EC5E6B4F6C16A53ED299E005&dn=Mushoku+Tensei%3A+Jobless+Reincarnation+Quest+of+Memories+%28v1.0.3+%2B+Windows+7+Fix%2C+MULTi3%29+%5BFitGirl+Repack%5D&tr=udp%3A%2F%2Fopentracker.i2p.rocks%3A6969%2Fannounce&tr=http%3A%2F%2Ftracker.gbitt.info%3A80%2Fannounce&tr=http%3A%2F%2Ftracker.ccp.ovh%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.ccp.ovh%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce&tr=udp%3A%2F%2Ftracker.openbittorrent.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.theoks.net%3A6969%2Fannounce&tr=https%3A%2F%2Ftracker.tamersunion.org%3A443%2Fannounce&tr=http%3A%2F%2Fopen.acgnxtracker.com%3A80%2Fannounce&tr=http%3A%2F%2Fopen.acgtracker.com%3A1096%2Fannounce&tr=http%3A%2F%2Ftracker.bt4g.com%3A2095%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=http%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce&tr=udp%3A%2F%2Fopentracker.i2p.rocks%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.internetwarriors.net%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969%2Fannounce&tr=udp%3A%2F%2Fcoppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.zer0day.to%3A1337%2Fannounce"),
           None // options
        ).await.unwrap().into_handle().unwrap();
        managed_torrent_handle.wait_until_completed().await.unwrap();
    })

}
*/


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![games_list, game_details, set_api_key, get_api_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
