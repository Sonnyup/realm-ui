use serde::{Deserialize, Serialize};
use serde_json::{to_string, from_str, Error};

// 定义Record 结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub local_host: String,
    pub local_port: u32,
    pub remote_host: String,
    pub remote_port: u32,
    pub protocol: Vec<String>,
    pub pid: u32,
}

impl Record {

    // 用于将Record结构体转换为json字符串，并处理错误
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        to_string(self)
    }

    // 用于从json字符串中解析Record结构体，并处理错误
    pub fn from_str(json_str: &str) -> Result<Record, Error> {
        from_str(json_str)
    }

    // Records结构体，转换为json字符串，并处理错误
    pub fn records_to_json(records: &Vec<Record>) -> Result<String, Error> {
        to_string(records)
    }
    
    // 从json字符串中解析Records结构体，并处理错误
    pub fn records_from_str(json_str: &str) -> Result<Vec<Record>, Error> {
        from_str(json_str)
    }

}