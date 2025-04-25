#!/bin/bash

# 设置PATH以包含Cargo bin目录
export PATH="$HOME/.cargo/bin:$PATH"

# 安装wasm-pack（如果需要）
if ! command -v wasm-pack &> /dev/null; then
    echo "正在尝试通过cargo安装wasm-pack..."
    cargo install wasm-pack
fi

# 编译为WebAssembly
echo "正在编译为WebAssembly..."
wasm-pack build --target web

# 创建pkg文件夹（如果不存在）
if [ ! -d "pkg" ]; then
    echo "创建pkg文件夹..."
    mkdir -p pkg
fi

# 启动本地服务器
echo "启动本地服务器在 http://localhost:8080..."
if command -v python3 &> /dev/null; then
    python3 -m http.server 8080
elif command -v python &> /dev/null; then
    python -m http.server 8080
else
    echo "错误: 未找到Python，无法启动HTTP服务器"
    exit 1
fi 