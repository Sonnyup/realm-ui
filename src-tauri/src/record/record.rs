use serde::{Deserialize, Serialize};

// 定义Record 结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub local_host: String,
    pub local_port: u32,
    pub remote_host: String,
    pub remote_port: u32,
    pub protocol: Vec<String>,
    pub status: bool,
}

impl Record {
    
    
}