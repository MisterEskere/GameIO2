use reqwest::Client;
use serde_json::Value;
use serde_json::json;
use lazy_static::lazy_static;

// lazy_static client
lazy_static! {
    static ref CLIENT: Client = Client::new();
}

pub async fn games_list(game_name: &str, api_key: &str) -> Result<Vec<Value>, String> {
    // Create the URL
    let url = format!("https://rawg.io/api/games?page=1&page_size=100&search={}&parent_platforms=1,6,5&stores=1,5,11&key={}", game_name, api_key);

    // Create a client instance
    let client = Client::new();

    // Make the request
    let response = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()), // Convert the error to a String here
    };

    // Check if the response status is success
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    // Parse the response body as JSON
    let response_json = match response.json::<Value>().await {
        Ok(json) => json,
        Err(e) => return Err(e.to_string()),
    };

    // Extract the "results" list from the JSON response
    let results = match response_json["results"].as_array() {
        Some(results) => results,
        None => return Err("Failed to extract 'results' from response".to_string()),
    };

    // Process the "results" list
    let mut games: Vec<Value> = Vec::new();
    for game in results {
        if let (Some(slug), Some(name), Some(background_image), Some(id)) = (
            game["slug"].as_str(),
            game["name"].as_str(),
            game["background_image"].as_str(),
            game["id"].as_i64(),
        ) {
            let game_json = json!({
                "id": id,
                "slug": slug,
                "name": name,
                "background_image": background_image,
            });
            games.push(game_json);
        }
    }

    Ok(games)
}


pub async fn game_details(game_id: i64, api_key: &str) -> Result<serde_json::Value, String> {

    // Create the URL
    let url = format!("https://rawg.io/api/games/{}?key={}", game_id, api_key);

    // Make the request
    let response = match CLIENT.get(&url).send().await {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()), // Convert the error to a String here
    };

    // Check if the response status is success
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    // Parse the response body as JSON
    let response = match response.json::<Value>().await {
        Ok(json) => json,
        Err(e) => return Err(e.to_string()),
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