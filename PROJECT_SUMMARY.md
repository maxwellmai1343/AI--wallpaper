# AI 壁纸生成器项目总结

## 1. 项目简介
**AI 壁纸生成器**是一款跨平台的桌面应用程序，旨在通过 AI 技术为用户提供个性化的壁纸生成体验。用户只需输入文本提示词，应用即可调用 AI 接口（目前为 Mock 实现）生成图片，并自动将其设置为桌面壁纸。

- **目标用户**: 追求个性化桌面的用户、设计师、AI 艺术爱好者。
- **核心价值**: 简化壁纸获取流程，实现“所想即所得”的桌面美化体验。

## 2. 核心功能
*   **AI 壁纸生成**: 支持用户输入文本提示词，生成定制化壁纸。
*   **自动设置壁纸**: 生成完成后，自动将图片应用为当前系统的桌面壁纸（支持 macOS 和 Windows）。
*   **智能分辨率适配**: 自动获取当前屏幕分辨率，生成合适尺寸的壁纸。
*   **历史记录管理**: 本地保存生成的壁纸历史，支持查看预览及重新设置为壁纸。
*   **系统托盘支持**: 提供系统托盘图标，支持后台运行和快速访问。
*   **缓存管理**: 支持查看和清理本地缓存的图片及历史记录。

## 3. 技术栈
本项目采用现代化的前后端分离架构，利用 Rust 的高性能和 Tauri 的轻量级特性，结合 Vue 3 的开发效率。

### 前端 (Frontend)
*   **框架**: Vue 3 (Script Setup + TypeScript)
*   **构建工具**: Vite 6
*   **样式库**: Tailwind CSS v4
*   **语言**: TypeScript

### 后端 (Backend)
*   **核心框架**: Tauri v2
*   **编程语言**: Rust
*   **关键依赖**:
    *   `tauri-plugin-opener`: 文件打开支持
    *   `tauri-plugin-fs`: 文件系统操作
    *   `wallpaper`: 跨平台壁纸设置库
    *   `reqwest`: HTTP 客户端 (用于后续对接真实 AI API)
    *   `serde`: 序列化/反序列化

## 4. 架构概览
应用采用典型的 Tauri 架构：
*   **前端层**: 负责用户界面交互，通过 Tauri Invoke 系统调用后端命令。
*   **后端层 (Rust)**: 处理核心业务逻辑，包括：
    *   **AI 适配器**: 目前实现了 `MockAiAdapter` 生成纯色图片，预留了 `AiApiAdapter` 接口用于对接真实 AI 服务（如 DALL-E, Midjourney）。
    *   **存储模块**: 管理本地文件存储（图片缓存）和数据持久化（`history.json`）。
    *   **系统交互**: 调用操作系统 API 获取分辨率、设置壁纸。

## 5. 目录结构
```
ai-wallpaper-app/
├── src/                  # 前端源码
│   ├── assets/           # 静态资源
│   ├── App.vue           # 主应用组件
│   ├── main.ts           # 前端入口
│   └── types.ts          # TypeScript 类型定义
├── src-tauri/            # Tauri 后端源码
│   ├── src/
│   │   ├── main.rs       # Rust 入口
│   │   ├── lib.rs        # 核心逻辑与命令注册
│   │   ├── ai.rs         # AI 接口适配层 (Mock/Real)
│   │   └── storage.rs    # 文件存储与历史记录管理
│   ├── capabilities/     # Tauri 权限配置
│   ├── tauri.conf.json   # Tauri 配置文件
│   └── Cargo.toml        # Rust 依赖配置
├── public/               # 公共静态文件
└── README.md             # 项目说明文档
```

## 6. 快速开始

### 环境要求
*   Node.js (建议 LTS 版本)
*   Rust (最新稳定版)
*   包管理器 (npm/pnpm/yarn)

### 安装与运行
1.  **安装依赖**:
    ```bash
    npm install
    ```
2.  **启动开发环境**:
    ```bash
    npm run tauri dev
    ```
    此命令将同时启动前端开发服务器和 Tauri 窗口。

### 构建发布
```bash
npm run tauri build
```

## 7. 当前状态与未来规划
*   **当前状态**: 完成了 MVP (最小可行性产品) 开发。核心流程跑通，包括前端交互、Rust 命令调用、文件存储、壁纸设置。AI 生成部分目前使用 Mock 数据（纯色图片）。
*   **未来规划**:
    1.  **接入真实 AI API**: 替换 `MockAiAdapter`，接入 OpenAI 或 Stable Diffusion 接口。
    2.  **优化 UI/UX**: 增加更丰富的加载动画、图片预览效果。
    3.  **多屏支持**: 优化多显示器环境下的壁纸设置策略。
    4.  **自定义配置**: 允许用户在设置页配置自己的 API Key 和模型参数。
