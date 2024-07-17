use reqwest::{Client, Error, ClientBuilder};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use scraper::Html;
use lazy_static::lazy_static;

// Create the client used for the scraping
const DOMAIN: &str = "1337x.to";
const IP: IpAddr = IpAddr::V4(Ipv4Addr::new(104, 31, 16, 11));
const PORT: u16 = 443;
const ADDR: SocketAddr = SocketAddr::new(IP, PORT);

lazy_static! {
    static ref CLIENT: Client = ClientBuilder::new()
        .resolve(DOMAIN, ADDR)
        .build()
        .expect("Failed to build client");
}


pub async fn get_torrents(game_name: &str) -> Result<Vec<(String, String)> , Error> {

    // Create the URL
    let url = format!("https://1337x.to/category-search/{}/Games/1/", game_name);

    // Get the HTML content of the page
    let html_content = get_page_html(&url).await?;

    // Select the first column of the table
    let selector = scraper::Selector::parse("td.coll-1").unwrap();

    // Create the result vector
    let mut torrents_pages: Vec<(String, String)> = Vec::new();

    // Iterate over the elements for each torrent found
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

    // Return the result vector
    Ok(torrents_pages)
}

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

async fn get_page_html(url: &str) -> Result<Html, Error> {

    // Send a GET request to the URL and get the response text
    let response_text = CLIENT.get(url).send().await?.text().await?;
    let document = Html::parse_document(&response_text);

    Ok(document)
}