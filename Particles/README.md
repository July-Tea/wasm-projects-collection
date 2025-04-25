# 精致的粒子互动系统

这是一个用WebAssembly实现的轻量级、高性能的粒子互动系统。它在一个440×330像素的画布上展示优雅的粒子动画，并支持鼠标互动。

## 功能特点

- 轻量级的粒子系统，性能优化
- 粒子之间的优雅连接线
- 基于深度信息的粒子大小和透明度变化
- 鼠标互动：
  - 鼠标悬停时粒子轻微排斥
  - 点击时粒子被吸引并在点击处新增粒子
- 简约的灰色调配色方案
- 适合作为等待界面的背景

## 技术栈

- Rust 语言
- WebAssembly (wasm-bindgen)
- HTML5 Canvas

## 开发环境设置

### 前提条件

- Rust 和 Cargo (https://www.rust-lang.org/tools/install)
- wasm-pack (https://rustwasm.github.io/wasm-pack/installer/)
- 一个现代浏览器

### 编译步骤

1. 克隆本仓库:
   ```
   git clone [仓库URL]
   cd particles
   ```

2. 使用wasm-pack构建WebAssembly包:
   ```
   wasm-pack build --target web
   ```

3. 启动一个本地服务器:
   ```
   # 如果安装了Python 3:
   python -m http.server
   # 或使用其他静态文件服务器
   ```

4. 在浏览器中访问 `http://localhost:8000`

## 使用方法

1. 将构建好的`pkg`目录和`index.html`文件部署到您的服务器。
2. 在您的HTML中包含`particles-canvas`元素和相应的脚本:
   ```html
   <canvas id="particles-canvas" width="440" height="330"></canvas>
   <script type="module">
       import init, { ParticleAnimation } from './pkg/particles.js';
       
       async function start() {
           await init();
           const animation = new ParticleAnimation('particles-canvas');
           animation.start();
       }
       
       start();
   </script>
   ```

## 自定义选项

您可以通过修改`src/lib.rs`中的参数来调整粒子系统的各种属性:

- 粒子数量
- 粒子大小范围
- 颜色和透明度
- 连线距离阈值
- 鼠标互动范围和强度

## 许可证

MIT

## 作者

[您的名字] 