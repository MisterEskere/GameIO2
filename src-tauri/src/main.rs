// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use scraper::{Html, Selector};
use serde_json::{json, Value};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/*
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/

/// Searches for game repacks on the FitGirl website using a specified search argument.
///
/// This function performs a search on the FitGirl Repacks site by constructing a URL with the search argument.
/// It then downloads the search results page, parses the HTML content to extract game titles and links,
/// and stores them in a vector of JSON objects.
///
/// # Arguments
///
/// * `search_argument` - A string slice that holds the search term to be used in the query.
///
/// # Examples
///
/// ```
/// fitgirl_search("cyberpunk");
/// ```
///
/// # Returns
/// 
/// A vector of JSON objects, where each object contains the title and link of a game repack.
///
/// This function downloads the search results into a temporary HTML file named `tmp.html`, which is then read and parsed.
/// Ensure that the `tmp.html` file is managed appropriately.
#[tauri::command]
fn fitgirl_search(search_argument: &str) -> Vec<Value> {

    // Download the search results of https://fitgirl-repacks.site
    let domain = "fitgirl-repacks.site";
    let url = format!("https://190.115.31.179/?s={}", search_argument);
    page_downloader(&url, domain);


    // Extract the contente of tmp.html
    let content = std::fs::read_to_string("tmp.html").unwrap();
    let document = Html::parse_document(&content);
    let selector = Selector::parse("article[class*='post type-post status-publish format-standard hentry category-lossless-repack']").unwrap();

    // Create a vector of JSON objects
    let mut games: Vec<Value> = Vec::new();

    // Iterate over the elements of the document
    for element in document.select(&selector) {
        
        //TODO optimize the code below
        // from the element parse the tag <h1 class="entry-title"> and get the text
        let game_title = element.select(&Selector::parse("h1[class*='entry-title']").unwrap()).next().unwrap().text().collect::<Vec<_>>().join(" ");

        // from the element parse the tag <a> inside the tag <h1 class="entry-title"> and get the href
        let game_link = element.select(&Selector::parse("h1[class*='entry-title'] a").unwrap()).next().unwrap().value().attr("href").unwrap();

        // create new JSON object
        let game = json!({
            "title": game_title,
            "link": game_link
        });

        // push the JSON object to the vector
        games.push(game);
    }

    // return the games vector
    games

}


fn page_downloader(url: &str, domain: &str) { 
    Command::new("python")
        .arg("page_downloader.py")
        .arg(url)
        .arg(domain)
        .status()
        .expect("failed to execute process");
}
/*
fn main() {
    let tmp_vector: Vec<Value> = fitgirl_search("gta");

    for game in tmp_vector {
        println!("Title: {}", game["title"]);
        println!("Link: {}", game["link"]);
    }
}
*/


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fitgirl_search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
