use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::{BaiduConfig, ProxyConfig};
use crate::translator::trait_def::{Translator, TranslationResult};

/// 百度翻译API响应结构
#[derive(Debug, Deserialize)]
struct BaiduResponse {
    from: String,
    to: String,
    trans_result: Vec<TransResult>,
    #[serde(default)]
    error_code: Option<String>,
    #[serde(default)]
    error_msg: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TransResult {
    src: String,
    dst: String,
}

/// 百度翻译器
pub struct BaiduTranslator {
    config: BaiduConfig,
    client: reqwest::Client,
}

impl BaiduTranslator {
    /// 创建新的百度翻译器实例
    pub fn new(config: BaiduConfig, proxy_config: &ProxyConfig) -> Self {
        let mut client_builder = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30));

        // 根据代理配置设置HTTP客户端
        let (http_proxy, https_proxy) = proxy_config.get_effective_proxy();
        
        if let Some(http_proxy_url) = http_proxy {
            if let Ok(proxy) = reqwest::Proxy::http(&http_proxy_url) {
                client_builder = client_builder.proxy(proxy);
                eprintln!("使用HTTP代理: {}", http_proxy_url);
            } else {
                eprintln!("警告: 无效的HTTP代理地址: {}", http_proxy_url);
            }
        }
        
        if let Some(https_proxy_url) = https_proxy {
            if let Ok(proxy) = reqwest::Proxy::https(&https_proxy_url) {
                client_builder = client_builder.proxy(proxy);
                eprintln!("使用HTTPS代理: {}", https_proxy_url);
            } else {
                eprintln!("警告: 无效的HTTPS代理地址: {}", https_proxy_url);
            }
        }
        
        let client = client_builder
            .build()
            .expect("创建HTTP客户端失败");
        
        Self { config, client }
    }

    /// 生成百度翻译API签名
    fn generate_sign(&self, query: &str, salt: &str) -> String {
        let sign_str = format!("{}{}{}{}", self.config.app_id, query, salt, self.config.secret_key);
        format!("{:x}", md5::compute(sign_str.as_bytes()))
    }

    /// 生成随机salt
    fn generate_salt() -> String {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string()
    }

    /// 语言代码映射，将常用语言代码转换为百度API支持的格式
    fn map_language_code(&self, lang: &str) -> String {
        match lang.to_lowercase().as_str() {
            "zh" | "zh-cn" | "chinese" => "zh".to_string(),
            "en" | "english" => "en".to_string(),
            "ja" | "jp" | "japanese" => "jp".to_string(),
            "ko" | "kr" | "korean" => "kor".to_string(),
            "fr" | "french" => "fra".to_string(),
            "es" | "spanish" => "spa".to_string(),
            "de" | "german" => "de".to_string(),
            "ru" | "russian" => "ru".to_string(),
            "auto" => "auto".to_string(),
            _ => lang.to_string(),
        }
    }

    /// 处理百度API错误
    fn handle_api_error(&self, error_code: &str) -> String {
        match error_code {
            "52001" => "请求超时，请重试".to_string(),
            "52002" => "系统错误，请重试".to_string(),
            "52003" => "未授权用户，请检查您的APP ID".to_string(),
            "52004" => "必填参数为空，请检查参数".to_string(),
            "52005" => "签名错误，请检查您的密钥".to_string(),
            "54000" => "必填参数为空".to_string(),
            "54001" => "签名错误".to_string(),
            "54003" => "访问频率受限，请降低请求频率".to_string(),
            "54004" => "账户余额不足".to_string(),
            "54005" => "长query请求频繁".to_string(),
            "58000" => "客户端IP非法".to_string(),
            "58001" => "译文语言方向不支持".to_string(),
            "58002" => "服务当前已关闭".to_string(),
            "90107" => "认证未通过或未生效".to_string(),
            _ => format!("未知错误: {}", error_code),
        }
    }
}

#[async_trait]
impl Translator for BaiduTranslator {
    async fn translate(&self, text: &str, from: &str, to: &str) -> Result<TranslationResult> {
        if text.trim().is_empty() {
            anyhow::bail!("翻译文本不能为空");
        }

        let from_lang = self.map_language_code(from);
        let to_lang = self.map_language_code(to);
        let salt = Self::generate_salt();
        let sign = self.generate_sign(text, &salt);

        // 构建请求参数
        let mut params = HashMap::new();
        params.insert("q", text);
        params.insert("from", &from_lang);
        params.insert("to", &to_lang);
        params.insert("appid", &self.config.app_id);
        params.insert("salt", &salt);
        params.insert("sign", &sign);

        // 发送请求
        let response = self
            .client
            .post(&self.config.api_url)
            .form(&params)
            .send()
            .await
            .with_context(|| {
                format!(
                    "发送翻译请求失败。URL: {}，请检查网络连接、代理设置和API配置", 
                    self.config.api_url
                )
            })?;

        let status = response.status();
        let body = response.text().await.context("读取响应失败")?;

        if !status.is_success() {
            anyhow::bail!("HTTP请求失败: {} - {}", status, body);
        }

        // 解析响应
        let baidu_response: BaiduResponse = serde_json::from_str(&body)
            .with_context(|| format!("解析API响应失败，响应内容: {}", body))?;

        // 检查API错误
        if let Some(error_code) = &baidu_response.error_code {
            let error_msg = self.handle_api_error(error_code);
            anyhow::bail!("百度翻译API错误 ({}): {}", error_code, error_msg);
        }

        if baidu_response.trans_result.is_empty() {
            anyhow::bail!("翻译结果为空");
        }

        // 提取翻译结果
        let trans_result = &baidu_response.trans_result[0];
        let detected_language = if from == "auto" && baidu_response.from != from {
            Some(baidu_response.from.clone())
        } else {
            None
        };

        Ok(TranslationResult {
            source: trans_result.src.clone(),
            target: trans_result.dst.clone(),
            from: from.to_string(),
            to: to.to_string(),
            detected_language,
        })
    }

    fn supported_languages(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("auto", "自动检测"),
            ("zh", "中文"),
            ("en", "英语"),
            ("jp", "日语"),
            ("kor", "韩语"),
            ("fra", "法语"),
            ("spa", "西班牙语"),
            ("de", "德语"),
            ("ru", "俄语"),
            ("th", "泰语"),
            ("ara", "阿拉伯语"),
            ("it", "意大利语"),
            ("pt", "葡萄牙语"),
            ("el", "希腊语"),
            ("nl", "荷兰语"),
            ("pl", "波兰语"),
            ("bul", "保加利亚语"),
            ("est", "爱沙尼亚语"),
            ("dan", "丹麦语"),
            ("fin", "芬兰语"),
            ("cs", "捷克语"),
            ("rom", "罗马尼亚语"),
            ("slo", "斯洛文尼亚语"),
            ("swe", "瑞典语"),
            ("hu", "匈牙利语"),
            ("vie", "越南语"),
        ]
    }

    fn name(&self) -> &'static str {
        "百度翻译"
    }
}
