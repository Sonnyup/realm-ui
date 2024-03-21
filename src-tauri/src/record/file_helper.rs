use std::{
    fs::File,
    io::{Read, Write},
};

pub struct FileHelper {
    file_path: String,
}

impl FileHelper {
    // 创建FileHelper实例
    pub fn new() -> Self {
        Self {
            file_path: String::from("records.json"),
        }
    }

    // 设置文件路径
    pub fn set_file_path(&mut self, file_path: String) {
        self.file_path = file_path;
    }

    // 打开文件并返回文件句柄
    pub fn open_file(&self) -> Result<File, std::io::Error> {
        File::open(&self.file_path)
    }

    // 读取文件内容，如果文件不存在则创建文件并返回[]字符串
    pub fn read_file_or_create(&self) -> Result<String, std::io::Error> {
        match File::open(&self.file_path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                // 判断contents是否为空字符串，如果为空字符串则返回[]字符串并写入到文件中
                if contents.is_empty() {
                    println!("File contents is empty");
                    file.write_all(b"[]")?;
                    Ok("[]".to_string())
                } else {
                    Ok(contents)
                }
            }
            Err(_) => {
                let mut file = File::create(&self.file_path)?;
                println!("File contents is empty, create new file");
                file.write_all(b"[]")?;
                Ok("[]".to_string())
            }
        }
    }

    // 清空文件内容并写入新内容
    pub fn write_file(&self, contents: String) -> Result<(), std::io::Error> {
        let mut file = File::create(&self.file_path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    // 读取文件内容
    pub fn read_file(&self) -> Result<String, std::io::Error> {
        let mut file = File::open(&self.file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
