# 个人 WebAssembly 项目集

本仓库是个人向的 WebAssembly 项目集合，用于展示和存储 Rust + WebAssembly 技术栈开发的各种小项目。所有项目均为独立模块，可以方便地集成到各种网页应用中。

## 项目列表

### 1. 粒子互动系统 (Particles)

一个轻量级、高性能的粒子互动系统，特点：

- **标准化尺寸**：440×330 像素的画布，适合嵌入到各种界面
- **粒子互动效果**：粒子之间的连接线，基于深度信息的大小和透明度变化
- **鼠标互动**：悬停排斥效果，点击吸引并新增粒子

### 2. 弹球游戏 (Pinball Game)

使用 Rust 和 WebAssembly 实现的简单弹球游戏：

- **标准化尺寸**：440×330 像素的画布，保持 4:3 的宽高比
- **游戏元素**：随机生成的砖块布局，物理反弹模拟
- **操作方式**：使用鼠标控制挡板
- **游戏机制**：分数和关卡系统

## WebAssembly 开发简易流程

### 1. 环境配置

```bash
# 安装 Rust 和 Cargo
brew install Rust

# 安装 wasm-pack
cargo install wasm-pack

# 添加 wasm32-unknown-unknown 目标
rustup target add wasm32-unknown-unknown
```

### 2. 创建项目

```bash
# 创建一个新的库项目
cargo new --lib my_wasm_project
cd my_wasm_project

# 编辑 Cargo.toml 添加必要依赖
# 示例内容:
# [lib]
# crate-type = ["cdylib", "rlib"]
# 
# [dependencies]
# wasm-bindgen = "0.2"
# js-sys = "0.3"
# web-sys = { version = "0.3", features = [...] }
```

### 3. 编写 Rust 代码

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MyApp {
    // 应用状态
}

#[wasm_bindgen]
impl MyApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // 初始化
    }
    
    pub fn update(&mut self) {
        // 更新逻辑
    }
    
    pub fn render(&self) {
        // 渲染逻辑
    }
}
```

### 4. 构建 WebAssembly

```bash
# 构建面向 web 的 wasm 包
wasm-pack build --target web
```

### 5. 在 HTML 中使用

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>WASM 应用</title>
    <style>
        canvas {
            border: 1px solid #ccc;
        }
    </style>
</head>
<body>
    <canvas id="canvas" width="440" height="330"></canvas>
    
    <script type="module">
        import init, { MyApp } from './pkg/my_wasm_project.js';
        
        async function run() {
            // 初始化 WASM 模块
            await init();
            
            // 创建应用实例
            const app = new MyApp();
            
            // 应用循环
            function loop() {
                app.update();
                app.render();
                requestAnimationFrame(loop);
            }
            
            requestAnimationFrame(loop);
        }
        
        run();
    </script>
</body>
</html>
```