pub mod record;

use record::Record;
use serde_json::{from_str, to_string, to_writer_pretty};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

#[tauri::command]
pub fn insert_record(data: &str) -> Result<String, String> {
    println!("data: {}", data);

    let mut file: File = get_file_handle().map_err(|err| err.to_string())?;
    // 读取文件内容到字符串中
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| err.to_string())?;

    let mut records: Vec<Record> = Vec::new();
    println!("contents: {:#?}", contents);
    // 如果文件内容为空
    if !contents.trim().is_empty() {
        records = from_str(&contents).map_err(|err| err.to_string())?;
    }

    println!("records: {:#?}", records);
    let record: Record = from_str(&data).map_err(|err| err.to_string())?;
    records.push(record);

    let file = File::options()
        .write(true)
        .truncate(true)
        .open("records.json")
        .map_err(|err| {
            eprintln!("Error getting file handle: {}", err);
            err.to_string()
        })?;
    to_writer_pretty(file, &records).map_err(|err| err.to_string())?;

    let json_str = to_string(&records).map_err(|err| err.to_string())?;

    Ok(json_str)
}

#[tauri::command]
pub fn get_records() -> Result<String, String> {
    let mut file: File = get_file_handle().map_err(|err| {
        eprintln!("Error getting file handle: {}", err);
        err.to_string()
    })?;

    // 读取文件内容到字符串中
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|err| {
        eprintln!("Error reading file contents: {}", err);
        err.to_string()
    })?;

    // 如果文件内容为空，则返回空的 JSON 数组
    if contents.trim().is_empty() {
        return Ok(String::from("[]"));
    }

    let records: Vec<Record> = from_str(&contents).map_err(|err| err.to_string())?;

    let json_str = to_string(&records).unwrap();
    Ok(json_str)
}

// 保存配置
#[tauri::command]
pub fn save_record(data: &str) -> Result<String, String> {
    let records: Vec<Record> = from_str(&data).map_err(|err| err.to_string())?;

    let file = File::options()
        .write(true)
        .truncate(true)
        .open("records.json")
        .map_err(|err| {
            eprintln!("Error getting file handle: {}", err);
            err.to_string()
        })?;
    to_writer_pretty(file, &records).map_err(|err| err.to_string())?;

    let json_str = to_string(&records).map_err(|err| err.to_string())?;

    Ok(json_str)
}

// 读取文件
fn get_file_handle() -> Result<File, Box<dyn std::error::Error>> {
    let filename = "records.json";

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        // .truncate(true)
        .open(filename)?;

    Ok(file)
}

#[derive(Debug)]
struct Childs {
    pid: u32,
    child: Child,
}

// 进程存储
static mut CHILDS: Vec<Childs> = Vec::new();

// 开启端口转发
#[tauri::command]
pub fn open_port(data: &str) -> Result<u32, String> {
    let record: Record = serde_json::from_str(&data).map_err(|err| err.to_string())?;

    let local_host_port = format!("{}:{}", record.local_host, record.local_port);
    let remote_host_port = format!("{}:{}", record.remote_host, record.remote_port);

    println!("open port: {} {}", local_host_port, remote_host_port);

    let mut command = Command::new("realm");
    command
        .creation_flags(0x08000000) // 隐藏CMD窗口
        .args(["-l", &local_host_port])
        .args(["-r", &remote_host_port]);

    let mut child = command.spawn().map_err(|err| err.to_string())?;

    // 等待进程结果
    sleep(Duration::from_millis(200));
    match child.try_wait() {
        Ok(Some(_)) => return Ok(0),
        Ok(None) => {
            println!("进程启动中");
        }
        Err(e) => println!("error attempting to wait: {e}"),
    }

    let pid = child.id().clone();
    unsafe {
        CHILDS.push(Childs {
            pid: child.id(),
            child: child,
        });
    }

    Ok(pid)
}

// 关闭进程转发
#[tauri::command]
pub fn close_port(pid: u32) -> Result<bool, String> {
    unsafe {
        for (k, c) in CHILDS.iter_mut().enumerate() {
            if c.pid == pid {
                let _ = c.child.kill();
                CHILDS.remove(k);
                break;
            }
        }
        sleep(Duration::from_millis(200));
    }

    Ok(true)
}

// 清空进程
pub fn close_all() {
    unsafe {
        for c in &mut CHILDS {
            let _ = c.child.kill();
        }
    }
}
