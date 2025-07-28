use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub translation: TranslationConfig,
    pub baidu: BaiduConfig,
    pub ui: UiConfig,
    pub proxy: ProxyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationConfig {
    pub default_from: String,
    pub default_to: String,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaiduConfig {
    pub app_id: String,
    pub secret_key: String,
    pub api_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub enable_colors: bool,
    pub show_language_detection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// 是否使用代理 - auto: 自动检测环境变量, true: 强制使用, false: 禁用代理
    pub enabled: ProxyMode,
    /// HTTP 代理地址 (如果为空则从环境变量读取)
    pub http_proxy: Option<String>,
    /// HTTPS 代理地址 (如果为空则从环境变量读取)
    pub https_proxy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProxyMode {
    Auto,   // 自动检测环境变量
    Enable, // 强制启用
    Disable,// 强制禁用
}

impl Default for BaiduConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            secret_key: String::new(),
            api_url: "https://fanyi-api.baidu.com/api/trans/vip/translate".to_string(),
        }
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            enabled: ProxyMode::Auto,
            http_proxy: None,
            https_proxy: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            translation: TranslationConfig {
                default_from: "zh".to_string(),
                default_to: "en".to_string(),
                provider: "baidu".to_string(),
            },
            baidu: BaiduConfig::default(),
            ui: UiConfig {
                enable_colors: true,
                show_language_detection: true,
            },
            proxy: ProxyConfig::default(),
        }
    }
}

impl ProxyConfig {
    /// 检查环境变量获取代理设置
    pub fn get_proxy_from_env() -> (Option<String>, Option<String>) {
        let http_proxy = std::env::var("http_proxy")
            .or_else(|_| std::env::var("HTTP_PROXY"))
            .ok();
        
        let https_proxy = std::env::var("https_proxy")
            .or_else(|_| std::env::var("HTTPS_PROXY"))
            .ok();
        
        (http_proxy, https_proxy)
    }
    
    /// 获取实际应该使用的代理设置
    pub fn get_effective_proxy(&self) -> (Option<String>, Option<String>) {
        match self.enabled {
            ProxyMode::Disable => (None, None),
            ProxyMode::Enable => {
                // 优先使用配置文件中的代理，如果没有则从环境变量获取
                let http = self.http_proxy.clone()
                    .or_else(|| Self::get_proxy_from_env().0);
                let https = self.https_proxy.clone()
                    .or_else(|| Self::get_proxy_from_env().1);
                (http, https)
            },
            ProxyMode::Auto => {
                // 自动模式：优先配置文件，然后环境变量
                let (env_http, env_https) = Self::get_proxy_from_env();
                let http = self.http_proxy.clone().or(env_http);
                let https = self.https_proxy.clone().or(env_https);
                (http, https)
            }
        }
    }
}

impl Config {
    /// 获取配置文件路径
    pub fn config_file_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("无法获取配置目录")?
            .join("fanyi");
        
        // 确保配置目录存在
        fs::create_dir_all(&config_dir)
            .context("创建配置目录失败")?;
        
        Ok(config_dir.join("config.toml"))
    }

    /// 加载配置文件
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .context("读取配置文件失败")?;
            
            let config: Config = toml::from_str(&content)
                .context("解析配置文件失败")?;
            
            Ok(config)
        } else {
            // 如果配置文件不存在，创建默认配置
            let default_config = Config::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    /// 保存配置文件
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;
        let content = toml::to_string_pretty(self)
            .context("序列化配置失败")?;
        
        fs::write(&config_path, content)
            .context("写入配置文件失败")?;
        
        Ok(())
    }

    /// 验证配置是否完整
    pub fn validate(&self) -> Result<()> {
        if self.baidu.app_id.is_empty() || self.baidu.secret_key.is_empty() {
            anyhow::bail!("百度翻译API密钥未配置，请运行 'fanyi config' 进行配置");
        }
        Ok(())
    }
} 