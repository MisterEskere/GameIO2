// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use scraper::{Html, Selector};
use tauri::http::header;
use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/*
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/


fn fitgirl_search(search_argument: &str) {
    // Download the search results of https://fitgirl-repacks.site
    let headers = "{'Host': 'fitgirl-repacks.site'";
    let url = format!("https://fitgirl-repacks.site/?s={}", search_argument);
    page_downloader(&url, headers);
}


fn page_downloader(url: &str, headers: &str) { 
    Command::new("python")
        .arg("page_downloader.py")
        .arg(url)
        .arg(headers)
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