mod config;
mod cli;
mod translator;
mod ui;

use anyhow::Result;
use clap::Parser;
use std::io::{self, IsTerminal};

use cli::{Cli, Commands};
use config::{Config, ProxyMode, ProxyConfig};
use translator::{BaiduTranslator, Translator};
use ui::{display_translation, display_error, display_success, display_info, display_warning};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("程序运行出错: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // 处理子命令
    match &cli.command {
        Some(Commands::Config {
            app_id,
            secret_key,
            default_from,
            default_to,
            proxy_mode,
            http_proxy,
            https_proxy,
            show,
        }) => {
            return handle_config_command(app_id, secret_key, default_from, default_to, 
                                       proxy_mode, http_proxy, https_proxy, *show);
        }
        Some(Commands::Languages) => {
            return handle_languages_command();
        }
        Some(Commands::ProxyStatus) => {
            return handle_proxy_status_command();
        }
        None => {}
    }

    // 加载配置
    let mut config = Config::load()?;
    let enable_colors = config.ui.enable_colors && !cli.no_color;

    // 处理命令行代理覆盖
    if cli.no_proxy {
        config.proxy.enabled = ProxyMode::Disable;
    } else if cli.force_proxy {
        config.proxy.enabled = ProxyMode::Enable;
    }

    // 如果禁用代理，清除代理相关环境变量，防止 reqwest 自动读取
    if config.proxy.enabled == ProxyMode::Disable {
        for key in &["http_proxy", "https_proxy", "HTTP_PROXY", "HTTPS_PROXY"] {
            std::env::remove_var(key);
        }
    }

    // 验证配置
    if let Err(e) = config.validate() {
        display_error(&e.to_string(), enable_colors);
        display_info("请使用 'fanyi config --app-id YOUR_APP_ID --secret-key YOUR_SECRET_KEY' 配置API密钥", enable_colors);
        std::process::exit(1);
    }

    // 获取要翻译的文本
    let text = get_translation_text(&cli).await?;
    if text.trim().is_empty() {
        display_error("要翻译的文本不能为空", enable_colors);
        std::process::exit(1);
    }

    // 获取源语言和目标语言
    let from_lang = cli.get_from_language(&config.translation.default_from);
    let to_lang = cli.get_to_language(&config.translation.default_to);

    // 创建翻译器（传递代理配置）
    let translator = BaiduTranslator::new(config.baidu.clone(), &config.proxy);

    // 执行翻译
    match translator.translate(&text, &from_lang, &to_lang).await {
        Ok(result) => {
            display_translation(&result, enable_colors, config.ui.show_language_detection);
        }
        Err(e) => {
            display_error(&e.to_string(), enable_colors);
            display_info("如果是网络错误，请检查代理设置或使用 'fanyi proxy-status' 查看代理状态", enable_colors);
            std::process::exit(1);
        }
    }

    Ok(())
}

/// 获取要翻译的文本
async fn get_translation_text(cli: &Cli) -> Result<String> {
    if let Some(text) = &cli.text {
        // 如果命令行提供了文本，直接使用
        Ok(text.clone())
    } else if !IsTerminal::is_terminal(&io::stdin()) {
        // 如果标准输入有管道数据，读取标准输入
        use std::io::Read;
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer.trim().to_string())
    } else {
        // 否则进入交互模式
        println!("请输入要翻译的文本 (按 Ctrl+C 退出):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}

/// 处理配置命令
fn handle_config_command(
    app_id: &Option<String>,
    secret_key: &Option<String>,
    default_from: &Option<String>,
    default_to: &Option<String>,
    proxy_mode: &Option<String>,
    http_proxy: &Option<String>,
    https_proxy: &Option<String>,
    show: bool,
) -> Result<()> {
    let mut config = Config::load().unwrap_or_default();
    let enable_colors = config.ui.enable_colors;

    if show {
        // 显示当前配置
        display_info("当前配置:", enable_colors);
        println!("  百度翻译APP ID: {}", 
            if config.baidu.app_id.is_empty() { 
                "未设置".to_string() 
            } else { 
                mask_string(&config.baidu.app_id, 4) 
            }
        );
        println!("  百度翻译密钥: {}", 
            if config.baidu.secret_key.is_empty() { 
                "未设置".to_string() 
            } else { 
                mask_string(&config.baidu.secret_key, 4) 
            }
        );
        println!("  默认源语言: {}", config.translation.default_from);
        println!("  默认目标语言: {}", config.translation.default_to);
        println!("  颜色输出: {}", if config.ui.enable_colors { "启用" } else { "禁用" });
        
        // 显示代理配置
        println!("  代理模式: {:?}", config.proxy.enabled);
        println!("  HTTP代理: {}", 
            config.proxy.http_proxy.as_deref().unwrap_or("未设置")
        );
        println!("  HTTPS代理: {}", 
            config.proxy.https_proxy.as_deref().unwrap_or("未设置")
        );
        
        return Ok(());
    }

    let mut updated = false;

    if let Some(id) = app_id {
        config.baidu.app_id = id.clone();
        updated = true;
    }

    if let Some(key) = secret_key {
        config.baidu.secret_key = key.clone();
        updated = true;
    }

    if let Some(from) = default_from {
        config.translation.default_from = from.clone();
        updated = true;
    }

    if let Some(to) = default_to {
        config.translation.default_to = to.clone();
        updated = true;
    }

    // 处理代理配置
    if let Some(mode) = proxy_mode {
        config.proxy.enabled = match mode.as_str() {
            "auto" => ProxyMode::Auto,
            "enable" => ProxyMode::Enable,
            "disable" => ProxyMode::Disable,
            _ => unreachable!(), // 已经在clap中验证过了
        };
        updated = true;
    }

    if let Some(proxy) = http_proxy {
        config.proxy.http_proxy = if proxy.is_empty() { None } else { Some(proxy.clone()) };
        updated = true;
    }

    if let Some(proxy) = https_proxy {
        config.proxy.https_proxy = if proxy.is_empty() { None } else { Some(proxy.clone()) };
        updated = true;
    }

    if updated {
        config.save()?;
        display_success("配置已更新", enable_colors);
        
        // 提示配置文件位置
        if let Ok(config_path) = Config::config_file_path() {
            display_info(&format!("配置文件位置: {}", config_path.display()), enable_colors);
        }
    } else {
        display_warning("没有指定要更新的配置项", enable_colors);
        display_info("使用 'fanyi config --help' 查看可用选项", enable_colors);
    }

    Ok(())
}

/// 处理languages命令
fn handle_languages_command() -> Result<()> {
    let translator = BaiduTranslator::new(config::BaiduConfig::default(), &ProxyConfig::default());
    let languages = translator.supported_languages();
    
    println!("支持的语言列表:");
    println!("{:<8} {}", "代码", "语言");
    println!("{}", "-".repeat(20));
    
    for (code, name) in languages {
        println!("{:<8} {}", code, name);
    }

    Ok(())
}

/// 处理代理状态命令
fn handle_proxy_status_command() -> Result<()> {
    let config = Config::load().unwrap_or_default();
    
    println!("=== 代理状态 ===");
    println!("配置的代理模式: {:?}", config.proxy.enabled);
    
    let (http_proxy, https_proxy) = config.proxy.get_effective_proxy();
    println!("当前生效的HTTP代理: {}", 
        http_proxy.as_deref().unwrap_or("无")
    );
    println!("当前生效的HTTPS代理: {}", 
        https_proxy.as_deref().unwrap_or("无")
    );
    
    println!("\n=== 环境变量 ===");
    let (env_http, env_https) = ProxyConfig::get_proxy_from_env();
    println!("http_proxy/HTTP_PROXY: {}", 
        env_http.as_deref().unwrap_or("未设置")
    );
    println!("https_proxy/HTTPS_PROXY: {}", 
        env_https.as_deref().unwrap_or("未设置")
    );
    
    println!("\n=== 配置文件中的代理设置 ===");
    println!("HTTP代理: {}", 
        config.proxy.http_proxy.as_deref().unwrap_or("未设置")
    );
    println!("HTTPS代理: {}", 
        config.proxy.https_proxy.as_deref().unwrap_or("未设置")
    );

    Ok(())
}

/// 掩码字符串，只显示前几个字符
fn mask_string(s: &str, show_chars: usize) -> String {
    if s.len() <= show_chars {
        "*".repeat(s.len())
    } else {
        format!("{}***", &s[..show_chars])
    }
}