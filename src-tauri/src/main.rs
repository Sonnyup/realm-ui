// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use realm_ui::record;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    //阻止默认关闭
                    api.prevent_close();
                    // ....
                    // 进行你的操作
                    // ....
                    // 关闭所有的进程

                    record::close_all();
                    let window = event.window().clone();
                    let _ = window.close();
                    println!("关闭窗口");
                }
                _ => {} //todo
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            record::insert_record,
            record::get_records,
            record::save_record,
            record::open_port,
            record::close_port,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
