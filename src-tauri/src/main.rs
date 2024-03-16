// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
// 定义config 结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub local_host: String,
    pub local_port: String,
    pub remote_host: String,
    pub remote_port: String,
    pub protocol: Vec<String>
}

#[tauri::command]
fn insert_record(data: &str) ->  String {
    println!("data: {}", data);
    let config:Config = serde_json::from_str(&data).unwrap();
    let json_str = serde_json::to_string(&config).unwrap();
    json_str
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            insert_record,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
