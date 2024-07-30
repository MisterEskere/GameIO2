use reqwest::Client;
use serde_json::Value;
use serde_json::json;
use lazy_static::lazy_static;
use log::{trace, debug, error};

// Client instance to make requests
lazy_static! {
    static ref CLIENT: Client = Client::new();
}

/// Function to get the list of games from the RAWG API
/// It will be called when the used opens the Home page and searches for a game
/// 
/// Arguments:
///    - game_name: &str - The name of the game to search for, if empty, it will return the top games
///    - api_key: &str - The API key to use for the request
/// 
/// Returns:
///   - Result<Vec<Value>, String> - A vector of JSON objects containing the details of the games
///   - String - An error message if the request fails
///
/// Example:
/// ```rust
///    let games = games_list("Cyberpunk 2077", "Y0ur_Ap1_K3y").await.unwrap();
/// ```
/// 
pub async fn games_list(game_name: &str, api_key: &str) -> Result<Vec<Value>, String> {

    // Create the URL
    let url = format!("https://rawg.io/api/games?page=1&page_size=100&search={}&parent_platforms=1,6,5&stores=1,5,11&key={}", game_name, api_key);
    trace!("Requesting games lists");
    debug!("URL: {}", url);

    // Make the get request
    let response = match get_request(&url).await {
        Ok(json) => json,
        Err(e) => return Err(e),
    };

    // Extract the "results" list from the JSON response
    let results = match response["results"].as_array() {
        Some(results) => results,
        None => return Err("Failed to extract 'results' from response".to_string()),
    };

    Ok(results.clone())
}


pub async fn game_details(game_id: i64, api_key: &str) -> Result<serde_json::Value, String> {

    // Create the URL
    let url = format!("https://rawg.io/api/games/{}?key={}", game_id, api_key);

    // Make the get request
    let response = match get_request(&url).await {
        Ok(json) => json,
        Err(e) => return Err(e),
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

/// Function to make a GET request to a URL and return the response as JSON
/// 
/// Arguments:
///   - url: &str - The URL to make the request to  
/// 
/// Returns:
///  - Result<Value, String> - The response as a JSON object
/// 
async fn get_request(url: &str) -> Result<serde_json::Value, String> {
    // Make the request
    let response = match CLIENT.get(url).send().await {
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

    Ok(response)
}
