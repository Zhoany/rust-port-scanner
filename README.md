# Rust Port Scanner

## 简介
这个项目是一个使用Rust语言编写的多线程端口扫描器。作为一个学习rust基础的工具编写，帮助理解Rust中的网络编程、多线程处理和命令行参数解析。

## 功能
- **多线程扫描**: 使用多线程并发扫描端口，提高扫描效率。
- **命令行界面**: 通过命令行参数指定目标IP地址和线程数。
- **进度显示**: 实时显示扫描进度条。
- **端口检测**: 检测和列出目标IP地址上开放的端口。

## 安装和运行
要运行这个端口扫描器，你需要先安装Rust编程环境。可以从[Rust官网](https://www.rust-lang.org/)获取安装指南。

### 克隆与编译
```bash
git clone https://github.com/Zhoany/rust-port-scanner.git
cd rust-port-scanner
cargo build --release
```

### 使用方法
```bash
rust-port-scanner [-j 线程数] [IP地址]
```
### 示例
```bash
rust-port-scanner -j 2000 192.168.1.1
```
