# Fanyi - 命令行翻译工具

一个简单实用的命令行翻译工具，支持多种语言互译，目前仅支持百度api，后续支持其他api和大模型翻译

## ✨ 特性

- 🌐 **多语言支持** - 支持26种语言互译，包括中、英、日、韩、法、德、俄等
- 🎯 **智能检测** - 自动识别输入文本的语言
- 🌈 **彩色输出** - 美观的颜色显示和格式化
- ⚡ **多种输入** - 支持命令行参数、管道输入、交互模式
- 🔧 **灵活配置** - 支持配置文件和命令行参数
- 🌍 **代理支持** - 智能代理检测和配置
- 📦 **轻量级** - 单个可执行文件，无依赖

## 🚀 快速开始

### 安装

#### 使用安装脚本（推荐）

```bash
# 克隆项目
git clone <repository-url>
cd fanyi

# 运行安装脚本
chmod +x install.sh
./install.sh
```

#### 手动安装

```bash
# 确保已安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 编译项目
cargo build --release

# 复制到用户目录
cp target/release/fanyi ~/.local/bin/

# 确保 ~/.local/bin 在 PATH 中
export PATH="$HOME/.local/bin:$PATH"
```

### 配置API密钥

1. 访问 [百度翻译开放平台](https://fanyi-api.baidu.com/)
2. 注册账号并获取 APP ID 和密钥
3. 配置到工具中：

```bash
fanyi config --app-id YOUR_APP_ID --secret-key YOUR_SECRET_KEY
```

## 📖 使用方法

### 基本翻译

```bash
# 默认中文到英文
fanyi "你好世界"
# 输出：Hello world

# 指定语言方向
fanyi --from en --to zh "Hello world"
# 输出：你好，世界

# 自动检测语言
fanyi --from auto --to zh "Good morning"
# 输出：早上好
```

### 多种输入方式

```bash
# 命令行参数
fanyi "要翻译的文本"

# 管道输入
echo "Hello" | fanyi --to zh

# 交互模式
fanyi
# 然后输入要翻译的文本
```

### 配置管理

```bash
# 查看当前配置
fanyi config --show

# 设置默认语言
fanyi config --default-from zh --default-to en

# 配置代理
fanyi config --proxy-mode disable    # 禁用代理
fanyi config --proxy-mode auto       # 自动检测
fanyi config --proxy-mode enable     # 强制启用
```

### 代理设置

```bash
# 查看代理状态
fanyi proxy-status

# 临时禁用代理
fanyi --no-proxy "文本"

# 临时强制使用代理
fanyi --force-proxy "文本"

# 配置自定义代理
fanyi config --http-proxy http://proxy.example.com:8080
```

### 其他功能

```bash
# 查看支持的语言
fanyi languages

# 禁用颜色输出
fanyi --no-color "文本"

# 查看帮助
fanyi --help
```

## 🛠️ 配置说明

配置文件位置：`~/.config/fanyi/config.toml`

```toml
[translation]
default_from = "zh"      # 默认源语言
default_to = "en"        # 默认目标语言
provider = "baidu"       # 翻译服务提供商

[baidu]
app_id = "YOUR_APP_ID"           # 百度翻译APP ID
secret_key = "YOUR_SECRET_KEY"   # 百度翻译密钥
api_url = "https://fanyi-api.baidu.com/api/trans/vip/translate"

[ui]
enable_colors = true             # 启用颜色输出
show_language_detection = true   # 显示语言检测结果

[proxy]
enabled = "auto"                 # 代理模式：auto/enable/disable
http_proxy = ""                  # HTTP代理地址（可选）
https_proxy = ""                 # HTTPS代理地址（可选）
```

## 🌍 代理配置

工具支持三种代理模式：

- **auto（默认）**：自动检测环境变量中的代理设置
- **enable**：强制启用代理
- **disable**：禁用所有代理

### 环境变量支持

工具会自动检测以下环境变量：
- `http_proxy` / `HTTP_PROXY`
- `https_proxy` / `HTTPS_PROXY`

### 代理优先级

1. 命令行参数（`--no-proxy`, `--force-proxy`）
2. 配置文件中的代理设置
3. 环境变量

## 🌐 支持的语言

| 代码 | 语言 | 代码 | 语言 |
|------|------|------|------|
| auto | 自动检测 | zh | 中文 |
| en | 英语 | jp | 日语 |
| kor | 韩语 | fra | 法语 |
| spa | 西班牙语 | de | 德语 |
| ru | 俄语 | th | 泰语 |
| ara | 阿拉伯语 | it | 意大利语 |
| pt | 葡萄牙语 | el | 希腊语 |
| nl | 荷兰语 | pl | 波兰语 |
| ... | 更多语言请使用 `fanyi languages` 查看 |

## 🔧 命令行参数

```
fanyi [选项] [文本] [命令]

选项:
  -f, --from <FROM>      源语言 (例如: zh, en, ja)
  -t, --to <TO>          目标语言 (例如: zh, en, ja)
      --no-color         禁用颜色输出
      --no-proxy         禁用代理
      --force-proxy      强制使用代理
  -h, --help             显示帮助信息
  -V, --version          显示版本信息

命令:
  config                 配置API密钥和设置
  languages             列出支持的语言
  proxy-status          显示代理状态
```

## 📝 使用示例

### 日常翻译

```bash
# 中译英（默认）
fanyi "今天天气很好"
# [ZH → EN]
# 原文: 今天天气很好
# 译文: The weather is very good today

# 英译中
fanyi --from en --to zh "How are you?"
# [EN → ZH]
# 原文: How are you?
# 译文: 你好吗？

# 自动检测 + 翻译
fanyi --from auto --to zh "Bonjour"
# [AUTO → ZH]
# 检测到语言: FRA
# 原文: Bonjour
# 译文: 你好
```

### 批量翻译

```bash
# 使用管道
echo -e "Hello\nWorld" | xargs -I {} fanyi --to zh "{}"

# 翻译文件内容
cat text.txt | fanyi --from en --to zh
```

### 代理配置示例

```bash
# 查看当前代理状态
fanyi proxy-status
# === 代理状态 ===
# 配置的代理模式: Auto
# 当前生效的HTTP代理: http://proxy.example.com:8080
# 当前生效的HTTPS代理: http://proxy.example.com:8080

# 禁用代理进行翻译
fanyi --no-proxy "无需代理的翻译"

# 在有代理的环境中临时禁用
fanyi config --proxy-mode disable
```

## 🛠️ 开发

### 编译

```bash
# 开发版本
cargo build

# 发布版本
cargo build --release
```

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy
cargo fmt
```

## 🔄 卸载

```bash
# 使用安装脚本卸载
./install.sh uninstall

# 或手动删除
rm ~/.local/bin/fanyi
rm -rf ~/.config/fanyi  # 删除配置（可选）
```

## ❗ 故障排除

### 网络连接问题

1. 检查网络连接
2. 检查代理设置：`fanyi proxy-status`
3. 尝试禁用代理：`fanyi --no-proxy "测试"`

### API密钥问题

1. 确认API密钥正确：`fanyi config --show`
2. 检查百度翻译账户余额和调用次数
3. 重新配置：`fanyi config --app-id YOUR_ID --secret-key YOUR_KEY`

### 安装问题

1. 确保Rust已正确安装：`rustc --version`
2. 检查PATH设置：`echo $PATH`
3. 确认 `~/.local/bin` 在PATH中

## 📄 许可证

MIT License
