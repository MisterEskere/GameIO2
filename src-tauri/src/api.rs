use anyhow::Ok;
use reqwest::Client;
use serde_json::Value;
use lazy_static::lazy_static;

use crate::env;

// Client instance to make requests
lazy_static! {
    static ref CLIENT: Client = Client::new();
}

/// Function to make a GET request to a URL and return the response as a String
/// 
/// Arguments:
/// - url: &str - The URL to make the request to
/// 
/// Returns:
/// - response: String - The response as a String
/// 
/// # Errors
/// - Any error that occurs during the request
/// 
/// # Example
/// ```
/// let response = get_request("https://httpbin.org/get").await.unwrap();
/// ```
async fn get_request(url: &str) -> Result<Value, anyhow::Error> {

    // Make the GET request
    let response = CLIENT.get(url)
        .send()
        .await?
        .text()
        .await?;

    // Convert the response to a JSON object
    let response = serde_json::from_str(&response)?;

    Ok(response)
}

/// Function to make a POST request to a URL and return the response as JSON
/// 
/// Arguments:
///  - url: &str - The URL to make the request to
/// - body: String - The body of the POST request
/// 
/// Returns:
/// - response: Value - The response as a JSON object
/// 
/// # Errors
/// - Any error that occurs during the request
/// 
/// # Example
/// ```
/// let response = post_request("https://httpbin.org/post", "body".to_string()).await.unwrap();
/// ```
/// 
async fn post_request(url: &str, body: String) -> Result<Value, anyhow::Error> {

    // Make the POST request
    let response = CLIENT.post(url)
        .body(body) // Pass the String directly
        .send()
        .await?
        .text()
        .await?;

    // Convert the response to a JSON object
    let response = serde_json::from_str(&response)?;

    Ok(response)
}


/// Function to get the access token from the Twitch API
/// 
/// # Returns
/// - `access_token: String` - The access token
/// 
/// # Errors
/// - Any error that occurs during the request
/// 
/// # Example
/// ```
/// let token = get_token().await.unwrap();
/// ```
///
async fn get_token() -> Result<String, anyhow::Error> {

    // Retrieve the ID_CLIENT and SECRET from the environment file
    let id_client = env::get_id_client().await.unwrap();
    let secret = env::get_secret().await.unwrap();

    // make the POST request
    let url = format!("https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials", id_client, secret);

    let response = post_request(&url, "".to_string()).await?;

    // extract the access token from the response
    let access_token = response["access_token"].as_str().unwrap().to_string();

    Ok(access_token)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_request() {
        let response = get_request("https://httpbin.org/get").await.unwrap();
        print!("{:?}", response);
        assert!(response.is_object());
    }

    #[tokio::test]
    async fn test_post_request() {
        let response = post_request("https://httpbin.org/post", "".to_string()).await.unwrap();
        print!("{:?}", response);
        assert!(response.is_object());
    }

    #[tokio::test]
    async fn test_get_token() {
        let token = get_token().await.unwrap();
        assert!(!token.is_empty());
    }
}
