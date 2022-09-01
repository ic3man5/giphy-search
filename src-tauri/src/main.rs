#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod gif;
use gif::giphy::{Giphy, GiphyURLType, DEFAULT_API_KEY};
use clipboard_win::{formats, set_clipboard};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[tauri::command]
fn search_gif(name: &str) -> Vec<String> {
    // Search for the gifs
    let giphy = Giphy::new(DEFAULT_API_KEY.to_string());
    let search_text = name.to_string();
    let urls = giphy
        .search_url(&search_text, GiphyURLType::Original, Some(300))
        .unwrap();
    // Pick three random ones
    let mut random_urls = Vec::new();
    for _ in 0..3 {
        let mut rng = thread_rng();
        let result = urls.choose(&mut rng).unwrap();
        random_urls.push(result.clone());
    }
    random_urls
}

#[tauri::command]
fn copy_to_clipboard(name: &str, url: &str) -> String {
    let clipboard_contents = format!("`{name}`\n{url}");
    set_clipboard(formats::Unicode, clipboard_contents).unwrap();
    format!("Copied {url} to clipboard!")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_gif, copy_to_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
