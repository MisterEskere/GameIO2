use std::env::VarError;
use std::fs::{self, OpenOptions};
use std::io::{self, Write}; // Keep this import as it includes Write

use std::collections::HashMap;

/// Function to make a GET request to a URL and return the JSON response.
/// It will be used to make requests to the RAWG API.
//pub async fn get_request(url: &str) -> Result<Value, reqwest::Error> {
//    let client = reqwest::Client::new();
//    let res = client.get(url).send().await?;

    // Directly parse the response body as JSON
//    let json = res.json::<Value>().await?;

//    Ok(json)
//}

/// Function to retrieve the ID_CLIENT from the environment file.
/// It will be used to make requests to the IGDB API.
pub async fn get_id_client() -> Result<String, VarError> {
    dotenv::dotenv().ok();

    let id_client = std::env::var("ID_CLIENT")?;

    Ok(id_client)
}

/// Function to retrieve the SECRET from the environment file.
/// It will be used to make requests to the IGDB API.
pub async fn get_secret() -> Result<String, VarError> {
    dotenv::dotenv().ok();

    let secret = std::env::var("SECRET")?;

    Ok(secret)
}

/// Function to retrieve the DOWNLOAD_PATH from the environment file.
/// It will be used to know where to download the torrents.
pub async fn get_download_path() -> Result<String, VarError> {
    dotenv::dotenv().ok();

    let download_path = std::env::var("DOWNLOAD_PATH")?;

    Ok(download_path)
}

/// Function to set the DOWNLOAD_PATH in the environment file.
/// It will be used to update the download path in the application.
pub async fn set_download_path(download_path: &str) -> Result<(), io::Error> {
    dotenv::dotenv().ok();

    let mut env_vars = HashMap::new();

    // Read the current .env file
    if let Ok(contents) = fs::read_to_string(".env") {
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once('=') {
                env_vars.insert(key.to_string(), value.to_string());
            }
        }
    }

    // Update the DOWNLOAD_PATH variable
    env_vars.insert("DOWNLOAD_PATH".to_string(), download_path.to_string());

    // Write the updated contents back to the .env file
    let mut file = OpenOptions::new().write(true).truncate(true).open(".env")?;
    for (key, value) in env_vars {
        writeln!(file, "{}={}", key, value)?;
    }

    Ok(())
}

/// Function to create the .ENV file with the KEYS and PATHS needed.
/// It will be called at the beginning of the application.
/// This is necessary to keep the API keys between sessions.
pub fn create_env_file() -> Result<(), std::io::Error> {

    // Attempt to create the .env file, in case it already exists exit early
    if std::path::Path::new(".env").exists() {
        return Ok(());
    }

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(".env")?;

    // Get the KEYS from the environment variables
    let id_client = std::env::var("ID_CLIENT").unwrap();
    let secret = std::env::var("SECRET").unwrap();

    // Store the PathBuf in a variable
    let download_path_buf = dirs::download_dir().unwrap();

    // Convert the PathBuf to a &str, ensuring the PathBuf lives long enough
    let download_path = download_path_buf.to_str().unwrap();

    // Write the API_KEY and DOWNLOAD_PATH to the file
    file.write_all(format!("ID_CLIENT={}\n", id_client).as_bytes())?;
    file.write_all(format!("SECRET={}\n", secret).as_bytes())?;

    file.write_all(format!("DOWNLOAD_PATH={}", download_path).as_bytes())?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_id_client() {
        let id_client = get_id_client().await.unwrap();

        assert!(id_client.len() > 0);
    }

    #[tokio::test]
    async fn test_get_id_secret() {
        let id_secret = get_secret().await.unwrap();

        assert!(id_secret.len() > 0);
    }

    #[tokio::test]
    async fn test_get_download_path() {
        let download_path = get_download_path().await.unwrap();

        assert!(download_path.len() > 0);
    }

    #[tokio::test]
    async fn test_set_download_path() {
        let result = set_download_path("/home/user/Downloads").await.unwrap();

        assert_eq!(result, ());
    }

    #[test]
    fn test_create_env_file() {
        let result = create_env_file().unwrap();

        assert_eq!(result, ());
    }
}