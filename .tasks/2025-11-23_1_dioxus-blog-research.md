# 背景
文件名：2025-11-23_1_dioxus-blog-research.md
创建于：2025-11-23_14:45:18
创建者：Sisyphus
主分支：master
任务分支：master
Yolo模式：Ask

# 任务描述
调研一个基于 Dioxus 的超高性能个人博客应该怎么做？目前只需要这个个人博客静态部署，然后能够渲染 markdown 以及其中的数学公式和 mermaid 之类的内容。然后需要能够使用 vue 和 react 的组件来展示高质量的画面以及动画。

# 项目概览
- 项目名称：zhengbi_yong_homepage
- Dioxus 版本：0.7.1
- 当前状态：基础项目结构，包含简单的 App 组件
- 已有目录结构：
  - src/components/ (animations, content, layout, slidev)
  - src/routes/blog/
  - src/utils/
  - assets/ (包含 tailwind.css, main.css)
- 当前依赖：仅 dioxus 0.7.1，未启用 router 功能
- 项目已配置 Tailwind CSS（自动模式）

⚠️ 警告：永远不要修改此部分 ⚠️
核心 RIPER-5 协议规则：
- 必须在每个响应开头声明模式 [MODE: MODE_NAME]
- RESEARCH 模式：只允许观察、阅读、提问，禁止建议、实施、规划
- 未经明确许可不能在模式之间转换
- 必须遵循模式转换信号
⚠️ 警告：永远不要修改此部分 ⚠️

# 分析

## 当前项目状态观察

### 项目结构
- 项目使用 Dioxus 0.7.1，这是最新版本，API 已完全改变（不再使用 cx, Scope, use_state）
- 项目目录结构显示已有组件和路由的规划（components/, routes/blog/），但实际代码中只有基础的 main.rs
- 已配置 Tailwind CSS，使用 Dioxus 0.7 的自动模式
- Cargo.lock 中未发现 pulldown-cmark 依赖，说明尚未集成 Markdown 渲染

### 代码现状
- main.rs 包含一个简单的 App 组件，展示 iframe 嵌入外部网站
- 未发现路由系统实现
- 未发现 Markdown 渲染相关代码
- 未发现数学公式或 Mermaid 图表处理逻辑
- 未发现 Vue/React 组件集成相关代码

## 技术调研发现

### 1. Dioxus 0.7 静态站点生成（SSG）

**关键发现：**
- Dioxus 提供了 CLI 工具 `dx`，支持构建命令：`dx build --release`
- 构建命令会生成静态文件，默认输出到 `dist` 目录
- 支持 `--platform web` 参数指定构建平台
- 可以使用 `dx build --release --platform web` 生成 Web 平台的静态文件
- 构建过程会自动处理 WebAssembly 编译和资源打包
- 静态文件可以直接部署到 GitHub Pages、Netlify、Vercel 等静态托管服务

**构建流程确认：**
- 安装 Dioxus CLI：`cargo install dioxus-cli`
- 构建命令：`dx build --release` 或 `dx build --release --platform web`
- 输出目录：`dist/`（包含 HTML、CSS、JS、WASM 等文件）
- 构建时会自动处理路由和资源优化

**静态路由处理：**
- 对于静态站点，需要为每个路由生成对应的 HTML 文件
- 可以使用构建脚本在构建时扫描路由并生成静态页面
- 客户端路由需要配置重定向规则（如 `_redirects` 文件用于 Netlify）

### 2. Markdown 渲染与 Frontmatter 解析

**Markdown 渲染确认：**
- **推荐库**：`pulldown-cmark`（版本 0.9.x）
- 使用 `Parser::new_ext()` 启用所有扩展选项（`Options::all()`）
- 使用 `html::push_html()` 将解析结果转换为 HTML
- 在 Dioxus 中使用 `dangerous_inner_html` 属性注入 HTML

**Frontmatter 解析确认：**
- **推荐格式**：YAML（最常用，兼容性最好）
- **解析库**：`serde_yaml`（版本 0.9.x）
- **标准字段**（基于 Jekyll、Hugo、Hexo 等主流静态站点生成器）：
  - `title`：文章标题（必需）
  - `date`：发布日期，格式 `YYYY-MM-DD` 或 `YYYY-MM-DD HH:MM:SS`
  - `author`：作者名称
  - `tags`：标签数组，如 `["Rust", "Dioxus", "Web"]`
  - `categories`：分类数组
  - `summary` 或 `description`：文章摘要
  - `cover_image` 或 `image`：封面图片 URL
  - `slug`：URL 友好标识符（可选，默认从文件名生成）
  - `draft`：是否为草稿（布尔值）
  - `updated`：最后更新时间
  - `layout`：布局模板名称（可选）

**代码高亮：**
- `pulldown-cmark` 本身不提供代码高亮
- 需要在生成的 HTML 中引入代码高亮库：
  - **highlight.js**：轻量级，支持多种语言
  - **Prism.js**：可扩展，插件丰富
- 在 Markdown 渲染后，对 `<pre><code>` 标签应用高亮库

**图片资源路径：**
- Markdown 中的图片路径需要相对于 `assets/` 目录
- 使用 Dioxus 的 `asset!()` 宏处理资源路径
- 构建时会自动处理资源路径转换

### 3. 数学公式渲染

**技术方案确认：**
- **推荐库**：KaTeX（性能更好，体积更小）或 MathJax（功能更全）
- **引入方式**：在 HTML 模板的 `<head>` 中引入 CDN 链接
- **初始化时机**：在 `DOMContentLoaded` 事件后调用渲染函数

**KaTeX 配置：**
```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.13.11/dist/katex.min.css">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.13.11/dist/katex.min.js"></script>
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.13.11/dist/contrib/auto-render.min.js"></script>
<script>
  document.addEventListener("DOMContentLoaded", function() {
    renderMathInElement(document.body, {
      delimiters: [
        {left: "$$", right: "$$", display: true},
        {left: "$", right: "$", display: false},
        {left: "\\[", right: "\\]", display: true},
        {left: "\\(", right: "\\)", display: false}
      ]
    });
  });
</script>
```

**在 Dioxus 中执行 JavaScript：**
- 可以使用 `use_eval` hook 执行 JavaScript 代码
- 或者在组件挂载后通过 `use_effect` 调用 JavaScript 函数
- 也可以直接在 HTML 模板中引入脚本（推荐用于静态站点）

### 4. Mermaid 图表渲染

**技术方案确认：**
- **推荐库**：Mermaid.js（最新版本 10.x）
- **引入方式**：使用 ES 模块导入或 CDN

**Mermaid 配置：**
```html
<script type="module">
  import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
  mermaid.initialize({ 
    startOnLoad: true,
    theme: 'default'
  });
</script>
```

**在 Markdown 中使用：**
- 使用代码块标记：`` ```mermaid ... ``` ``
- Mermaid 会自动识别并渲染带有 `class="mermaid"` 的代码块
- 需要在 Markdown 渲染时保留 Mermaid 代码块的特殊标记

**初始化时机：**
- 在页面加载完成后自动初始化（`startOnLoad: true`）
- 或在组件挂载后手动调用 `mermaid.init()`

### 5. Vue 和 React 组件集成

**技术方案确认：**

**方案 1：Web Components 封装（推荐）**
- 使用 Vue 3 的 `defineCustomElement` 将组件封装为 Web Component
- 使用 React 的 `react-to-webcomponent` 或 `react-web-component` 库
- 在 Dioxus 中通过自定义元素标签使用：`<my-vue-component />`

**方案 2：JavaScript 手动挂载**
- 在 Dioxus 组件中创建容器元素（带特定 `id`）
- 使用 `use_eval` 或 `use_effect` 在组件挂载后执行 JavaScript
- JavaScript 代码中初始化 Vue/React 应用并挂载到容器

**方案 3：Iframe 嵌入**
- 创建独立的 Vue/React 应用页面
- 在 Dioxus 中使用 `iframe` 标签嵌入（当前代码已有示例）
- 适合完全独立的组件展示

**Dioxus JavaScript 互操作：**
- `use_eval` hook：用于执行 JavaScript 代码并获取返回值
- `use_effect` hook：用于在组件生命周期中执行副作用（如 DOM 操作）
- `web-sys` crate：可以直接操作 DOM，但会增加 WASM 体积

### 6. 路由系统

**路由系统确认：**
- 需要在 `Cargo.toml` 中启用 `router` feature：`dioxus = { version = "0.7.1", features = ["router"] }`
- 使用 `#[derive(Routable, Clone, PartialEq)]` 定义路由枚举
- 使用 `#[route("/path")]` 注解定义路由路径
- 支持动态路由段：`#[route("/blog/:id")]`
- 支持布局组件：`#[layout(LayoutComponent)]`

**静态站点路由处理：**
- 需要为每个路由生成对应的 HTML 文件
- 可以使用构建脚本在构建时扫描所有路由并生成静态页面
- 客户端路由需要配置重定向规则：
  - Netlify：`_redirects` 文件，规则：`/* /index.html 200`
  - Vercel：`vercel.json` 配置重定向
  - GitHub Pages：使用 404.html 处理客户端路由

### 7. 构建流程详细调研

**构建时内容处理：**
- 需要在构建时扫描 `blogs/` 文件夹中的所有 Markdown 文件
- 可以使用 `build.rs` 脚本在编译时处理内容
- 或使用 `include_dir!` 宏在编译时包含目录内容
- 解析 Frontmatter 并生成文章元数据索引
- 将 Markdown 转换为 HTML 并嵌入到 Dioxus 组件中

**资源优化：**
- CSS 和 JavaScript 文件会自动压缩
- 图片资源需要手动优化（可以使用 `image` crate）
- WebAssembly 文件会自动优化

**索引生成：**
- 根据文章元数据生成首页文章列表
- 生成标签页、分类页等索引页面
- 可以生成 RSS/Atom feed

### 8. 部署方案调研

**GitHub Pages：**
- 将 `dist/` 目录内容推送到 `gh-pages` 分支
- 在仓库设置中启用 GitHub Pages
- 支持自定义域名和 HTTPS
- 需要配置 404.html 处理客户端路由

**Netlify：**
- 支持自动部署（连接 GitHub 仓库）
- 构建命令：`dx build --release`
- 发布目录：`dist`
- 重定向规则：创建 `_redirects` 文件，内容：`/* /index.html 200`
- 支持环境变量和构建钩子

**Vercel：**
- 支持自动部署
- 构建命令：`dx build --release`
- 输出目录：`dist`
- 重定向配置：创建 `vercel.json`
- 提供全球 CDN 和边缘计算

**部署配置示例（Netlify）：**
```
构建命令：dx build --release --platform web
发布目录：dist
```

**部署配置示例（Vercel vercel.json）：**
```json
{
  "buildCommand": "dx build --release --platform web",
  "outputDirectory": "dist",
  "rewrites": [
    { "source": "/(.*)", "destination": "/index.html" }
  ]
}
```

## 技术栈确认

基于深入调研，确认的技术栈包括：

**核心依赖：**
- `dioxus = { version = "0.7.1", features = ["router"] }` - 主框架，启用路由功能
- `pulldown-cmark = "0.9"` - Markdown 解析库
- `serde = { version = "1.0", features = ["derive"] }` - 序列化框架
- `serde_yaml = "0.9"` - YAML 解析（用于 Frontmatter）

**前端 JavaScript 库（CDN 引入）：**
- KaTeX 0.13.11 - 数学公式渲染
- Mermaid.js 10.x - 图表渲染
- highlight.js 或 Prism.js - 代码高亮（可选）

**可选依赖：**
- `include_dir` - 编译时包含目录内容（用于构建时扫描 Markdown）
- `walkdir` - 递归遍历目录（用于构建脚本）
- `chrono` - 日期时间处理（用于 Frontmatter 日期解析）

## 架构设计确认

### 内容管理架构

**Markdown 文件存储：**
- 存储位置：`blogs/` 文件夹（项目根目录）
- 文件命名：建议使用 `YYYY-MM-DD-slug.md` 格式
- 文件结构：每个文件包含 YAML Frontmatter + Markdown 正文

**Frontmatter 标准格式：**
```yaml
---
title: "文章标题"
date: 2025-11-23
author: "作者名称"
tags: ["标签1", "标签2"]
categories: ["分类1"]
summary: "文章摘要"
cover_image: "/assets/images/cover.jpg"
draft: false
---
```

**文章元数据结构：**
- 使用 Rust 结构体表示：`PostMetadata`
- 字段支持可选类型（`Option<T>`）以兼容不同文章
- 使用 `serde` 进行序列化/反序列化

### 构建流程架构

**构建时处理流程：**
1. **扫描阶段**：遍历 `blogs/` 目录，收集所有 `.md` 文件
2. **解析阶段**：对每个文件：
   - 分离 Frontmatter 和正文
   - 使用 `serde_yaml` 解析 Frontmatter
   - 使用 `pulldown-cmark` 将正文转换为 HTML
   - 处理代码高亮（可选）
3. **索引生成**：根据元数据生成：
   - 文章列表（按日期排序）
   - 标签索引
   - 分类索引
   - RSS/Atom feed（可选）
4. **页面生成**：为每篇文章生成对应的路由页面
5. **资源优化**：压缩 CSS、JS，优化图片

**构建脚本实现方式：**
- 方案 1：使用 `build.rs` 在编译时处理（推荐）
- 方案 2：使用独立的构建脚本（Rust 或 Shell）
- 方案 3：使用 Dioxus CLI 插件（如果支持）

### 部署架构

**静态文件结构：**
```
dist/
├── index.html          # 首页
├── blog/
│   ├── index.html     # 博客列表页
│   ├── post-1.html    # 文章页面
│   └── post-2.html
├── assets/
│   ├── css/
│   ├── js/
│   └── images/
└── _redirects         # Netlify 重定向规则
```

**重定向规则配置：**
- **Netlify**：`_redirects` 文件
  ```
  /*    /index.html   200
  ```
- **Vercel**：`vercel.json`
  ```json
  {
    "rewrites": [
      { "source": "/(.*)", "destination": "/index.html" }
    ]
  }
  ```
- **GitHub Pages**：使用 `404.html` 处理客户端路由

**CDN 和缓存策略：**
- 静态资源（CSS、JS、图片）设置长期缓存（1年）
- HTML 文件设置短期缓存（1小时）或禁用缓存
- 使用内容哈希作为文件名实现缓存失效

### 性能优化架构

**代码分割：**
- Dioxus 0.7 支持代码分割（需要确认具体实现）
- 按路由分割代码，减少初始加载体积

**资源加载优化：**
- 使用 CDN 加载第三方库（KaTeX、Mermaid）
- 图片懒加载
- 代码高亮库按需加载

**渲染优化：**
- 使用 `use_memo` 缓存 Markdown 渲染结果
- 大型文章使用虚拟滚动（如果实现列表页）
- 数学公式和 Mermaid 图表延迟渲染

## 关键技术问题解答总结

### 已确认的技术问题

1. **静态站点生成**：使用 `dx build --release --platform web` 命令
2. **Markdown 渲染**：使用 `pulldown-cmark` + `dangerous_inner_html`
3. **Frontmatter 解析**：使用 `serde_yaml`，支持标准 YAML 格式
4. **数学公式**：使用 KaTeX，在 HTML 模板中引入并初始化
5. **Mermaid 图表**：使用 Mermaid.js，自动识别代码块
6. **Vue/React 集成**：推荐使用 Web Components 封装
7. **路由系统**：启用 `router` feature，使用 `Routable` trait
8. **构建流程**：在构建时扫描并处理 Markdown 文件
9. **部署方案**：支持 GitHub Pages、Netlify、Vercel

### 待实现的技术细节

1. **构建脚本**：需要实现 `build.rs` 或独立脚本扫描 `blogs/` 目录
2. **代码高亮**：需要选择并集成代码高亮库（highlight.js 或 Prism.js）
3. **Web Components 封装**：需要实现 Vue/React 组件到 Web Components 的转换
4. **RSS Feed 生成**：可选功能，需要实现 RSS/Atom feed 生成
5. **搜索功能**：可选功能，需要实现客户端搜索或集成搜索服务

# 提议的解决方案

## 性能优化方案探索

### 方案 1：渐进式资源加载策略

**核心思路：** 将资源分为关键资源和非关键资源，优先加载关键资源，延迟加载非关键资源。

**优势：**
- 显著提升首屏加载时间（First Contentful Paint）
- 减少初始 JavaScript 和 CSS 体积
- 改善用户体验感知性能

**劣势：**
- 需要仔细识别关键资源
- 可能增加实现复杂度

**具体实现：**
- 关键 CSS 内联到 HTML，非关键 CSS 异步加载
- JavaScript 使用 `defer` 或 `async` 属性
- 图片使用懒加载（Intersection Observer API）
- 第三方库（KaTeX、Mermaid）按需加载

### 方案 2：WebAssembly 体积优化

**核心思路：** 通过多种技术手段减少 WASM 文件体积。

**优势：**
- 减少下载时间
- 降低内存占用
- 提升解析速度

**劣势：**
- 可能需要牺牲部分功能
- 优化过程可能复杂

**具体实现：**
- 使用 `wasm-opt` 工具优化 WASM 文件
- 启用 LTO（Link Time Optimization）
- 移除未使用的代码和符号
- 使用 `wasm-pack` 的 `--target web` 模式
- 考虑使用 `wasm-bindgen` 的 `--no-typescript` 选项

### 方案 3：智能缓存策略

**核心思路：** 多层次缓存策略，减少重复请求。

**优势：**
- 显著提升重复访问速度
- 减少服务器负载
- 改善离线体验

**劣势：**
- 需要处理缓存失效
- 可能占用用户存储空间

**具体实现：**
- 静态资源使用长期缓存（1年）+ 内容哈希
- HTML 使用短期缓存（1小时）或禁用缓存
- Service Worker 实现离线缓存
- 使用 Cache API 缓存 API 响应

## 兼容性优化方案探索

### 方案 1：渐进增强策略

**核心思路：** 确保基本功能在所有浏览器可用，增强功能在支持的浏览器提供。

**优势：**
- 最大兼容性
- 优雅降级
- 不依赖 Polyfill

**劣势：**
- 需要维护多套代码
- 可能限制功能使用

**具体实现：**
- 使用 `<script type="module">` 和 `nomodule` 分别加载现代和传统代码
- 检测浏览器特性，动态加载 Polyfill
- CSS 使用 `@supports` 查询提供降级方案
- WebAssembly 不支持时提供 JavaScript 后备方案

### 方案 2：响应式设计优化

**核心思路：** 确保在所有设备上都有良好的视觉和交互体验。

**优势：**
- 统一的代码库
- 良好的用户体验
- 符合现代 Web 标准

**劣势：**
- 需要仔细设计断点
- 可能增加 CSS 体积

**具体实现：**
- 使用 Tailwind CSS 的响应式工具类
- 移动优先设计（Mobile First）
- 使用 `srcset` 和 `sizes` 提供响应式图片
- 触摸设备优化（增大点击区域，避免悬停依赖）

### 方案 3：浏览器特性检测与 Polyfill

**核心思路：** 动态检测浏览器支持，按需加载 Polyfill。

**优势：**
- 现代浏览器不加载额外代码
- 传统浏览器获得兼容性支持
- 平衡性能和兼容性

**劣势：**
- 需要维护 Polyfill 列表
- 可能增加加载时间（传统浏览器）

**具体实现：**
- 使用 `core-js` 或 `polyfill.io` 按需加载
- 检测 WebAssembly 支持
- 检测 Intersection Observer 支持
- 检测 Service Worker 支持

## 设备观感优化方案探索

### 方案 1：自适应暗色模式

**核心思路：** 支持系统主题检测和手动切换，提供一致的视觉体验。

**优势：**
- 提升用户体验
- 减少眼部疲劳
- 符合现代应用标准

**劣势：**
- 需要维护两套主题
- 可能增加 CSS 体积

**具体实现：**
- 使用 CSS 变量定义主题颜色
- 检测 `prefers-color-scheme` 媒体查询
- 提供手动切换按钮
- 使用 `localStorage` 保存用户偏好
- 平滑过渡动画

### 方案 2：高质量图片与响应式图片

**核心思路：** 根据设备和网络条件提供最优图片。

**优势：**
- 减少带宽消耗
- 提升加载速度
- 保持视觉质量

**劣势：**
- 需要生成多版本图片
- 增加构建复杂度

**具体实现：**
- 使用 WebP 和 AVIF 格式（现代浏览器）
- 提供 JPEG/PNG 后备（传统浏览器）
- 使用 `srcset` 和 `sizes` 属性
- 使用 `loading="lazy"` 实现懒加载
- 使用 `picture` 元素提供艺术指导

### 方案 3：触摸与交互优化

**核心思路：** 针对触摸设备优化交互体验。

**优势：**
- 提升移动设备体验
- 减少误触
- 更自然的交互

**劣势：**
- 需要测试多种设备
- 可能影响桌面体验

**具体实现：**
- 增大触摸目标（至少 44x44px）
- 使用 `touch-action` CSS 属性优化手势
- 避免依赖悬停状态
- 提供触觉反馈（如果支持）
- 优化滚动性能（`-webkit-overflow-scrolling: touch`）

## 鲁棒性优化方案探索

### 方案 1：全面错误处理机制

**核心思路：** 在关键路径添加错误处理，确保应用稳定运行。

**优势：**
- 防止应用崩溃
- 提供友好错误提示
- 便于问题诊断

**劣势：**
- 增加代码复杂度
- 需要设计错误 UI

**具体实现：**
- 使用 Rust 的 `Result` 类型处理错误
- 在组件边界添加错误边界
- 全局错误处理器捕获未处理错误
- 记录错误日志（开发环境）
- 提供错误恢复机制

### 方案 2：输入验证与安全防护

**核心思路：** 防止恶意输入和安全漏洞。

**优势：**
- 提升安全性
- 防止 XSS 攻击
- 保护用户数据

**劣势：**
- 需要验证所有输入
- 可能影响性能

**具体实现：**
- 使用 `dangerous_inner_html` 时进行 HTML 转义
- 验证 Frontmatter 数据格式
- 使用 Content Security Policy (CSP)
- 防止点击劫持（X-Frame-Options）
- 使用 HTTPS 强制传输

### 方案 3：渐进式降级策略

**核心思路：** 在功能不可用时提供降级方案。

**优势：**
- 提升可用性
- 适应不同环境
- 更好的用户体验

**劣势：**
- 需要实现多套方案
- 增加测试复杂度

**具体实现：**
- WebAssembly 不支持时使用 JavaScript 渲染
- Service Worker 不支持时使用传统缓存
- 数学公式渲染失败时显示原始 LaTeX
- Mermaid 渲染失败时显示代码块
- 网络错误时显示离线提示

## 加载速度优化方案探索

### 方案 1：关键渲染路径优化

**核心思路：** 优化从 HTML 到首次渲染的路径。

**优势：**
- 显著提升首屏时间
- 改善用户体验
- 提升 SEO 排名

**劣势：**
- 需要仔细分析
- 可能增加维护成本

**具体实现：**
- 内联关键 CSS
- 延迟加载非关键 CSS
- 使用 `preload` 预加载关键资源
- 使用 `preconnect` 提前建立连接
- 优化字体加载（`font-display: swap`）

### 方案 2：资源预加载与预取

**核心思路：** 预测用户行为，提前加载资源。

**优势：**
- 提升感知性能
- 减少等待时间
- 改善用户体验

**劣势：**
- 可能浪费带宽
- 需要准确预测

**具体实现：**
- 使用 `preload` 预加载关键资源
- 使用 `prefetch` 预取可能访问的资源
- 使用 `dns-prefetch` 提前解析域名
- 使用 `prerender` 预渲染下一页（谨慎使用）

### 方案 3：Service Worker 与离线支持

**核心思路：** 使用 Service Worker 实现离线访问和缓存。

**优势：**
- 离线可用
- 提升重复访问速度
- 提供 PWA 体验

**劣势：**
- 需要处理缓存更新
- 增加实现复杂度

**具体实现：**
- 使用 Cache First 策略缓存静态资源
- 使用 Network First 策略缓存 HTML
- 实现缓存版本管理
- 提供离线提示页面
- 支持后台同步

## 动画流畅度优化方案探索

### 方案 1：GPU 加速动画

**核心思路：** 使用 GPU 加速的属性实现动画。

**优势：**
- 60fps 流畅动画
- 不阻塞主线程
- 低 CPU 占用

**劣势：**
- 可能增加内存占用
- 需要避免过度使用

**具体实现：**
- 使用 `transform` 和 `opacity` 属性
- 避免使用 `left`、`top`、`width`、`height`
- 使用 `will-change` 提示浏览器优化
- 使用 `translateZ(0)` 或 `translate3d(0,0,0)` 触发硬件加速
- 避免在动画中修改布局属性

### 方案 2：高性能动画库选择

**核心思路：** 选择合适的动画库，平衡功能和性能。

**优势：**
- 减少实现工作量
- 获得专业优化
- 丰富的动画效果

**劣势：**
- 可能增加包体积
- 学习成本

**具体实现：**
- **CSS 动画**：简单动画使用 CSS `@keyframes` 和 `transition`
- **GSAP**：复杂动画使用 GSAP（高性能，但体积较大）
- **Framer Motion**：React 风格动画（如果使用 React 组件）
- **自定义动画**：使用 `requestAnimationFrame` 实现自定义动画

### 方案 3：动画性能监控与优化

**核心思路：** 监控动画性能，持续优化。

**优势：**
- 确保流畅体验
- 及时发现问题
- 数据驱动优化

**劣势：**
- 需要实现监控
- 可能影响性能

**具体实现：**
- 使用 `Performance API` 监控帧率
- 使用 Chrome DevTools Performance 面板分析
- 实现 FPS 监控组件（开发环境）
- 检测低性能设备，降低动画复杂度
- 使用 `Intersection Observer` 实现视口内动画

## 综合优化方案推荐

### 推荐方案组合

基于以上探索，推荐以下综合方案：

**性能优化：**
- 渐进式资源加载 + WebAssembly 优化 + 智能缓存
- 关键渲染路径优化 + 资源预加载

**兼容性：**
- 渐进增强 + 响应式设计 + 特性检测

**设备观感：**
- 自适应暗色模式 + 响应式图片 + 触摸优化

**鲁棒性：**
- 全面错误处理 + 输入验证 + 渐进降级

**加载速度：**
- 关键渲染路径优化 + Service Worker + CDN

**动画流畅度：**
- GPU 加速 + CSS 动画优先 + 性能监控

### 方案权衡考虑

**性能 vs 兼容性：**
- 优先支持现代浏览器，提供传统浏览器降级
- 使用特性检测，按需加载 Polyfill

**功能 vs 体积：**
- 核心功能必须包含
- 增强功能按需加载
- 使用代码分割减少初始体积

**体验 vs 性能：**
- 动画使用 GPU 加速，避免性能损失
- 提供性能选项，允许用户禁用动画

**开发效率 vs 优化：**
- 使用工具自动化优化（构建时）
- 保持代码可维护性
- 平衡优化程度和开发时间

# 当前执行步骤
"3. 规划模式 - 执行计划制定"

# 执行计划文档
详细的执行计划和手册已创建：`.tasks/2025-11-23_1_EXECUTION_PLAN.md`

该文档包含：
- 35 个详细执行步骤
- 每个步骤的 AI Prompt
- 文件路径和函数签名
- 预期结果和检查清单
- 执行顺序说明
- 完成标准

# 任务进度
[2025-11-23_16:30:00]
- 已修改：Cargo.toml, src/content/metadata.rs, src/content/markdown.rs, src/content/index.rs, src/content/mod.rs
- 更改：执行步骤 1-5，完成项目基础配置：
  1. 步骤 1：更新 Cargo.toml 依赖配置（启用 router feature，添加所有必需依赖）
  2. 步骤 2：创建项目目录结构（blogs/, src/content/, src/utils/, assets/js/, assets/images/）
  3. 步骤 3：定义文章元数据结构（PostMetadata，支持所有标准 Frontmatter 字段）
  4. 步骤 4：实现 Markdown 解析模块（Post 结构，parse_markdown_file, render_markdown, process_markdown_file）
  5. 步骤 5：实现内容扫描和索引生成（PostIndex，scan_blogs_directory，标签和分类索引）
- 原因：开始执行执行计划，按照步骤手册逐步实施
- 阻碍因素：无
- 状态：成功

[2025-11-23_16:45:00]
- 已修改：src/routes/mod.rs, src/routes/home.rs, src/routes/blog_list.rs, src/routes/blog_post.rs, src/routes/blog_tag.rs, src/routes/blog_category.rs, src/routes/not_found.rs, src/components/layout/main_layout.rs, src/components/layout/theme_toggle.rs, src/components/mod.rs, src/utils/state.rs, src/utils/mod.rs, src/main.rs, Dioxus.toml, assets/js/katex-init.js, assets/js/mermaid-init.js, assets/js/theme.js, assets/js/highlight-init.js, Cargo.toml
- 更改：执行步骤 6-15，完成核心功能实现：
  6. 步骤 6：创建路由系统（Route 枚举，所有路由组件）
  7. 步骤 7：创建布局组件（MainLayout，导航栏和页脚）
  8. 步骤 8：实现首页组件（显示最新文章预览卡片）
  9. 步骤 9：实现博客列表页组件（文章列表和筛选器）
  10. 步骤 10：实现文章详情页组件（Markdown 渲染，数学公式和 Mermaid 支持）
  11. 步骤 11：创建全局状态管理（AppState，Context API）
  12. 步骤 12：集成 KaTeX 数学公式渲染（CDN 引入，初始化脚本）
  13. 步骤 13：集成 Mermaid 图表渲染（ES 模块，初始化脚本）
  14. 步骤 14：实现暗色模式切换（ThemeToggle 组件，JavaScript 集成）
  15. 步骤 15：实现代码高亮（highlight.js，初始化脚本）
- 原因：继续执行执行计划，完成核心功能实现
- 阻碍因素：修复了主题切换组件的 web-sys API 使用问题（改用 JavaScript eval）
- 状态：成功

[2025-11-23_14:45:18]
- 已修改：无
- 更改：完成 Dioxus 0.7 个人博客技术调研
- 原因：用户要求进入研究模式，调研基于 Dioxus 的超高性能个人博客实现方案
- 阻碍因素：无
- 状态：未确认

[2025-11-23_15:00:00]
- 已修改：.tasks/2025-11-23_1_dioxus-blog-research.md
- 更改：深入调研并完善技术细节，包括：
  1. 确认静态站点生成流程（dx build 命令）
  2. 确认 Markdown 渲染和 Frontmatter 解析方案
  3. 确认数学公式（KaTeX）和 Mermaid 图表渲染方案
  4. 确认 Vue/React 组件集成方案（Web Components）
  5. 确认路由系统和构建流程
  6. 确认部署方案（GitHub Pages、Netlify、Vercel）
  7. 定义 Frontmatter 标准字段格式
  8. 确认构建时内容处理流程
  9. 确认重定向规则和 CDN 配置
- 原因：用户要求联网搜索所有需要确认的技术问题，完善任务文件
- 阻碍因素：无
- 状态：未确认

[2025-11-23_15:30:00]
- 已修改：.tasks/2025-11-23_1_dioxus-blog-research.md
- 更改：进入创新模式，深入探索优化方案，包括：
  1. **性能优化方案**：
     - 渐进式资源加载策略（关键资源优先，非关键资源延迟）
     - WebAssembly 体积优化（wasm-opt、LTO、代码移除）
     - 智能缓存策略（多层次缓存、Service Worker、Cache API）
  2. **兼容性优化方案**：
     - 渐进增强策略（基本功能全支持，增强功能按需）
     - 响应式设计优化（移动优先、Tailwind CSS、响应式图片）
     - 浏览器特性检测与 Polyfill（动态加载、按需支持）
  3. **设备观感优化方案**：
     - 自适应暗色模式（系统检测、手动切换、CSS 变量）
     - 高质量图片与响应式图片（WebP/AVIF、srcset、懒加载）
     - 触摸与交互优化（增大触摸目标、手势优化、触觉反馈）
  4. **鲁棒性优化方案**：
     - 全面错误处理机制（Result 类型、错误边界、全局捕获）
     - 输入验证与安全防护（HTML 转义、CSP、HTTPS）
     - 渐进式降级策略（功能不可用时的后备方案）
  5. **加载速度优化方案**：
     - 关键渲染路径优化（内联关键 CSS、预加载、字体优化）
     - 资源预加载与预取（preload、prefetch、dns-prefetch）
     - Service Worker 与离线支持（Cache First、Network First、PWA）
  6. **动画流畅度优化方案**：
     - GPU 加速动画（transform、opacity、will-change）
     - 高性能动画库选择（CSS 动画、GSAP、Framer Motion）
     - 动画性能监控与优化（Performance API、FPS 监控）
  7. **综合方案推荐**：提供方案组合建议和权衡考虑
- 原因：用户要求进入创新模式，从性能、兼容性、设备观感、鲁棒性、加载速度、动画流畅度等维度深入优化方案
- 阻碍因素：无
- 状态：未确认

[2025-11-23_16:00:00]
- 已修改：.tasks/2025-11-23_1_dioxus-blog-research.md, .tasks/2025-11-23_1_EXECUTION_PLAN.md
- 更改：进入规划模式，创建详细的执行计划和手册，包括：
  1. **35 个详细执行步骤**：
     - 步骤 1-5：项目基础配置（依赖管理、目录结构、数据结构）
     - 步骤 6-15：核心功能实现（路由、布局、组件、集成）
     - 步骤 16-20：内容管理系统（构建时处理、图片优化、离线支持）
     - 步骤 21-25：性能优化（WASM 优化、资源预加载、代码分割）
     - 步骤 26-30：用户体验增强（搜索、RSS、无障碍、SEO、安全）
     - 步骤 31-35：部署配置（构建脚本、Netlify、Vercel、GitHub Pages、测试）
  2. **每个步骤包含**：
     - 明确的目标说明
     - 具体的文件路径
     - 详细的 AI Prompt（可直接使用）
     - 预期结果描述
     - 技术规范要求
  3. **执行手册包含**：
     - 执行顺序说明（必须顺序、可并行、最后执行）
     - 每个步骤的检查清单
     - 注意事项和最佳实践
     - 完成标准
     - 后续优化方向
  4. **AI Prompt 特点**：
     - 每个 Prompt 都包含完整的技术要求
     - 明确指定文件路径和函数签名
     - 包含错误处理和最佳实践要求
     - 可直接复制使用，AI 能完美理解并执行
- 原因：用户要求进入规划模式，总结成可执行的步骤，每一步写出能让AI完美工作的prompt，总结成一个计划和执行手册
- 阻碍因素：无
- 状态：未确认

# 最终审查
（待完成）

