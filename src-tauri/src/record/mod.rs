pub mod record;

use record::Record;
use std::fs::File;
use std::path::Path;
use serde_json::{ from_reader, to_string };

#[tauri::command]
pub fn insert_record(data: &str) -> Result<String, String> {
    println!("data: {}", data);
    let record:Record = serde_json::from_str(&data).map_err(|err| err.to_string())?;
    let json_str = serde_json::to_string(&record).map_err(|err| err.to_string())?;
    Ok(json_str)
}

#[tauri::command]
pub fn get_records() -> Result<String, String> {
    let filename ="records.json";

    if !Path::new(filename).exists() {
        File::create(filename).unwrap();
        return Ok(String::from("[]"));
    }

    let file = File::open(filename).map_err(|err| err.to_string())?;
    let records: Vec<Record> = from_reader(file).map_err(|err| err.to_string())?;

    let json_str = to_string(&records).unwrap();
    Ok(json_str)
}