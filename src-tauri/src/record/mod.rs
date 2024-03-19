pub mod record;

use record::Record;
use serde_json::{from_str, to_string, to_writer_pretty};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::net::IpAddr;
use std::os::windows::process::CommandExt;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

// 获取配置列表
#[tauri::command]
pub fn get_records() -> Result<String, String> {
    sleep(Duration::from_millis(200));
    // 获取文件内容
    let contents = get_file_contents().map_err(|err| err.to_string())?;
    println!("获取配置列表");
    Ok(contents)
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
fn get_file_contents() -> Result<String, String> {
    let filename = "records.json";

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        // .truncate(true)
        .open(filename)
        .map_err(|err| err.to_string())?;

    // let mut file: File = get_file_handle().map_err(|err|  err.to_string())?;

    // 读取文件内容到字符串中
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| err.to_string())?;

    // 如果文件内容为空，则返回空的 JSON 数组
    if contents.trim().is_empty() {
        return Ok(String::from("[]"));
    }

    Ok(contents)
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

    let pid = port_forward(&record).map_err(|err| err.to_string())?;
    Ok(pid)
}

// 开启端口转发
fn port_forward(record: &Record) -> Result<u32, String> {
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

// 初始化端口转发
pub fn init_ports() -> Result<(), String> {
    println!("init_ports");
    // 获取文件内容
    let contents = get_file_contents().map_err(|err| err.to_string())?;

    // 解析成数组
    let mut records: Vec<Record> = from_str(&contents).map_err(|err| err.to_string())?;

    // 开启端口转发
    for record in &mut records {
        if record.status > 0 {
            let pid = port_forward(&record).map_err(|err| err.to_string())?;
            println!("init_ports: {}", pid);
            record.status = pid;
        }
    }

    // 保存最新内容到文件中
    let json_str = to_string(&records).map_err(|err| err.to_string())?;
    let _ = save_record(&json_str);
    // println!("init_ports: {:?}", &records);
    Ok(())
}

use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

// 获取IP地址
pub fn get_ip_address(domain_name: &str) -> Result<IpAddr, String> {
    // 使用函数
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
        .map_err(|err| err.to_string())?;

    let response = resolver
        .lookup_ip(domain_name)
        .map_err(|err| err.to_string())?;

    let address = response.iter().next().expect("no addresses returned!");
    if address.is_ipv4() {
        println!("IPV4: {}", address);
        // assert_eq!(address, IpAddr::V4(Ipv4Addr::new(93, 184, 216, 34)));
    } else {
        println!("IPV6: {}", address);
    }
    Ok(address)
}