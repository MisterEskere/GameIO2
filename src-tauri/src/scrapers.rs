/*
This file will be used to scrape the 1337x.to website and its magnet links.
*/

use reqwest::{Client, Error, ClientBuilder};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use scraper::Html;
use lazy_static::lazy_static;
use log::{trace, debug};

// Create the client used for the scraping
const DOMAIN: &str = "1337x.to"; // website to scrape
const IP: IpAddr = IpAddr::V4(Ipv4Addr::new(104, 31, 16, 11)); // IP of the website
const PORT: u16 = 0; // Port of the website, unused.
const ADDR: SocketAddr = SocketAddr::new(IP, PORT); // Address of the website

lazy_static! {
    static ref CLIENT: Client = ClientBuilder::new()
        .resolve(DOMAIN, ADDR)
        .build()
        .expect("Failed to build client");
}

/// Function to get the torrents of a game from 1337x.to.
/// It will be called when the user clicks on the download button of a game.
/// Flow:
/// * `Create the URL
/// * `Get the HTML content of the page
/// * `Get the column with the name and the href of the torrent
/// * `Create the result vector
///  
/// # Arguments
/// * `game_name` - A string slice that holds the name of the game to search for.
/// 
pub async fn get_torrents(game_name: &str) -> Result<Vec<(String, String)> , Error> {

    trace!("Getting torrents for game: {}", game_name);

    // Create the URL
    let url = format!("https://1337x.to/category-search/{}/Games/1/", game_name);
    debug!("URL: {}", url);

    // Get the HTML content of the page
    trace!("Getting HTML content");
    let html_content = get_page_html(&url).await?;

    // Select the first column of the table
    let selector = scraper::Selector::parse("td.coll-1").unwrap();

    // Create the result vector
    let mut torrents_pages: Vec<(String, String)> = Vec::new();

    // Iterate over the elements for each torrent found
    trace!("Iterating over the HTML list");
    for element in html_content.select(&selector) {

        // get the name of the torrent
        let name = element.text().collect::<Vec<_>>().join("");

        // get the page of the torrent's magnet link
        let href = element.select(&scraper::Selector
            ::parse("a").unwrap())
            .nth(1)
            .unwrap()
            .value()
            .attr("href")
            .unwrap();
        let href = format!("https://1337x.to{}", href);

        // push the name and the href to the result vector
        torrents_pages.push((name, href.to_string()));
    }
    trace!("Number of torrents: {}", torrents_pages.len());

    // Return the result vector
    Ok(torrents_pages)
}

/// Function to get the magnet link of a torrent from 1337x.to.
/// It will be called when a user clicks on the torrent entry.
/// 
/// Flow:
/// * `Get the HTML content of the page
/// * `Extract all the magnet link
/// * `Return the magnet link
/// 
pub async fn get_magnet_link(url: &str) -> Result<String, Error> {
    // Get the HTML content of the page
    let html_content = get_page_html(url).await?;

    // Extract all the <a> tags
    let selector = scraper::Selector
        ::parse("a").unwrap();
    let mut a_tags = html_content.select(&selector);

    // Out of those tagas select the one with the magnet link as href
    let magnet_link = a_tags
        .find(|tag| tag.value().attr("href").unwrap().starts_with("magnet"))
        .unwrap()
        .value()
        .attr("href")
        .unwrap();
    
    print!("Magnet link: {}", magnet_link);
    Ok(magnet_link.to_string())
}

/// Function to get the HTML content of a page.
/// 
/// Arguments:
/// * `url` - A string slice that holds the URL of the page.
/// 
/// Returns:
/// A Result enum with the HTML content of the page.
async fn get_page_html(url: &str) -> Result<Html, Error> {

    // Send a GET request to the URL and get the response text
    let response_text = CLIENT.get(url).send().await?.text().await?;
    let document = Html::parse_document(&response_text);

    Ok(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_torrents() {
        let torrents = get_torrents("Cyberpunk 2077").await.unwrap();
        assert!(torrents.len() > 0);
    }

    #[tokio::test]
    async fn test_get_magnet_link() {
        let magnet_link = get_magnet_link("https://1337x.to/torrent/4640384/Cyberpunk-2077-v1-06-REPACK-CODEX/").await.unwrap();
        assert!(magnet_link.starts_with("magnet:?"));
    }
}