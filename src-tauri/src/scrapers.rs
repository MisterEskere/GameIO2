/// pub function to download the HTLM content of a URL and return it as a String
/// # Arguments
/// * `url` - A string slice that holds the URL to download
/// # Returns
/// * A Result containing a String with the HTML content or an error message
/// # Example
/// ```
/// let url = "https://www.rust-lang.org/";
/// let html = download_html(url).await.unwrap();
/// ```
/// # Note
/// This function uses the reqwest crate to download the HTML content of a URL.
/// The function is asynchronous and must be awaited.
/// The function returns a Result containing a String with the HTML content or an error message.
pub async fn download_html(game_name: &str) -> Result<String, reqwest::Error> {

    // create the url = "https://1337x.to/search/game_name/1/"
    let url = format!("https://1337x.to/search/{}/1/", game_name);

    let client = reqwest::Client::new();
    match client.get(url).send().await {
        Ok(res) => match res.text().await {
            Ok(text) => Ok(text),
            Err(e) => {
                eprintln!("Error getting text from response: {}", e);
                Err(e)
            },
        },
        Err(e) => {
            eprintln!("Error sending request: {}", e);
            Err(e)
        },
    }
}