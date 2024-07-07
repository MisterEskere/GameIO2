use std::env;
use dotenv::dotenv;
use std::process::Command;
use serde_json::Value; // Ensure serde_json is added to your Cargo.toml

pub async fn get_request(url: &str) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;

    // Directly parse the response body as JSON
    let json = res.json::<Value>().await?;

    Ok(json)
}

pub async fn get_api_key() -> String {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    api_key
}


fn page_downloader(url: &str, domain: &str) { 
    Command::new("python")
        .arg("page_downloader.py")
        .arg(url)
        .arg(domain)
        .status()
        .expect("failed to execute process");
}