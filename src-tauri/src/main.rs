// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use scraper::{selector, Html, Selector};
use serde_json::{json, Value};
use std::collections::HashMap;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/*
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/


fn fitgirl_search(search_argument: &str) {

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

    for element in document.select(&selector) {
        
        // from the element parse the tag <h1 class="entry-title"> and get the text
        let game_title = element.select(&Selector::parse("h1[class*='entry-title']").unwrap()).next().unwrap().text().collect::<Vec<_>>().join(" ");
        print!("Game title: {}\n", game_title);
    }

    // print the vector of JSON objects
    for game in games {
        println!("---------------------");
        println!("{}", game);
    }

}


fn page_downloader(url: &str, domain: &str) { 
    Command::new("python")
        .arg("page_downloader.py")
        .arg(url)
        .arg(domain)
        .status()
        .expect("failed to execute process");
}

fn main() {
    fitgirl_search("gta");
}

/*
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
*/