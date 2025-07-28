use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// 翻译结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    /// 源文本
    pub source: String,
    /// 翻译结果
    pub target: String,
    /// 源语言
    pub from: String,
    /// 目标语言
    pub to: String,
    /// 检测到的源语言（如果自动检测）
    pub detected_language: Option<String>,
}

/// 翻译器抽象接口
#[async_trait]
pub trait Translator: Send + Sync {
    /// 翻译文本
    /// 
    /// # 参数
    /// - `text`: 要翻译的文本
    /// - `from`: 源语言代码，"auto" 表示自动检测
    /// - `to`: 目标语言代码
    /// 
    /// # 返回
    /// 返回翻译结果
    async fn translate(&self, text: &str, from: &str, to: &str) -> Result<TranslationResult>;
    
    /// 获取支持的语言列表
    fn supported_languages(&self) -> Vec<(&'static str, &'static str)>;
    
    /// 获取翻译器名称
    fn name(&self) -> &'static str;
} 