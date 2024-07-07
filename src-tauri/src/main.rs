// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
use serde_json::{json, Value};

/// Makes a GET request to "https://rawg.io/api/games?page=1&page_size=10&search=NAME_OF_GAME&parent_platforms=1,6,5&stores=1,5,11"
async fn games_list(game: &str) -> Result<Vec<serde_json::Value>, reqwest::Error> {

    // Retrieve the API_KEY from the .env file
    let api_key = utils::get_api_key().await;

    // Create the URL
    let url: String = format!("https://rawg.io/api/games?page=1&page_size=10&search={}&parent_platforms=1,6,5&stores=1,5,11&key={}", game, api_key);

    // Make the request
    let response = utils::get_request(&url).await?;

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
async fn game_details(game_id: &i64) -> Result<Value, reqwest::Error> {

    // Retrieve the API_KEY from the .env file
    let api_key = utils::get_api_key().await;

    // Create the URL
    let url = format!("https://rawg.io/api/games/{}?key={}", game_id, api_key);

    // Make the request
    let response = utils::get_request(&url).await?;

    // Of the response, extract the "id", "slug", "name", "name_original", "description", 
    // "metacritic", "image_background", "background_image_additional", "released", "genres"
    let id = response["id"].as_i64().unwrap();
    let slug = response["slug"].as_str().unwrap();
    let name = response["name"].as_str().unwrap();
    let name_original = response["name_original"].as_str().unwrap();
    let description = response["description"].as_str().unwrap();
    let metacritic = response["metacritic"].as_i64().unwrap();
    let background_image = response["background_image"].as_str().unwrap();
    let background_image_additional = response["background_image_additional"].as_str().unwrap();
    let released = response["released"].as_str().unwrap();
    let genres = response["genres"].as_array().unwrap();

    // make a JSON object with the extracted fields
    let game = json!({
        "id": id,
        "slug": slug,
        "name": name,
        "name_original": name_original,
        "description": description,
        "metacritic": metacritic,
        "background_image": background_image,
        "background_image_additional": background_image_additional,
        "released": released,
        "genres": genres
    });

    // Return the response
    Ok(game)
}

#[tokio::main]
async fn main() {
    // test the games_list function
    let games = games_list("League of Legends").await.unwrap();
    println!("{:?}", games);

    let game = game_details(&23598).await.unwrap();
    println!("{:?}", game);
}


/*
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fitgirl_search, fitgirl_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
*/