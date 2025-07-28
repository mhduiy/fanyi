#!/bin/bash

# Fanyi 翻译工具安装脚本
# 作者: Assistant
# 描述: 将 fanyi 命令行翻译工具安装到用户目录

set -e  # 遇到错误时退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}[信息]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[成功]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[警告]${NC} $1"
}

print_error() {
    echo -e "${RED}[错误]${NC} $1"
}

# 检查命令是否存在
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 主安装函数
main() {
    echo "=========================================="
    echo "    Fanyi 命令行翻译工具安装程序"
    echo "=========================================="
    echo

    # 检查 Rust 和 Cargo
    print_info "检查依赖环境..."
    if ! command_exists rustc; then
        print_error "未找到 Rust 编译器"
        print_info "请先安装 Rust: https://rustup.rs/"
        print_info "运行: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi

    if ! command_exists cargo; then
        print_error "未找到 Cargo 包管理器"
        print_info "Cargo 通常与 Rust 一起安装，请检查 Rust 安装"
        exit 1
    fi

    print_success "依赖环境检查通过"
    echo "  - Rust: $(rustc --version)"
    echo "  - Cargo: $(cargo --version)"
    echo

    # 编译项目
    print_info "编译 fanyi..."
    if ! cargo build --release; then
        print_error "编译失败"
        exit 1
    fi
    print_success "编译完成"
    echo

    # 检查编译产物
    if [ ! -f "target/release/fanyi" ]; then
        print_error "未找到编译后的可执行文件"
        exit 1
    fi

    # 创建安装目录
    INSTALL_DIR="$HOME/.local/bin"
    print_info "创建安装目录: $INSTALL_DIR"
    mkdir -p "$INSTALL_DIR"

    # 安装可执行文件
    print_info "安装 fanyi 到 $INSTALL_DIR..."
    cp "target/release/fanyi" "$INSTALL_DIR/fanyi"
    chmod +x "$INSTALL_DIR/fanyi"
    print_success "安装完成"
    echo

    # 检查 PATH
    print_info "检查 PATH 环境变量..."
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        print_warning "$INSTALL_DIR 不在 PATH 中"
        print_info "请将以下行添加到您的 shell 配置文件中 (~/.bashrc, ~/.zshrc 等):"
        echo
        echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo
        print_info "然后运行: source ~/.bashrc (或相应的配置文件)"
        echo
        
        # 尝试自动添加到 shell 配置
        SHELL_CONFIG=""
        if [ -n "$ZSH_VERSION" ]; then
            SHELL_CONFIG="$HOME/.zshrc"
        elif [ -n "$BASH_VERSION" ]; then
            SHELL_CONFIG="$HOME/.bashrc"
        fi
        
        if [ -n "$SHELL_CONFIG" ] && [ -f "$SHELL_CONFIG" ]; then
            read -p "是否自动添加到 $SHELL_CONFIG? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_CONFIG"
                print_success "已添加到 $SHELL_CONFIG"
                print_info "请运行: source $SHELL_CONFIG"
            fi
        fi
    else
        print_success "$INSTALL_DIR 已在 PATH 中"
    fi
    echo

    # 显示版本信息
    print_info "测试安装..."
    if "$INSTALL_DIR/fanyi" --version >/dev/null 2>&1; then
        print_success "安装测试通过"
        echo "  版本: $("$INSTALL_DIR/fanyi" --version)"
    else
        print_warning "无法运行 fanyi，请检查安装"
    fi
    echo

    # 显示配置提示
    print_info "配置百度翻译API:"
    echo "  1. 访问: https://fanyi-api.baidu.com/"
    echo "  2. 注册并获取 APP ID 和密钥"
    echo "  3. 运行: fanyi config --app-id YOUR_APP_ID --secret-key YOUR_SECRET_KEY"
    echo

    # 显示使用示例
    print_info "使用示例:"
    echo "  fanyi \"你好世界\"                    # 基本翻译"
    echo "  fanyi --from en --to zh \"Hello\"     # 指定语言"
    echo "  fanyi config --show                   # 查看配置"
    echo "  fanyi proxy-status                    # 查看代理状态"
    echo "  fanyi languages                       # 支持的语言"
    echo "  fanyi --help                          # 查看帮助"
    echo

    print_success "安装完成！"
    print_info "如果 fanyi 命令不能直接使用，请确保 ~/.local/bin 在您的 PATH 中"
}

# 卸载函数
uninstall() {
    echo "=========================================="
    echo "    Fanyi 卸载程序"
    echo "=========================================="
    echo

    INSTALL_DIR="$HOME/.local/bin"
    CONFIG_DIR="$HOME/.config/fanyi"

    if [ -f "$INSTALL_DIR/fanyi" ]; then
        print_info "删除可执行文件..."
        rm -f "$INSTALL_DIR/fanyi"
        print_success "已删除 $INSTALL_DIR/fanyi"
    else
        print_warning "未找到可执行文件 $INSTALL_DIR/fanyi"
    fi

    if [ -d "$CONFIG_DIR" ]; then
        read -p "是否删除配置目录 $CONFIG_DIR? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm -rf "$CONFIG_DIR"
            print_success "已删除配置目录"
        else
            print_info "保留配置目录"
        fi
    fi

    print_success "卸载完成"
}

# 解析命令行参数
case "${1:-}" in
    "uninstall"|"remove")
        uninstall
        ;;
    "help"|"--help"|"-h")
        echo "用法: $0 [选项]"
        echo
        echo "选项:"
        echo "  (无参数)     安装 fanyi"
        echo "  uninstall    卸载 fanyi"
        echo "  help         显示此帮助"
        ;;
    "")
        main
        ;;
    *)
        print_error "未知参数: $1"
        echo "使用 '$0 help' 查看帮助"
        exit 1
        ;;
esac 