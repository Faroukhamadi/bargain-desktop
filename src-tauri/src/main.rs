// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use serde::{Deserialize, Serialize};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, get_messages])
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

// {
//     2   │   "id": "1dee5252-0855-470d-bc77-0dd14b26e0c3",
//     3   │   "title": "Chat 1712025755149",
//     4   │   "created_at": "2024-04-02T02:42:35.149Z",
//     5   │   "messages": [
//     6   │     {
//     7   │       "from": "user",
//     8   │       "text": "salem bro belahi bch nthabet sure switches red wela 5ater barch
//         │ a y7ot switch we howa switch a5er "
//     9   │     },

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
fn get_messages() -> Result<String, String> {
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
