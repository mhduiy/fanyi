use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fanyi")]
#[command(about = "一个简单实用的命令行翻译工具")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// 源语言 (例如: zh, en, ja)
    #[arg(short, long)]
    pub from: Option<String>,
    
    /// 目标语言 (例如: zh, en, ja)
    #[arg(short, long)]
    pub to: Option<String>,
    
    /// 要翻译的文本
    #[arg(value_name = "TEXT")]
    pub text: Option<String>,
    
    /// 禁用颜色输出
    #[arg(long)]
    pub no_color: bool,
    
    /// 禁用代理 (忽略环境变量和配置文件中的代理设置)
    #[arg(long)]
    pub no_proxy: bool,
    
    /// 强制使用代理 (即使配置为禁用)
    #[arg(long)]
    pub force_proxy: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 配置API密钥和默认设置
    Config {
        /// 百度翻译APP ID
        #[arg(long)]
        app_id: Option<String>,
        
        /// 百度翻译密钥
        #[arg(long)]
        secret_key: Option<String>,
        
        /// 默认源语言
        #[arg(long)]
        default_from: Option<String>,
        
        /// 默认目标语言
        #[arg(long)]
        default_to: Option<String>,
        
        /// 代理模式: auto, enable, disable
        #[arg(long, value_parser = parse_proxy_mode)]
        proxy_mode: Option<String>,
        
        /// HTTP代理地址 (例如: http://proxy.example.com:8080)
        #[arg(long)]
        http_proxy: Option<String>,
        
        /// HTTPS代理地址 (例如: http://proxy.example.com:8080)
        #[arg(long)]
        https_proxy: Option<String>,
        
        /// 显示当前配置
        #[arg(long)]
        show: bool,
    },
    /// 列出支持的语言
    Languages,
    /// 显示代理状态和环境变量
    ProxyStatus,
}

fn parse_proxy_mode(s: &str) -> Result<String, String> {
    match s.to_lowercase().as_str() {
        "auto" | "enable" | "disable" => Ok(s.to_lowercase()),
        _ => Err("代理模式必须是 auto, enable 或 disable 之一".to_string()),
    }
}

impl Cli {
    /// 获取源语言，优先使用命令行参数，否则使用配置文件默认值
    pub fn get_from_language(&self, default: &str) -> String {
        self.from.clone().unwrap_or_else(|| default.to_string())
    }
    
    /// 获取目标语言，优先使用命令行参数，否则使用配置文件默认值
    pub fn get_to_language(&self, default: &str) -> String {
        self.to.clone().unwrap_or_else(|| default.to_string())
    }
} 