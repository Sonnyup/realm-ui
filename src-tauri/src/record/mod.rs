pub mod record;

use record::Record;
use serde_json::{from_reader, from_str, to_string, to_writer_pretty};
use std::fs::{File, OpenOptions};
use std::io::Read;

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

    let mut file = File::options()
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

    // let records: Vec<Record> = from_reader(&file).map_err(|err| {
    //     eprintln!("Error getting file from_reader: {}", err);
    //     err.to_string()
    // })?;

    let records: Vec<Record> = from_str(&contents).map_err(|err| err.to_string())?;

    let json_str = to_string(&records).unwrap();
    Ok(json_str)
}

fn get_file_handle() -> Result<File, Box<dyn std::error::Error>> {
    let filename = "records.json";

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        // .truncate(true)
        .open(filename)?;

    Ok(file)

    // 重新打开文件以获取新的句柄，因为之前的句柄可能处于不可预测的状态
    // let mut binding = OpenOptions::new();
    // let new_options = binding.read(true).write(true);
    // let new_file = new_options.open(filename)?;

    // Ok(new_file)
    // let file = if Path::new(filename).exists() {
    //     // 如果文件存在，直接打开它
    //     File::open(filename).map_err(|err| err.to_string())
    // } else {
    //     // 如果文件不存在，尝试创建它并打开
    //     let mut file = OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .open(filename)
    //         .map_err(|err| err.to_string());

    //     file.write_all(b"[]");
    //     file
    // };

    // file
}
