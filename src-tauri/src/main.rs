// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

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

#[tauri::command]
fn get_messages() -> Result<String, String> {
    let path = "/tmp/bargain";
    // read chats from the path directory which are files that contain messages
    // structured like this and they are json
    // {
    //     "id": "69",
    //     "messages": [
    //         {
    //             "from": "farouk",
    //             "text": "hello this is me faroukkkkkkkkkk"
    //         },
    //         {
    //             "from": "ai",
    //             "text": "hello this is me the ai hahahah cool"
    //         },
    //         {
    //             "from": "farouk",
    //             "text": "hello this is me faroukkkkkkkkkk"
    //         },
    //         {
    //             "from": "ai",
    //             "text": "hello this is me the ai hahahah cool"
    //         },
    //         {
    //             "from": "farouk",
    //             "text": "hello this is me faroukkkkkkkkkk"
    //         },
    //         {
    //             "from": "ai",
    //             "text": "hello this is me the ai hahahah cool"
    //         },
    //         {
    //             "from": "farouk",
    //             "text": "hello this is me faroukkkkkkkkkk"
    //         },
    //         {
    //             "from": "ai",
    //             "text": "hello this is me the ai hahahah cool"
    //         },
    //         {
    //             "from": "farouk",
    //             "text": "hello this is me faroukkkkkkkkkk"
    //         },
    //         {
    //             "from": "ai",
    //             "text": "hello this is me the ai hahahah cool"
    //         },
    //     ]
    // }
    let mut messages = vec![];
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let content = fs::read_to_string(path).unwrap();
        messages.push(content);
    }
    Ok(messages.join("\n"))
}
