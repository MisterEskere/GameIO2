use serde_json::Value;
use std::env::VarError;
use std::io::Write;

/// Function to make a GET request to a URL and return the JSON response.
pub async fn get_request(url: &str) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;

    // Directly parse the response body as JSON
    let json = res.json::<Value>().await?;

    Ok(json)
}

/// Function to retrieve the API_KEY from the environment file.
pub async fn get_api_key() -> Result<String, VarError> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("API_KEY")?;

    Ok(api_key)
}

/// Function to retrieve the DOWNLOAD_PATH from the environment file.
pub async fn get_download_path() -> Result<String, VarError> {
    dotenv::dotenv().ok();

    let download_path = std::env::var("DOWNLOAD_PATH")?;

    Ok(download_path)
}

/// Function to set the DOWNLOAD_PATH in the environment file.
pub async fn set_download_path(download_path: &str) -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    let mut file = std::fs::OpenOptions::new().write(true).open(".env")?;

    file.write_all(format!("DOWNLOAD_PATH={}", download_path).as_bytes())?;

    Ok(())
}

/// Function to create the .ENV file with the API_KEY and the DOWNLOAD_PATH.
pub fn create_env_file() -> Result<(), std::io::Error> {
    // Attempt to create the .env file, in case it already exists exit early
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(".env")?;

    // Get the API_KEY from the environment
    let api_key = std::env::var("API_KEY").unwrap();

    // Store the PathBuf in a variable
    let download_path_buf = dirs::download_dir().unwrap();

    // Convert the PathBuf to a &str, ensuring the PathBuf lives long enough
    let download_path = download_path_buf.to_str().unwrap();

    // Write the API_KEY and DOWNLOAD_PATH to the file
    file.write_all(format!("API_KEY={}\n", api_key).as_bytes())?;
    file.write_all(format!("DOWNLOAD_PATH={}", download_path).as_bytes())?;

    Ok(())
}
