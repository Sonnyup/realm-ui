// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use realm_ui::record;

fn main() {
    let app = tauri::Builder::default()
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
            let _ = record::init_ports();

            // 获取IP地址
            let a = record::get_ip_address("www.baidu.com");
            println!("a: {:?}", &a);
        }
        _ => {}
    });
}
