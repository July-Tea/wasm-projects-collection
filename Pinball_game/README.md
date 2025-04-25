# 简易弹球游戏 - WebAssembly

这是一个使用Rust和WebAssembly实现的简单弹球游戏。游戏具有随机生成的砖块布局，使用鼠标控制挡板来反弹球。

## 游戏特点

- 简单的黑白配色设计
- 随机生成的砖块布局
- 鼠标控制挡板
- 分数和关卡系统
- 物理反弹模拟

## 运行游戏

### 前提条件

- Rust 和 Cargo（建议使用rustup安装）
- wasm-pack（构建脚本会自动安装）
- 支持WebAssembly的现代浏览器

### 构建和运行

1. 克隆仓库
2. 运行构建脚本

```bash
# 使脚本可执行
chmod +x build.sh

# 运行构建脚本
./build.sh
```

3. 打开浏览器访问 http://localhost:8080

## 游戏控制

- 使用鼠标左右移动控制挡板
- 当球落下时，游戏结束
- 点击屏幕重新开始游戏
- 消除所有砖块后，会进入下一关

## 项目结构

- `src/lib.rs`: 游戏的主要代码
- `index.html`: 游戏的HTML模板
- `build.sh`: 构建和运行脚本
- `Cargo.toml`: Rust项目配置文件

## 构建过程

1. wasm-pack将Rust代码编译为WebAssembly
2. 生成的WASM文件和JavaScript胶水代码位于`pkg`目录
3. HTML页面加载并运行WASM模块

## 自定义

如果想要修改游戏，可以调整以下常量：

- `CANVAS_WIDTH` 和 `CANVAS_HEIGHT`: 游戏画布大小
- `BALL_RADIUS`: 球的半径
- `PADDLE_WIDTH` 和 `PADDLE_HEIGHT`: 挡板的尺寸
- `BRICK_ROWS` 和 `BRICK_COLS`: 砖块的行数和列数

## 技术细节

- 使用Canvas 2D API进行渲染
- 使用requestAnimationFrame进行游戏循环
- 通过wasm-bindgen实现Rust和JavaScript的互操作
- 使用web-sys与Web API交互 