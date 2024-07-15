use std::env;
use dotenv::dotenv;
use serde_json::Value;
use std::fs::OpenOptions;
use std::path::Path;

pub async fn get_request(url: &str) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;

    // Directly parse the response body as JSON
    let json = res.json::<Value>().await?;

    Ok(json)
}

pub async fn get_api_key() -> String {
    // Check if the .env file exists, create it if it doesn't
    let env_path = Path::new(".env");
    if !env_path.exists() {
        let _ = OpenOptions::new().create(true).write(true).open(env_path);
    }

    // Retrieve the API_KEY from the .env file
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set!!!");
    api_key
}

pub async fn set_api_key(api_key: &str) {

    // Check if the .env file exists, create it if it doesn't
    let env_path = Path::new(".env");
    if !env_path.exists() {
        let _ = OpenOptions::new().create(true).write(true).open(env_path);
    }

    // Set the API_KEY in the .env file
    dotenv().ok();
    env::set_var("API_KEY", api_key);
}
