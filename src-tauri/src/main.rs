// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("{}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use encoding_rs::GBK;
use encoding_rs::UTF_8;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
#[tauri::command]
fn run(name: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .creation_flags(0x08000000) // 设置窗口隐藏
            .arg("/c")
            .arg(name)
            .stdout(Stdio::piped())
            .output()
            .expect("cmd exec error!")
    } else {
        Command::new("sh")
            .creation_flags(0x08000000) // 设置窗口隐藏
            .arg("-c")
            .arg(name)
            .stdout(Stdio::piped())
            .output()
            .expect("sh exec error!")
    };

    return if output.status.success() {
        let (result, _, had_err) = UTF_8.decode(&output.stdout,);

        if !had_err {
            return result.to_string();
        }
        let (result, _, had_err) = GBK.decode(&output.stdout);
        if !had_err {
            return result.to_string();
        }
        format!("{}", String::from_utf8_lossy(&output.stdout))
    } else {
        // 如果命令失败，打印 stderr
        let (result, _, had_err) = UTF_8.decode(&output.stderr);

        if !had_err {
            return format!("{}", result.to_string());
        }
        let (result, _, had_err) = GBK.decode(&output.stderr);
        if !had_err {
            return format!("{}", result.to_string());
        }
        format!("{}", String::from_utf8_lossy(&output.stderr))
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
