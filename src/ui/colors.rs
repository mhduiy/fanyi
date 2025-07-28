use colored::*;
use crate::translator::trait_def::TranslationResult;

/// 显示翻译结果，带有颜色和格式化
pub fn display_translation(result: &TranslationResult, enable_colors: bool, show_detection: bool) {
    if enable_colors {
        display_colored_translation(result, show_detection);
    } else {
        display_plain_translation(result, show_detection);
    }
}

/// 彩色显示翻译结果
fn display_colored_translation(result: &TranslationResult, show_detection: bool) {
    // 显示源语言和目标语言信息
    let lang_info = format!("[{} → {}]", result.from.to_uppercase(), result.to.to_uppercase());
    println!("{}", lang_info.blue().bold());
    
    // 如果有语言检测结果且与输入不同，显示检测信息
    if show_detection {
        if let Some(detected) = &result.detected_language {
            if detected != &result.from {
                let detection_info = format!("检测到语言: {}", detected.to_uppercase());
                println!("{}", detection_info.yellow());
            }
        }
    }
    
    // 显示原文
    println!("{} {}", "原文:".green().bold(), result.source.white());
    
    // 显示翻译结果
    println!("{} {}", "译文:".cyan().bold(), result.target.bright_white().bold());
}

/// 纯文本显示翻译结果
fn display_plain_translation(result: &TranslationResult, show_detection: bool) {
    // 显示源语言和目标语言信息
    println!("[{} → {}]", result.from.to_uppercase(), result.to.to_uppercase());
    
    // 如果有语言检测结果且与输入不同，显示检测信息
    if show_detection {
        if let Some(detected) = &result.detected_language {
            if detected != &result.from {
                println!("检测到语言: {}", detected.to_uppercase());
            }
        }
    }
    
    // 显示原文和译文
    println!("原文: {}", result.source);
    println!("译文: {}", result.target);
}

/// 显示错误信息
pub fn display_error(error: &str, enable_colors: bool) {
    if enable_colors {
        eprintln!("{} {}", "错误:".red().bold(), error);
    } else {
        eprintln!("错误: {}", error);
    }
}

/// 显示成功信息
pub fn display_success(message: &str, enable_colors: bool) {
    if enable_colors {
        println!("{} {}", "✓".green().bold(), message.green());
    } else {
        println!("✓ {}", message);
    }
}

/// 显示警告信息
pub fn display_warning(message: &str, enable_colors: bool) {
    if enable_colors {
        println!("{} {}", "⚠".yellow().bold(), message.yellow());
    } else {
        println!("⚠ {}", message);
    }
}

/// 显示信息
pub fn display_info(message: &str, enable_colors: bool) {
    if enable_colors {
        println!("{} {}", "ℹ".blue().bold(), message);
    } else {
        println!("ℹ {}", message);
    }
} 