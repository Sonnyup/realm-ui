use crate::record::{Record, FileHelper};
use std::os::windows::process::CommandExt;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
use serde_json::to_string_pretty;


// 进程存储
#[derive(Debug)]
struct StorageProcess {
    pid: u32,
    process: Child,
}

// 进程存储
static mut STORAGEPROCESS: Vec<StorageProcess> = Vec::new();

pub struct Commands {}

impl Commands {
    pub fn new() -> Self {
        Self {}
    }

    // 开启端口转发
    pub fn forward(&self, record: &Record) -> Result<u32, String> {
        let remote_ip = self
            .get_ip_address(&record.remote_host)
            .map_err(|_| String::from("地址解析失败，请确认后重试！"))?;
        println!("remote_ip:{}", remote_ip);

        let local_host_port = format!("{}:{}", record.local_host, record.local_port);
        let remote_host_port = format!("{}:{}", remote_ip, record.remote_port);
        let protocol = &record.protocol;
        println!("open port: {} {}", local_host_port, remote_host_port);

        let mut command = Command::new("realm");
        command
            .creation_flags(0x08000000) // 隐藏CMD窗口
            .args(["-l", &local_host_port])
            .args(["-r", &remote_host_port]);

        if protocol.contains(&String::from("udp")) {
            command.args(["-u"]);
        }

        println!("command: {:?}", command);
        let mut child = command.spawn().map_err(|err| err.to_string())?;

        // 等待进程结果
        sleep(Duration::from_millis(300));
        match child.try_wait() {
            Ok(Some(_)) => return Ok(0),
            Ok(None) => {
                println!("进程启动中");
            }
            Err(e) => println!("error attempting to wait: {e}"),
        }

        let pid = child.id().clone();
        unsafe {
            STORAGEPROCESS.push(StorageProcess {
                pid: child.id(),
                process: child,
            });
        }
        Ok(pid)
    }

    // 关闭端口转发
    pub fn close_forward(&self, pid: u32) -> Result<bool, String> {
        unsafe {
            for (k, c) in STORAGEPROCESS.iter_mut().enumerate() {
                if c.pid == pid {
                    let _ = c.process.kill();
                    STORAGEPROCESS.remove(k);
                    break;
                }
            }
            sleep(Duration::from_millis(200));
        }

        Ok(true)
    }

    // 清空进程
    pub fn close_forward_all(&self) {
        unsafe {
            for c in &mut STORAGEPROCESS {
                let _ = c.process.kill();
            }
        }
    }

    // 初始化端口转发
    pub fn init_forward(&self) -> Result<(), String> {
        println!("初始化端口转发");

        // 读取文件内容
        let contents = match FileHelper::new().read_file_or_create() {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("读取配置文件失败: {}", err);
                return Err(err.to_string());
            }
        };

        // 解析成数组
        let mut records = match Record::records_from_str(&contents) {
            Ok(records) => records,
            Err(err) => {
                eprintln!("配置内容解析失败: {}", err);
                return Err(err.to_string());
            }
        };

        // 开启端口转发
        for record in &mut records {
            if record.pid > 0 {
                let pid = self.forward(&record).map_err(|err| err.to_string())?;
                println!("开启端口转发-进程ID: {}", pid);
                record.pid = pid;
            }
        }

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

        Ok(())
    }

    // 获取IP地址
    pub fn get_ip_address(&self, domain_name: &str) -> Result<String, String> {
        // 使用函数
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
            .map_err(|err| err.to_string())?;

        let response = resolver
            .lookup_ip(domain_name)
            .map_err(|err| err.to_string())?;

        let address = response.iter().next().expect("no addresses returned!");
        Ok(address.to_string())
    }
}
