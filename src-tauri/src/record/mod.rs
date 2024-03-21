pub mod commands;
pub mod file_helper;
pub mod record;

use commands::Commands as NewCommand;
use file_helper::FileHelper;
use record::Record;
use serde_json::to_string_pretty;
use std::thread::sleep;
use std::time::Duration;

// 获取配置列表
#[tauri::command]
pub fn get_records() -> Result<String, String> {
    // 延时
    sleep(Duration::from_millis(200));

    // 读取文件内容
    match FileHelper::new().read_file_or_create() {
        Ok(contents) => Ok(contents),
        Err(err) => {
            eprintln!("读取配置文件失败: {}", err);
            return Err(err.to_string());
        }
    }
}

// 保存配置
#[tauri::command]
pub fn save_record(json_str: &str) -> Result<String, String> {
    // 解析json字符串
    let records = match Record::records_from_str(&json_str) {
        Ok(records) => records,
        Err(err) => {
            eprintln!("配置内容解析失败: {}", err);
            return Err(err.to_string());
        }
    };

    //美化json字符串
    let json_str = to_string_pretty(&records).map_err(|err| {
        eprintln!("配置格式化失败: {}", err);
        err.to_string()
    })?;

    // 保存到文件
    FileHelper::new()
        .write_file(json_str.to_string())
        .map_err(|err| {
            eprintln!("配置文件报错失败: {}", err);
            err.to_string()
        })?;

    // 返回json字符串
    Ok(json_str.to_string())
}

// 开启端口转发
#[tauri::command]
pub fn open_port(json_str: &str) -> Result<u32, String> {
    let record = match Record::from_str(&json_str) {
        Ok(record) => record,
        Err(err) => {
            eprintln!("配置内容解析失败: {}", err);
            return Err(err.to_string());
        }
    };

    let pid = NewCommand::new()
        .forward(&record)
        .map_err(|err| err.to_string())?;
    Ok(pid)
}

// 关闭进程转发
#[tauri::command]
pub fn close_port(pid: u32) -> Result<bool, String> {
    NewCommand::new()
        .close_forward(pid)
        .map_err(|err| err.to_string())
}

