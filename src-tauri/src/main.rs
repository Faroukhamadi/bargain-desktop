// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use serde::{Deserialize, Serialize};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, get_chats, get_chat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Deserialize, Serialize)]
struct Message {
    from: String,
    text: String,
}

#[derive(Deserialize, Serialize)]
struct Chats {
    id: String,
    title: String,
    created_at: String,
    messages: Vec<Message>,
}

#[tauri::command]
fn get_chats() -> Result<String, String> {
    let path = "/tmp/bargain";
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut chats = Vec::new();
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                let content = fs::read_to_string(path).unwrap();
                let chat: Chats = serde_json::from_str(&content).unwrap();
                chats.push(chat);
            }
            let json = serde_json::to_string(&chats).unwrap();
            Ok(json)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn get_chat(id: String) -> Result<String, String> {
    let path = format!("/tmp/bargain/chat_{}.json", id);
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e.to_string()),
    }
}
