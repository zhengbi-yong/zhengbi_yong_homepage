# Dioxus 超高性能个人博客 - 执行计划与手册

## 项目概述

基于 Dioxus 0.7.1 构建超高性能个人博客，支持：
- 静态站点生成（SSG）
- Markdown 渲染（含数学公式和 Mermaid 图表）
- Vue/React 组件集成
- 响应式设计和暗色模式
- 性能优化和离线支持

## 执行阶段划分

### 阶段 1：项目基础配置（步骤 1-5）
### 阶段 2：核心功能实现（步骤 6-15）
### 阶段 3：内容管理系统（步骤 16-20）
### 阶段 4：性能优化（步骤 21-25）
### 阶段 5：用户体验增强（步骤 26-30）
### 阶段 6：部署配置（步骤 31-35）

---

## 详细执行步骤

### 步骤 1：更新 Cargo.toml 依赖配置

**目标：** 添加所有必需的依赖项

**文件路径：** `Cargo.toml`

**AI Prompt：**
```
在 Cargo.toml 中更新依赖配置：
1. 启用 dioxus 的 router feature
2. 添加以下依赖：
   - pulldown-cmark = "0.9"
   - serde = { version = "1.0", features = ["derive"] }
   - serde_yaml = "0.9"
   - chrono = { version = "0.4", features = ["serde"] }
   - walkdir = "2"
   - include_dir = "0.7"
   - thiserror = "1.0"
   - anyhow = "1.0"

确保所有依赖版本兼容，并保持 Cargo.toml 格式正确。
```

**预期结果：**
- Cargo.toml 包含所有必需依赖
- 可以运行 `cargo check` 无错误

---

### 步骤 2：创建项目目录结构

**目标：** 建立完整的项目目录结构

**目录结构：**
```
blogs/                    # Markdown 文件存储
src/
  components/
    animations/          # 动画组件
    content/            # 内容展示组件
    layout/             # 布局组件
  content/              # 内容处理模块
  routes/               # 路由定义
    blog/               # 博客相关路由
  utils/                # 工具函数
  main.rs               # 入口文件
assets/
  js/                   # JavaScript 文件
  css/                  # CSS 文件
  images/               # 图片资源
build.rs                # 构建脚本（可选）
```

**AI Prompt：**
```
创建以下目录结构（如果不存在）：
- blogs/ 目录（项目根目录）
- src/content/ 目录
- src/utils/ 目录
- assets/js/ 目录
- assets/images/ 目录

确保所有目录已创建，并检查现有目录结构是否完整。
```

**预期结果：**
- 所有必需目录已创建
- 目录结构符合项目规范

---

### 步骤 3：定义文章元数据结构

**目标：** 创建 Frontmatter 解析的数据结构

**文件路径：** `src/content/metadata.rs`

**AI Prompt：**
```
在 src/content/metadata.rs 中创建文章元数据结构：

1. 定义 PostMetadata 结构体，包含以下字段（使用 Option 类型以支持可选字段）：
   - title: String（必需）
   - date: chrono::NaiveDate（必需）
   - author: Option<String>
   - tags: Option<Vec<String>>
   - categories: Option<Vec<String>>
   - summary: Option<String>
   - cover_image: Option<String>
   - slug: Option<String>
   - draft: Option<bool>（默认 false）
   - updated: Option<chrono::NaiveDateTime>
   - layout: Option<String>

2. 为 PostMetadata 实现：
   - Debug trait
   - Clone trait
   - PartialEq trait
   - serde::Deserialize trait（使用 serde_yaml）

3. 实现以下方法：
   - new() -> Self（创建默认值）
   - from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error>
   - is_draft(&self) -> bool（检查是否为草稿）
   - get_slug(&self, filename: &str) -> String（从文件名生成 slug）

4. 添加适当的错误处理，使用 thiserror 定义自定义错误类型。

确保代码符合 Rust 最佳实践，包含适当的文档注释。
```

**预期结果：**
- metadata.rs 文件创建完成
- 结构体定义完整，支持所有标准 Frontmatter 字段
- 包含错误处理

---

### 步骤 4：实现 Markdown 解析模块

**目标：** 创建 Markdown 解析和 Frontmatter 分离功能

**文件路径：** `src/content/markdown.rs`

**AI Prompt：**
```
在 src/content/markdown.rs 中实现 Markdown 解析功能：

1. 定义 Post 结构体：
   - metadata: PostMetadata
   - content: String（Markdown 正文）
   - html_content: String（渲染后的 HTML）

2. 实现 parse_markdown_file(path: &Path) -> Result<Post, Error> 函数：
   - 读取文件内容
   - 分离 Frontmatter（YAML，位于 --- 之间）和正文
   - 解析 Frontmatter 为 PostMetadata
   - 返回 Post 结构

3. 实现 render_markdown(markdown: &str) -> String 函数：
   - 使用 pulldown_cmark::Parser::new_ext() 启用所有扩展
   - 使用 pulldown_cmark::html::push_html() 转换为 HTML
   - 返回渲染后的 HTML

4. 实现 process_markdown_file(path: &Path) -> Result<Post, Error> 函数：
   - 调用 parse_markdown_file 解析文件
   - 调用 render_markdown 渲染内容
   - 设置 Post 的 html_content 字段
   - 返回完整的 Post

5. 添加适当的错误处理，使用 anyhow 或 thiserror。

6. 导出必要的类型和函数。

确保代码健壮，处理各种边界情况（如缺少 Frontmatter、格式错误等）。
```

**预期结果：**
- markdown.rs 文件创建完成
- 可以解析 Markdown 文件和 Frontmatter
- 可以渲染 Markdown 为 HTML

---

### 步骤 5：实现内容扫描和索引生成

**目标：** 在构建时扫描 blogs 目录并生成文章索引

**文件路径：** `src/content/index.rs`

**AI Prompt：**
```
在 src/content/index.rs 中实现内容索引功能：

1. 定义 PostIndex 结构体：
   - posts: Vec<Post>
   - tags: HashMap<String, Vec<usize>>（标签到文章索引的映射）
   - categories: HashMap<String, Vec<usize>>（分类到文章索引的映射）
   - sorted_by_date: Vec<usize>（按日期排序的文章索引）

2. 实现 scan_blogs_directory(path: &Path) -> Result<PostIndex, Error> 函数：
   - 使用 walkdir 递归遍历 blogs 目录
   - 找到所有 .md 文件
   - 对每个文件调用 process_markdown_file
   - 过滤掉 draft: true 的文章
   - 构建标签和分类索引
   - 按日期排序文章
   - 返回 PostIndex

3. 实现 PostIndex 的方法：
   - get_posts_by_tag(&self, tag: &str) -> Vec<&Post>
   - get_posts_by_category(&self, category: &str) -> Vec<&Post>
   - get_recent_posts(&self, count: usize) -> Vec<&Post>
   - get_all_posts(&self) -> &[Post]

4. 添加适当的错误处理和日志记录。

5. 导出必要的类型和函数。

确保代码高效，处理大量文章时性能良好。
```

**预期结果：**
- index.rs 文件创建完成
- 可以扫描 blogs 目录
- 可以生成文章索引和分类索引

---

### 步骤 6：创建路由系统

**目标：** 定义博客的路由结构

**文件路径：** `src/routes/mod.rs`

**AI Prompt：**
```
在 src/routes/mod.rs 中定义路由系统：

1. 使用 dioxus::prelude::* 导入必要模块

2. 定义 Route 枚举，使用 #[derive(Routable, Clone, PartialEq)]：
   - #[route("/")] Home {}
   - #[route("/blog")] BlogList {}
   - #[route("/blog/:slug")] BlogPost { slug: String }
   - #[route("/blog/tag/:tag")] BlogTag { tag: String }
   - #[route("/blog/category/:category")] BlogCategory { category: String }
   - #[route("/404")] NotFound {}

3. 为每个路由创建对应的组件函数（暂时返回简单的占位符）：
   - Home() -> Element
   - BlogList() -> Element
   - BlogPost { slug } -> Element
   - BlogTag { tag } -> Element
   - BlogCategory { category } -> Element
   - NotFound() -> Element

4. 导出 Route 枚举和所有组件。

确保路由定义符合 Dioxus 0.7 的规范。
```

**预期结果：**
- mod.rs 文件创建完成
- 路由枚举定义完整
- 所有路由组件已创建（占位符）

---

### 步骤 7：创建布局组件

**目标：** 创建主布局组件，包含导航和页脚

**文件路径：** `src/components/layout/main_layout.rs`

**AI Prompt：**
```
在 src/components/layout/main_layout.rs 中创建主布局组件：

1. 创建 MainLayout 组件：
   - 接受 children 参数（使用 children: Element）
   - 包含以下结构：
     - header（导航栏）
     - main（使用 Outlet<Route> 渲染子路由）
     - footer（页脚）

2. 导航栏包含：
   - 网站标题/Logo（链接到首页）
   - 导航链接：首页、博客列表
   - 暗色模式切换按钮（暂时占位）

3. 使用 Tailwind CSS 类名进行样式设计：
   - 响应式设计（移动端和桌面端）
   - 现代化的视觉效果
   - 适当的间距和布局

4. 页脚包含：
   - 版权信息
   - 社交媒体链接（可选）

5. 导出 MainLayout 组件。

6. 在 src/routes/mod.rs 中为所有路由添加 #[layout(MainLayout)] 注解。

确保布局组件符合 Dioxus 0.7 的组件规范，使用正确的 props 类型。
```

**预期结果：**
- main_layout.rs 文件创建完成
- 布局组件功能完整
- 路由已应用布局

---

### 步骤 8：实现首页组件

**目标：** 创建首页，显示最新文章列表

**文件路径：** `src/routes/home.rs`

**AI Prompt：**
```
在 src/routes/home.rs 中实现首页组件：

1. 创建 Home 组件：
   - 使用 use_context 或全局状态获取文章索引
   - 显示网站标题和简介
   - 显示最新 5-10 篇文章的预览卡片

2. 文章预览卡片包含：
   - 封面图片（如果有）
   - 标题（链接到文章详情页）
   - 发布日期
   - 摘要
   - 标签列表

3. 使用 Tailwind CSS 设计：
   - 响应式网格布局
   - 卡片悬停效果
   - 现代化的视觉设计

4. 实现文章卡片的点击跳转到详情页。

5. 添加加载状态处理（如果数据未准备好）。

确保组件符合 Dioxus 0.7 的规范，使用 Signal 进行状态管理。
```

**预期结果：**
- home.rs 文件创建完成
- 首页显示最新文章列表
- 响应式设计

---

### 步骤 9：实现博客列表页组件

**目标：** 创建博客列表页，显示所有文章

**文件路径：** `src/routes/blog_list.rs`

**AI Prompt：**
```
在 src/routes/blog_list.rs 中实现博客列表页组件：

1. 创建 BlogList 组件：
   - 显示所有已发布的文章
   - 按日期倒序排列
   - 支持分页（可选）

2. 文章列表项包含：
   - 标题（链接）
   - 发布日期
   - 摘要
   - 标签和分类
   - 阅读时间估算（可选）

3. 添加筛选功能：
   - 按标签筛选
   - 按分类筛选
   - 搜索功能（可选）

4. 使用 Tailwind CSS 设计：
   - 清晰的列表布局
   - 响应式设计
   - 筛选器 UI

5. 实现路由跳转到文章详情页。

确保组件性能良好，处理大量文章时流畅。
```

**预期结果：**
- blog_list.rs 文件创建完成
- 博客列表页功能完整
- 支持筛选和搜索

---

### 步骤 10：实现文章详情页组件

**目标：** 创建文章详情页，渲染 Markdown 内容

**文件路径：** `src/routes/blog_post.rs`

**AI Prompt：**
```
在 src/routes/blog_post.rs 中实现文章详情页组件：

1. 创建 BlogPost 组件：
   - 接收 slug 参数
   - 根据 slug 查找对应的文章
   - 显示文章完整内容

2. 文章页面包含：
   - 标题
   - 元数据（日期、作者、标签、分类）
   - 封面图片（如果有）
   - 文章正文（使用 dangerous_inner_html 渲染 HTML）
   - 上一篇/下一篇文章导航

3. 使用 Tailwind CSS 设计：
   - 适合阅读的排版
   - 合适的行宽和字体大小
   - 代码块样式
   - 响应式设计

4. 添加错误处理：
   - 文章不存在时显示 404
   - 加载状态处理

5. 实现数学公式和 Mermaid 图表的初始化（使用 use_effect）：
   - 在组件挂载后调用 KaTeX 的 renderMathInElement
   - 在组件挂载后调用 Mermaid 的 init

确保组件符合 Dioxus 0.7 的规范，正确处理 HTML 注入的安全性。
```

**预期结果：**
- blog_post.rs 文件创建完成
- 文章详情页功能完整
- 支持数学公式和 Mermaid 图表渲染

---

### 步骤 11：创建全局状态管理

**目标：** 使用 Context API 管理文章索引

**文件路径：** `src/utils/state.rs`

**AI Prompt：**
```
在 src/utils/state.rs 中创建全局状态管理：

1. 定义 AppState 结构体：
   - post_index: PostIndex（文章索引）
   - theme: Signal<String>（主题：light/dark）

2. 实现 AppState 的方法：
   - new() -> Self（创建初始状态）
   - get_post_by_slug(&self, slug: &str) -> Option<&Post>
   - get_posts_by_tag(&self, tag: &str) -> Vec<&Post>
   - toggle_theme(&mut self)

3. 在 src/main.rs 的 App 组件中：
   - 使用 use_signal 创建 AppState
   - 使用 use_context_provider 提供状态
   - 在构建时或首次加载时初始化 PostIndex

4. 确保状态在组件间正确共享。

5. 导出 AppState 和相关类型。

确保状态管理符合 Dioxus 0.7 的 Context API 规范。
```

**预期结果：**
- state.rs 文件创建完成
- 全局状态管理功能完整
- 状态在组件间正确共享

---

### 步骤 12：集成 KaTeX 数学公式渲染

**目标：** 在 HTML 模板中添加 KaTeX 支持

**文件路径：** `Dioxus.toml` 和 `assets/js/katex-init.js`

**AI Prompt：**
```
集成 KaTeX 数学公式渲染：

1. 在 Dioxus.toml 的 [web.resource] 中添加：
   - KaTeX CSS：https://cdn.jsdelivr.net/npm/katex@0.13.11/dist/katex.min.css
   - KaTeX JS：https://cdn.jsdelivr.net/npm/katex@0.13.11/dist/katex.min.js
   - KaTeX auto-render：https://cdn.jsdelivr.net/npm/katex@0.13.11/dist/contrib/auto-render.min.js

2. 创建 assets/js/katex-init.js 文件：
   - 定义 renderMath() 函数
   - 使用 renderMathInElement 渲染页面中的数学公式
   - 支持 $$...$$（块级）和 $...$（行内）格式
   - 支持 \\[...\\] 和 \\(...\\) 格式

3. 在文章详情页组件中使用 use_effect：
   - 组件挂载后调用 renderMath() 函数
   - 使用 use_eval 或直接调用 JavaScript

4. 确保数学公式在 Markdown 渲染后正确显示。

确保 KaTeX 正确初始化，支持所有常见的数学公式格式。
```

**预期结果：**
- KaTeX 已集成
- 数学公式可以正确渲染
- 支持多种数学公式格式

---

### 步骤 13：集成 Mermaid 图表渲染

**目标：** 在 HTML 模板中添加 Mermaid 支持

**文件路径：** `Dioxus.toml` 和 `assets/js/mermaid-init.js`

**AI Prompt：**
```
集成 Mermaid 图表渲染：

1. 在 Dioxus.toml 的 [web.resource] 中添加：
   - Mermaid JS（ES 模块）：https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs

2. 创建 assets/js/mermaid-init.js 文件：
   - 使用 ES 模块导入 Mermaid
   - 定义 initMermaid() 函数
   - 配置 Mermaid 主题和选项
   - 调用 mermaid.initialize() 和 mermaid.init()

3. 在 Markdown 渲染时：
   - 确保 ```mermaid 代码块被保留
   - 添加 class="mermaid" 到代码块容器

4. 在文章详情页组件中使用 use_effect：
   - 组件挂载后调用 initMermaid() 函数
   - 处理动态加载的 Mermaid 图表

5. 确保 Mermaid 图表在 Markdown 渲染后正确显示。

确保 Mermaid 正确初始化，支持所有常见的图表类型。
```

**预期结果：**
- Mermaid 已集成
- 图表可以正确渲染
- 支持多种图表类型

---

### 步骤 14：实现暗色模式切换

**目标：** 添加暗色模式支持和切换功能

**文件路径：** `src/components/layout/theme_toggle.rs` 和 `assets/css/theme.css`

**AI Prompt：**
```
实现暗色模式切换功能：

1. 创建 src/components/layout/theme_toggle.rs：
   - 创建 ThemeToggle 组件
   - 使用 use_context 获取主题状态
   - 实现切换按钮
   - 使用 localStorage 保存用户偏好

2. 创建 assets/css/theme.css：
   - 定义 CSS 变量用于主题颜色
   - 定义 .dark 类的样式
   - 使用 prefers-color-scheme 媒体查询

3. 在 MainLayout 中：
   - 添加 ThemeToggle 组件到导航栏
   - 根据主题状态应用 CSS 类

4. 在 src/utils/state.rs 中：
   - 实现 toggle_theme 方法
   - 更新 document.documentElement 的 class
   - 保存到 localStorage

5. 确保主题切换平滑，使用 CSS transition。

确保暗色模式在所有组件中正确应用，包括代码块、数学公式等。
```

**预期结果：**
- 暗色模式功能完整
- 主题切换流畅
- 用户偏好被保存

---

### 步骤 15：实现代码高亮

**目标：** 添加代码语法高亮支持

**文件路径：** `Dioxus.toml` 和 `assets/js/highlight-init.js`

**AI Prompt：**
```
集成代码高亮功能（使用 highlight.js）：

1. 在 Dioxus.toml 的 [web.resource] 中添加：
   - highlight.js CSS：https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css
   - highlight.js JS：https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js

2. 创建 assets/js/highlight-init.js 文件：
   - 定义 initHighlight() 函数
   - 使用 hljs.highlightAll() 高亮所有代码块
   - 支持暗色模式主题切换

3. 在 Markdown 渲染时：
   - 确保代码块保留语言标识
   - 添加适当的 class

4. 在文章详情页组件中使用 use_effect：
   - 组件挂载后调用 initHighlight() 函数
   - 主题切换时重新初始化

5. 确保代码高亮在暗色模式下也正常显示。

确保代码高亮支持常见的编程语言，性能良好。
```

**预期结果：**
- 代码高亮功能完整
- 支持多种编程语言
- 暗色模式兼容

---

### 步骤 16：创建构建时内容处理

**目标：** 在构建时扫描和处理 Markdown 文件

**文件路径：** `build.rs`（可选）或 `src/utils/build.rs`

**AI Prompt：**
```
实现构建时内容处理：

1. 创建 build.rs 文件（在项目根目录）：
   - 使用 walkdir 扫描 blogs 目录
   - 处理所有 .md 文件
   - 生成文章索引 JSON 文件到 assets/ 目录

2. 或者创建 src/utils/build.rs：
   - 定义 generate_index() 函数
   - 在应用启动时调用（开发模式）
   - 在构建时调用（生产模式）

3. 生成的文章索引包含：
   - 所有文章的元数据
   - 标签和分类索引
   - 按日期排序的文章列表

4. 确保构建脚本在 cargo build 时自动运行。

5. 添加适当的错误处理和日志。

确保构建时处理不影响开发体验，性能良好。
```

**预期结果：**
- 构建时内容处理功能完整
- 文章索引自动生成
- 构建流程顺畅

---

### 步骤 17：实现图片懒加载

**目标：** 优化图片加载性能

**文件路径：** `src/components/content/lazy_image.rs` 和 `assets/js/lazy-load.js`

**AI Prompt：**
```
实现图片懒加载功能：

1. 创建 src/components/content/lazy_image.rs：
   - 创建 LazyImage 组件
   - 使用 Intersection Observer API（通过 use_effect）
   - 图片进入视口时才开始加载
   - 显示占位符或模糊效果

2. 创建 assets/js/lazy-load.js：
   - 使用 Intersection Observer 实现懒加载
   - 支持原生 img 标签的 loading="lazy" 属性

3. 在 Markdown 渲染时：
   - 为所有图片添加 loading="lazy" 属性
   - 或使用 LazyImage 组件包装

4. 确保懒加载不影响用户体验：
   - 适当的占位符
   - 平滑的加载动画

确保图片懒加载在所有浏览器中正常工作。
```

**预期结果：**
- 图片懒加载功能完整
- 性能提升明显
- 用户体验良好

---

### 步骤 18：实现响应式图片支持

**目标：** 根据设备提供最优图片

**文件路径：** `src/components/content/responsive_image.rs`

**AI Prompt：**
```
实现响应式图片支持：

1. 创建 src/components/content/responsive_image.rs：
   - 创建 ResponsiveImage 组件
   - 支持 srcset 和 sizes 属性
   - 支持 WebP/AVIF 格式（现代浏览器）
   - 提供 JPEG/PNG 后备（传统浏览器）

2. 在 Markdown 渲染时：
   - 检测图片路径
   - 生成多尺寸图片（如果存在）
   - 使用 picture 元素包装

3. 确保响应式图片：
   - 减少带宽消耗
   - 提升加载速度
   - 保持视觉质量

4. 添加图片优化工具集成（可选）：
   - 在构建时生成多尺寸图片
   - 转换为 WebP/AVIF 格式

确保响应式图片在所有设备上正常工作。
```

**预期结果：**
- 响应式图片功能完整
- 性能优化明显
- 视觉质量保持

---

### 步骤 19：实现 Service Worker 和离线支持

**目标：** 添加 PWA 功能和离线支持

**文件路径：** `assets/js/service-worker.js` 和 `assets/manifest.json`

**AI Prompt：**
```
实现 Service Worker 和离线支持：

1. 创建 assets/js/service-worker.js：
   - 实现 Cache First 策略（静态资源）
   - 实现 Network First 策略（HTML）
   - 缓存版本管理
   - 离线页面支持

2. 创建 assets/manifest.json：
   - 定义 PWA 清单
   - 应用名称、图标、主题色等
   - 显示模式配置

3. 在 src/main.rs 中：
   - 注册 Service Worker
   - 处理更新通知

4. 确保 Service Worker：
   - 正确缓存资源
   - 处理缓存更新
   - 提供离线体验

确保 Service Worker 符合最佳实践，性能良好。
```

**预期结果：**
- Service Worker 功能完整
- 离线支持正常
- PWA 功能可用

---

### 步骤 20：实现错误处理和错误边界

**目标：** 添加全面的错误处理机制

**文件路径：** `src/components/layout/error_boundary.rs`

**AI Prompt：**
```
实现错误处理和错误边界：

1. 创建 src/components/layout/error_boundary.rs：
   - 创建 ErrorBoundary 组件
   - 捕获子组件错误
   - 显示友好的错误信息
   - 提供重试功能

2. 在关键组件中添加错误处理：
   - 文件读取错误
   - 解析错误
   - 渲染错误

3. 实现全局错误处理器：
   - 捕获未处理的错误
   - 记录错误日志（开发环境）
   - 显示错误提示

4. 确保错误处理：
   - 不导致应用崩溃
   - 提供有用的错误信息
   - 允许用户恢复

确保错误处理覆盖所有关键路径。
```

**预期结果：**
- 错误处理功能完整
- 应用稳定性提升
- 用户体验改善

---

### 步骤 21：优化 WebAssembly 体积

**目标：** 减少 WASM 文件大小

**文件路径：** `.cargo/config.toml` 和构建配置

**AI Prompt：**
```
优化 WebAssembly 体积：

1. 创建 .cargo/config.toml：
   - 配置 LTO（Link Time Optimization）
   - 配置代码生成选项
   - 启用优化选项

2. 在 Cargo.toml 中：
   - 配置 profile.release 优化选项
   - 启用 strip 选项

3. 使用 wasm-opt 工具：
   - 在构建后处理 WASM 文件
   - 启用所有优化选项
   - 移除调试信息

4. 代码层面优化：
   - 移除未使用的依赖
   - 使用 feature flags 控制功能
   - 避免不必要的代码

5. 确保优化后：
   - WASM 文件体积显著减小
   - 功能不受影响
   - 性能保持良好

确保优化过程自动化，不影响开发体验。
```

**预期结果：**
- WASM 文件体积减小
- 加载速度提升
- 功能完整

---

### 步骤 22：实现资源预加载和预取

**目标：** 优化资源加载策略

**文件路径：** `src/components/layout/resource_hints.rs`

**AI Prompt：**
```
实现资源预加载和预取：

1. 创建 src/components/layout/resource_hints.rs：
   - 创建 ResourceHints 组件
   - 添加 preload 链接（关键资源）
   - 添加 prefetch 链接（可能访问的资源）
   - 添加 dns-prefetch 链接（第三方域名）

2. 在 MainLayout 中：
   - 预加载关键 CSS 和 JS
   - 预连接 CDN 域名
   - 预取下一页可能访问的资源

3. 根据路由动态调整：
   - 博客列表页预取文章详情页资源
   - 文章详情页预取相关文章资源

4. 确保预加载：
   - 不阻塞关键资源
   - 提升感知性能
   - 不浪费带宽

确保资源预加载策略合理，性能提升明显。
```

**预期结果：**
- 资源预加载功能完整
- 加载速度提升
- 用户体验改善

---

### 步骤 23：实现动画系统

**目标：** 添加流畅的动画效果

**文件路径：** `src/components/animations/` 目录

**AI Prompt：**
```
实现动画系统：

1. 创建动画组件：
   - FadeIn 组件（淡入动画）
   - SlideIn 组件（滑入动画）
   - ScaleIn 组件（缩放动画）

2. 使用 CSS 动画：
   - 使用 transform 和 opacity（GPU 加速）
   - 使用 will-change 提示浏览器
   - 使用 @keyframes 定义动画

3. 在关键交互中添加动画：
   - 页面切换动画
   - 组件出现动画
   - 悬停效果

4. 确保动画：
   - 60fps 流畅
   - 不阻塞主线程
   - 性能良好

5. 添加动画性能监控（开发环境）：
   - FPS 监控
   - 性能警告

确保动画符合最佳实践，性能优秀。
```

**预期结果：**
- 动画系统完整
- 动画流畅
- 性能良好

---

### 步骤 24：实现性能监控

**目标：** 添加性能监控和优化建议

**文件路径：** `src/utils/performance.rs` 和 `assets/js/performance.js`

**AI Prompt：**
```
实现性能监控：

1. 创建 src/utils/performance.rs：
   - 定义性能指标收集
   - Web Vitals 指标（LCP、FID、CLS）
   - 自定义性能指标

2. 创建 assets/js/performance.js：
   - 使用 Performance API
   - 收集性能数据
   - 发送到分析服务（可选）

3. 在关键路径添加性能标记：
   - 页面加载时间
   - 组件渲染时间
   - 资源加载时间

4. 开发环境显示性能面板：
   - 实时性能指标
   - 性能警告
   - 优化建议

5. 确保性能监控：
   - 不影响生产性能
   - 提供有用数据
   - 易于使用

确保性能监控工具完善，有助于持续优化。
```

**预期结果：**
- 性能监控功能完整
- 性能数据准确
- 优化建议有用

---

### 步骤 25：实现代码分割和懒加载

**目标：** 优化初始加载体积

**文件路径：** 路由组件和构建配置

**AI Prompt：**
```
实现代码分割和懒加载：

1. 按路由分割代码：
   - 首页代码
   - 博客列表页代码
   - 文章详情页代码
   - 标签/分类页代码

2. 实现组件懒加载：
   - 使用动态导入
   - 按需加载组件
   - 显示加载状态

3. 优化第三方库加载：
   - KaTeX、Mermaid 按需加载
   - 代码高亮库按需加载

4. 确保代码分割：
   - 初始加载体积减小
   - 不影响功能
   - 加载体验良好

5. 添加加载状态和错误处理。

确保代码分割策略合理，性能提升明显。
```

**预期结果：**
- 代码分割功能完整
- 初始加载体积减小
- 用户体验良好

---

### 步骤 26：实现搜索功能（可选）

**目标：** 添加文章搜索功能

**文件路径：** `src/components/content/search.rs`

**AI Prompt：**
```
实现搜索功能（可选）：

1. 创建 src/components/content/search.rs：
   - 创建 Search 组件
   - 实现客户端搜索
   - 支持标题、内容、标签搜索

2. 实现搜索索引：
   - 在构建时生成搜索索引
   - 使用倒排索引（可选）
   - 支持模糊搜索

3. 搜索 UI：
   - 搜索输入框
   - 搜索结果列表
   - 高亮匹配内容

4. 确保搜索：
   - 性能良好
   - 结果准确
   - 用户体验好

5. 可选：集成 Algolia 或其他搜索服务。

确保搜索功能完善，性能优秀。
```

**预期结果：**
- 搜索功能完整（如果实现）
- 搜索性能良好
- 用户体验良好

---

### 步骤 27：实现 RSS Feed 生成（可选）

**目标：** 生成 RSS/Atom feed

**文件路径：** `src/utils/feed.rs`

**AI Prompt：**
```
实现 RSS Feed 生成（可选）：

1. 创建 src/utils/feed.rs：
   - 定义 RSS/Atom feed 结构
   - 实现 feed 生成函数
   - 包含所有文章信息

2. 在构建时生成 feed.xml：
   - 包含文章标题、摘要、链接
   - 包含发布日期
   - 符合 RSS 2.0 或 Atom 标准

3. 在 MainLayout 中添加 feed 链接：
   - <link rel="alternate" type="application/rss+xml">

4. 确保 feed：
   - 格式正确
   - 内容完整
   - 符合标准

确保 RSS feed 功能完善，符合标准。
```

**预期结果：**
- RSS feed 功能完整（如果实现）
- feed 格式正确
- 符合标准

---

### 步骤 28：实现无障碍支持

**目标：** 提升无障碍访问性

**文件路径：** 所有组件

**AI Prompt：**
```
实现无障碍支持：

1. 添加语义化 HTML：
   - 使用正确的 HTML 标签
   - 使用 ARIA 属性
   - 确保键盘导航

2. 添加无障碍特性：
   - alt 文本（图片）
   - aria-label（按钮）
   - role 属性
   - tabindex 管理

3. 确保：
   - 屏幕阅读器兼容
   - 键盘导航完整
   - 颜色对比度足够
   - 焦点管理正确

4. 测试无障碍性：
   - 使用屏幕阅读器测试
   - 使用键盘导航测试
   - 使用无障碍工具检查

确保应用符合 WCAG 2.1 AA 标准。
```

**预期结果：**
- 无障碍支持完整
- 符合 WCAG 标准
- 所有用户可访问

---

### 步骤 29：实现 SEO 优化

**目标：** 提升搜索引擎优化

**文件路径：** `src/components/layout/seo.rs`

**AI Prompt：**
```
实现 SEO 优化：

1. 创建 src/components/layout/seo.rs：
   - 创建 SEO 组件
   - 动态设置 meta 标签
   - 设置 Open Graph 标签
   - 设置 Twitter Card 标签

2. 为每个页面设置：
   - 标题（title）
   - 描述（description）
   - 关键词（keywords）
   - 规范 URL（canonical）

3. 添加结构化数据：
   - JSON-LD 格式
   - Article 结构化数据
   - BreadcrumbList 结构化数据

4. 确保：
   - 每个页面有唯一标题和描述
   - 图片有 alt 文本
   - 链接有描述性文本
   - 网站地图生成（可选）

确保 SEO 优化完善，搜索引擎友好。
```

**预期结果：**
- SEO 优化完整
- 搜索引擎友好
- 结构化数据正确

---

### 步骤 30：实现安全防护

**目标：** 添加安全措施

**文件路径：** `Dioxus.toml` 和 HTML 模板

**AI Prompt：**
```
实现安全防护：

1. 添加 Content Security Policy (CSP)：
   - 在 HTML meta 标签中设置
   - 限制资源来源
   - 防止 XSS 攻击

2. 确保 HTML 转义：
   - 使用 dangerous_inner_html 时验证内容
   - 转义用户输入
   - 防止 XSS

3. 添加安全头：
   - X-Frame-Options
   - X-Content-Type-Options
   - Referrer-Policy

4. 确保：
   - 所有外部资源使用 HTTPS
   - 不包含敏感信息
   - 输入验证完整

确保安全措施完善，防止常见攻击。
```

**预期结果：**
- 安全防护完整
- 防止常见攻击
- 符合安全最佳实践

---

### 步骤 31：配置构建脚本

**目标：** 优化构建流程

**文件路径：** `build.sh` 或 `build.ps1` 和 CI/CD 配置

**AI Prompt：**
```
配置构建脚本：

1. 创建构建脚本（build.sh 或 build.ps1）：
   - 清理旧构建
   - 运行 cargo build --release
   - 处理 Markdown 文件
   - 优化资源
   - 生成静态文件

2. 配置 GitHub Actions（如果使用）：
   - 安装 Rust 工具链
   - 安装 Dioxus CLI
   - 运行构建
   - 部署到 GitHub Pages

3. 确保构建：
   - 自动化
   - 可重复
   - 错误处理完善

确保构建流程顺畅，自动化程度高。
```

**预期结果：**
- 构建脚本完整
- 构建流程顺畅
- CI/CD 配置正确

---

### 步骤 32：配置部署到 Netlify

**目标：** 配置 Netlify 部署

**文件路径：** `netlify.toml` 和 `_redirects`

**AI Prompt：**
```
配置 Netlify 部署：

1. 创建 netlify.toml：
   - 构建命令：dx build --release --platform web
   - 发布目录：dist
   - 环境变量配置

2. 创建 _redirects 文件：
   - 客户端路由重定向：/* /index.html 200
   - 其他重定向规则

3. 配置 Netlify 设置：
   - 构建钩子
   - 环境变量
   - 域名配置

4. 确保部署：
   - 自动化
   - 错误处理
   - 通知配置

确保 Netlify 部署配置正确，部署顺畅。
```

**预期结果：**
- Netlify 配置完整
- 部署顺畅
- 路由正确

---

### 步骤 33：配置部署到 Vercel

**目标：** 配置 Vercel 部署

**文件路径：** `vercel.json`

**AI Prompt：**
```
配置 Vercel 部署：

1. 创建 vercel.json：
   - 构建命令配置
   - 输出目录配置
   - 重定向规则
   - 头部配置

2. 配置 Vercel 项目：
   - 连接 GitHub 仓库
   - 环境变量
   - 域名配置

3. 确保部署：
   - 自动化
   - 性能优化
   - CDN 配置

确保 Vercel 部署配置正确，部署顺畅。
```

**预期结果：**
- Vercel 配置完整
- 部署顺畅
- 性能优化

---

### 步骤 34：配置部署到 GitHub Pages

**目标：** 配置 GitHub Pages 部署

**文件路径：** `.github/workflows/deploy.yml`

**AI Prompt：**
```
配置 GitHub Pages 部署：

1. 创建 .github/workflows/deploy.yml：
   - 触发条件（push 到 main）
   - 安装 Rust 和 Dioxus CLI
   - 构建项目
   - 部署到 gh-pages 分支

2. 创建 404.html：
   - 处理客户端路由
   - 重定向到 index.html

3. 配置 GitHub Pages：
   - 选择 gh-pages 分支
   - 自定义域名（可选）

4. 确保部署：
   - 自动化
   - 错误处理
   - 通知配置

确保 GitHub Pages 部署配置正确，部署顺畅。
```

**预期结果：**
- GitHub Pages 配置完整
- 部署顺畅
- 路由正确

---

### 步骤 35：最终测试和优化

**目标：** 全面测试和性能优化

**文件路径：** 所有文件

**AI Prompt：**
```
进行最终测试和优化：

1. 功能测试：
   - 所有路由正常工作
   - Markdown 渲染正确
   - 数学公式和 Mermaid 图表正常
   - 暗色模式正常
   - 响应式设计正常

2. 性能测试：
   - 加载速度测试
   - 渲染性能测试
   - 动画流畅度测试
   - 资源大小检查

3. 兼容性测试：
   - 主流浏览器测试
   - 移动设备测试
   - 不同屏幕尺寸测试

4. 无障碍测试：
   - 屏幕阅读器测试
   - 键盘导航测试
   - 颜色对比度检查

5. 安全测试：
   - XSS 防护测试
   - CSP 配置检查
   - 输入验证测试

6. 优化：
   - 修复发现的问题
   - 性能优化
   - 代码清理

确保所有测试通过，应用稳定可靠。
```

**预期结果：**
- 所有测试通过
- 性能优秀
- 应用稳定

---

## 执行顺序说明

### 必须按顺序执行的步骤：
- 步骤 1-5：基础配置和数据结构（必须首先完成）
- 步骤 6-11：核心功能实现（依赖步骤 1-5）
- 步骤 12-15：功能集成（依赖步骤 6-11）

### 可以并行执行的步骤：
- 步骤 16-20：内容管理和优化（可并行）
- 步骤 21-25：性能优化（可并行）
- 步骤 26-30：用户体验增强（可并行）

### 最后执行的步骤：
- 步骤 31-35：部署和测试（必须最后执行）

---

## 每个步骤的执行检查清单

对于每个步骤，执行后应检查：

- [ ] 代码编译无错误
- [ ] 功能按预期工作
- [ ] 符合 Dioxus 0.7 规范
- [ ] 错误处理完善
- [ ] 代码注释清晰
- [ ] 符合 Rust 最佳实践

---

## 注意事项

1. **Dioxus 0.7 API 变化**：
   - 不再使用 `cx`、`Scope`、`use_state`
   - 使用 `Signal`、`use_signal`、`use_memo`
   - 组件 props 必须是 owned 类型

2. **性能考虑**：
   - 避免不必要的重渲染
   - 使用 `use_memo` 缓存计算结果
   - 优化资源加载

3. **安全性**：
   - 使用 `dangerous_inner_html` 时验证内容
   - 转义用户输入
   - 配置 CSP

4. **兼容性**：
   - 测试主流浏览器
   - 提供降级方案
   - 使用特性检测

---

## 完成标准

项目完成时应该满足：

- ✅ 所有功能正常工作
- ✅ 性能指标优秀（LCP < 2.5s, FID < 100ms, CLS < 0.1）
- ✅ 所有测试通过
- ✅ 代码质量高
- ✅ 文档完整
- ✅ 可以成功部署

---

## 后续优化方向

项目完成后可以考虑：

1. 添加评论系统
2. 添加文章分享功能
3. 添加阅读进度指示
4. 添加文章目录导航
5. 添加相关文章推荐
6. 添加统计分析

---

## 使用说明

1. 按照步骤顺序执行
2. 每个步骤使用提供的 AI Prompt
3. 执行后检查清单
4. 遇到问题及时解决
5. 保持代码质量

祝开发顺利！

