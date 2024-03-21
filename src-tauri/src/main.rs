// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use realm_ui::record;
use realm_ui::record::commands::Commands;

fn main() {

    let app = tauri::Builder::default()
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    //阻止默认关闭
                    api.prevent_close();

                    // 关闭全部端口转发进程
                    Commands::new().close_forward_all();
                    let window = event.window().clone();
                    let _ = window.close();
                    println!("关闭窗口");
                }
                _ => {} //todo
            }
        })
        .invoke_handler(tauri::generate_handler![
            record::get_records,
            record::save_record,
            record::open_port,
            record::close_port,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::Ready => {
            // 初始化端口转发
            match Commands::new().init_forward() {
                Ok(_) => {
                    println!("端口转发初始化成功");
                }
                Err(e) => {
                    println!("端口转发初始化失败：{}", e);
                }
            }
        }
        _ => {}
    });
}
