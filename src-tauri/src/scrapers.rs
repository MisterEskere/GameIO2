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

const TRUSTED_UPLOADERS: [&str; 8] = ["0xEMPRESS", "anadius", "DODI", "FitGirl", "JohnCena141", "KaOsKrew", "s7on3r", "TinyRepacks"];

pub async fn get_torrents(game_name: &str) -> Result<Vec<(String, String)> , Error> {

    // Create the URL
    let url = format!("https://1337x.to/category-search/{}/Games/1/", game_name);

    // Get the HTML content of the page
    let html_content = get_page_html(&url).await?;

    // Extract all the <tr> elements of the html
    let selector = scraper::Selector
        ::parse("tr").unwrap();

    // Create the result vector
    let mut torrents_pages: Vec<(String, String)> = Vec::new();

 
    // Iterate over all the <tr> elements
    for element in html_content.select(&selector) {
        // Bind the result of `select` to a variable to extend its lifetime
        let ths = element.select(&scraper::Selector::parse("th").unwrap()).collect::<Vec<_>>();
    
        println!("{:?}", ths);
    
        // Since `ths` is now a Vec, we can directly use it without cloning
        let uploader = ths.get(5).unwrap().text().collect::<Vec<_>>().join("");
        if !TRUSTED_UPLOADERS.contains(&&*uploader) {
            continue;
        }

        // get the name of the torrent, text of the 1st th element
        let name = ths.get(0).unwrap().text().collect::<Vec<_>>().join("");

        // get the href of the 1st th element
        let href = ths.get(0).unwrap().select(&scraper::Selector::parse("a").unwrap()).next().unwrap().value().attr("href").unwrap();
        let href = format!("https://1337x.to{}", href);

        // push the name and the href to the result vector
        torrents_pages.push((name, href));
    }

    // Return the result vector
    Ok(torrents_pages)
}

pub async fn get_magnet_link(url: &str) -> Result<String, Error> {

    // Get the HTML content of the page
    let html_content = get_page_html(url).await?;

    // Select the magnet link, get the href of the <a> with the id "openPopup"
    let magnet_link = html_content.select(&scraper::Selector
        ::parse("a#openPopup").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    Ok(magnet_link.to_string())
}

async fn get_page_html(url: &str) -> Result<Html, Error> {

    // Send a GET request to the URL and get the response text
    let response_text = CLIENT.get(url).send().await?.text().await?;
    let document = Html::parse_document(&response_text);

    Ok(document)
}