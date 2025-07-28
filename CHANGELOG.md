# Changelog

All notable changes to the **Fanyi** command-line translation tool will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-07-28

### Added
- 🎉 **初始版本发布** - 基于百度翻译API的命令行翻译工具
- 🌐 **多语言支持** - 支持26种语言互译（中文、英语、日语、韩语、法语、德语、俄语等）
- 🎯 **智能语言检测** - 自动识别输入文本的语言（`auto`模式）
- 🌈 **彩色输出** - 美观的颜色显示和格式化输出
- ⚡ **多种输入方式**：
  - 命令行参数：`fanyi "文本"`
  - 管道输入：`echo "文本" | fanyi`
  - 交互模式：直接运行`fanyi`后输入文本
- 🔧 **灵活的配置系统**：
  - TOML配置文件（`~/.config/fanyi/config.toml`）
  - 支持默认语言设置
  - API密钥安全存储
- 📋 **命令行界面**：
  - `--from` / `-f` 指定源语言
  - `--to` / `-t` 指定目标语言
  - `--no-color` 禁用彩色输出
  - `--help` / `-h` 显示帮助信息
- 🛠️ **子命令支持**：
  - `fanyi config` - 配置管理
  - `fanyi languages` - 查看支持的语言列表
- 🔐 **API集成** - 百度翻译API完整集成，包括MD5签名验证

### Added - 代理支持功能
- 🌍 **智能代理检测** - 自动检测环境变量中的代理设置
- 🔧 **灵活代理配置**：
  - 三种代理模式：`auto`（自动检测）、`enable`（强制启用）、`disable`（禁用）
  - 支持HTTP和HTTPS代理分别配置
  - 环境变量支持：`http_proxy`、`https_proxy`、`HTTP_PROXY`、`HTTPS_PROXY`
- 🎛️ **命令行代理控制**：
  - `--no-proxy` 临时禁用代理
  - `--force-proxy` 临时强制使用代理
- 📊 **代理状态查看** - `fanyi proxy-status` 命令显示详细代理信息
- ⚙️ **配置文件代理设置**：
  - `fanyi config --proxy-mode <mode>` 设置代理模式
  - `fanyi config --http-proxy <url>` 设置HTTP代理
  - `fanyi config --https-proxy <url>` 设置HTTPS代理

### Added - 安装和文档
- 📦 **智能安装脚本** (`install.sh`)：
  - 自动环境检测（Rust/Cargo）
  - 自动编译和安装到`~/.local/bin`
  - 智能PATH配置和检测
  - 支持自动添加PATH到shell配置文件
  - 完整的卸载功能
  - 彩色输出和用户友好提示
- 📖 **完整文档** (`README.md`)：
  - 详细的功能介绍和使用指南
  - 安装和配置教程
  - 代理配置详解
  - 故障排除指南
  - 丰富的使用示例
- ⚖️ **开源许可** - MIT License
- 📝 **变更日志** - 本文件，记录所有版本变更

### Technical Details
- 🦀 **Rust 2021** - 使用最新的Rust 2021版本
- 📦 **核心依赖**：
  - `clap 4.5` - 命令行参数解析
  - `reqwest 0.11` - HTTP客户端
  - `tokio 1.0` - 异步运行时
  - `serde 1.0` - 序列化/反序列化
  - `toml 0.8` - 配置文件格式
  - `colored 2.1` - 彩色输出
  - `anyhow 1.0` - 错误处理
  - `md5 0.7` - API签名生成
- 🏗️ **模块化架构**：
  - `config` - 配置管理模块
  - `cli` - 命令行参数处理模块
  - `translator` - 翻译器抽象和实现模块
  - `ui` - 用户界面和输出模块
- 🔄 **异步设计** - 全异步架构，支持高效的网络请求

### Fixed
- 🔧 **网络连接问题** - 解决TLS握手失败和代理干扰问题
- 📋 **配置文件兼容性** - 修复配置文件格式变更导致的解析错误
- 🎨 **输出格式** - 优化翻译结果的显示格式和颜色方案
- ⚠️ **错误处理** - 改进API错误信息的显示和用户提示

### Changed
- 🔒 **安全性提升** - API密钥在配置显示时进行脱敏处理
- 📊 **日志改进** - 增强错误信息的详细程度和可读性
- 🎯 **用户体验** - 优化命令行交互和提示信息
