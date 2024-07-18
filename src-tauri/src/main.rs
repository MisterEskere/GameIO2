// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod env;
mod scrapers;
mod torrent;
mod database;

use serde_json::json;

/// Makes a GET request to "https://rawg.io/api/games?page=1&page_size=10&search=NAME_OF_GAME&parent_platforms=1,6,5&stores=1,5,11"
#[tauri::command]
async fn games_list(game_name: &str) -> Result<Vec<serde_json::Value>, String> {

    // Retrieve the API_KEY from the .env file, managing the case that get_api_key returns an error
    let api_key = match env::get_api_key().await {
        Ok(key) => key,
        Err(e) => return Err(format!("Failed to get API key: {}", e)),
    };

    // Create the URL
    let url: String = format!("https://rawg.io/api/games?page=1&page_size=100&search={}&parent_platforms=1,6,5&stores=1,5,11&key={}", game_name, api_key);

    // Make the request
    let response = match env::get_request(&url).await {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()), // Convert the error to a String here
    };

    // Of the response, extract the "next" field TODO da implementare
    // let next = response["next"].as_str().unwrap()?;
    

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
    let api_key = match env::get_api_key().await {
        Ok(key) => key,
        Err(e) => return Err(format!("Failed to get API key: {}", e)),
    };

    // Create the URL
    let url = format!("https://rawg.io/api/games/{}?key={}", game_id, api_key);

    // Make the request
    let response = match env::get_request(&url).await {
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
    match env::set_api_key(api_key).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn get_api_key() -> Result<String, String> {
    match env::get_api_key().await {
        Ok(api_key) => Ok(api_key),
        Err(e) => Err(format!("Failed to get API key: {}", e)),
    }
}

#[tauri::command]
async fn get_torrents(game_name: &str) -> Result<Vec<(String, String)>, String> {
    let torrents = scrapers::get_torrents(game_name).await.unwrap();
    Ok(torrents)
}

#[tauri::command]
async fn download_torrent(url: &str) -> Result<String, String> {
    let magnet_link: String = scrapers::get_magnet_link(url).await.unwrap();

    let handle = torrent::download_torrent("downloads", &magnet_link).await.unwrap();

    Ok(handle().to_string())
}

/*
#[tokio::main]
async fn main() {
    let name = "cyberpunk";

    // Get the list of torrents
    let torrents = scrapers::get_torrents(name).await.unwrap();
    println!("Torrents: {:?}", torrents);

    // wait 5 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));

    // get the magnet link of the first torrent
    let magnet_link = scrapers::get_magnet_link(&torrents[0].1).await.unwrap();

    println!("Magnet link: {}", magnet_link);
}
*/

/// This function will be used to start all the torrents previously started. 
/// This will be called at the beginning of the application.
async fn start_torrents() {
    let torrents = database::get_downloads().await.unwrap();

    for torrent in torrents {
        let magnet_link = torrent["link"].as_str().unwrap();
        torrent::download_torrent("downloads", magnet_link).await.unwrap();
    }
}

/// This function will be used to start a new torrent download.
/// It will take the magnet link of the torrent and start the download.
#[tauri::command]
async fn start_torrent(magnet_link: &str) {

    // Retrive the downloads folder from the .env file


    // Start the torrent download
    torrent::download_torrent("downloads", magnet_link).await.unwrap();
    
}

fn main() {

    // Attempt to create the database.sqlite file
    database::create_database_sqlite();

    // Call the start_torrents function
    tokio::runtime::Runtime::new().unwrap().block_on(start_torrents());

    // Run the tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![games_list, game_details, set_api_key, get_api_key, get_torrents, get_magnet_link])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
