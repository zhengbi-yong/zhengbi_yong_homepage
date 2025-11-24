use dioxus::prelude::*;
use crate::routes::Route;
use crate::content::{render_markdown, parse_markdown_content, BLOGS_DIR};

/// 文章详情页组件
#[component]
pub fn BlogPost(slug: String) -> Element {
    println!("BlogPost rendered for slug: {}", slug);
    
    // 使用 signal 管理 HTML 内容，确保 Dioxus VDOM 与真实 DOM 保持同步
    let mut html_content = use_signal(|| String::new());
    let mut post_title = use_signal(|| "加载中...".to_string());
    let mut post_date = use_signal(|| "".to_string());
    let mut post_tags = use_signal(|| Vec::<String>::new());

    // 模拟异步加载文章内容
    use_effect(move || {
        // 在嵌入的博客目录中查找匹配 slug 的文件
        let found_file = BLOGS_DIR.files().find(|file| {
            file.path()
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.contains(&slug) || slug == "math-test" && s == "math-test") 
                .unwrap_or(false)
        });

        if let Some(file) = found_file {
            if let Some(content_str) = file.contents_utf8() {
                if let Ok(post) = parse_markdown_content(content_str) {
                    post_title.set(post.metadata.title.clone());
                    if let Some(date) = post.metadata.date {
                        post_date.set(date.format("%Y-%m-%d").to_string());
                    }
                    if let Some(tags) = post.metadata.tags {
                        post_tags.set(tags);
                    }
                    
                    // 渲染 Markdown 内容
                    let rendered = render_markdown(&post.content);
                    html_content.set(rendered);
                    return;
                }
            }
        }
        
        html_content.set(format!("<h1>文章未找到</h1><p>无法找到与 '{}' 匹配的文章。</p>", slug));
    });

    // 监听内容变化，触发 JS 渲染
    // 这里只负责 Math/Highlight
    use_effect(move || {
        if html_content.read().is_empty() {
            return;
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            if let Some(window) = window() {
                web_sys::console::log_1(&"Content updated, triggering Math/Highlight init".into());
                if let Some(document) = window.document() {
                    // 延迟执行，确保 DOM 已更新
                    let js_code = r#"
                        (function() {
                            function initContent() {
                                const article = document.querySelector('article');
                                if (!article) {
                                    setTimeout(initContent, 100);
                                    return;
                                }
                                
                                // 重新渲染数学公式
                                if (typeof window.renderMath === 'function') {
                                    try {
                                        window.renderMath(article);
                                    } catch (e) {
                                        console.error('渲染数学公式失败:', e);
                                    }
                                }
                                
                                // 重新高亮代码块
                                if (typeof window.highlightElement === 'function') {
                                    try {
                                        window.highlightElement(article);
                                    } catch (e) {
                                        console.error('高亮代码块失败:', e);
                                    }
                                } else if (typeof window.highlightAll === 'function') {
                                    try {
                                        window.highlightAll();
                                    } catch (e) {
                                        console.error('高亮代码块失败:', e);
                                    }
                                }
                            }
                            
                            // 多次尝试，确保内容已加载
                            setTimeout(initContent, 100);
                            setTimeout(initContent, 300);
                            setTimeout(initContent, 500);
                        })();
                    "#;
                    
                    if let Ok(script) = document.create_element("script") {
                        script.set_text_content(Some(js_code));
                        if let Some(head) = document.head() {
                            let _ = head.append_child(&script);
                            // 延迟移除 script 标签
                            let head_clone = head.clone();
                            let script_clone = script.clone();
                            let js_cleanup = r#"
                                setTimeout(function() {
                                    const head = document.head;
                                    const script = arguments[0];
                                    if (script && head) {
                                        head.removeChild(script);
                                    }
                                }, 300);
                            "#;
                            if let Ok(cleanup_script) = document.create_element("script") {
                                cleanup_script.set_text_content(Some(js_cleanup));
                                let _ = head.append_child(&cleanup_script);
                            }
                        }
                    }
                }
            }
        }
    });
    
    rsx! {
        div {
            class: "container mx-auto px-4 py-8 max-w-4xl",
            // 文章头部信息
            header {
                class: "mb-8",
                h1 {
                    class: "text-4xl font-bold text-gray-900 dark:text-white mb-4",
                    "{post_title}"
                }
                div {
                    class: "flex flex-wrap items-center gap-4 text-sm text-gray-600 dark:text-gray-400 mb-4",
                    if !post_date.read().is_empty() {
                        span { "{post_date}" }
                        span { "•" }
                    }
                    span { "Sisyphus" }
                }
                div {
                    class: "flex flex-wrap gap-2 mb-4",
                    for tag in post_tags.read().iter() {
                        span {
                            class: "px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded-full text-sm",
                            "{tag}"
                        }
                    }
                }
            }
            
            // 文章正文内容
            article {
                class: "prose prose-lg dark:prose-invert max-w-none",
                // 使用 dangerous_inner_html 渲染 signal 中的内容
                div {
                    id: "article-content",
                    dangerous_inner_html: "{html_content}",
                }
            }
            
            // 导航：上一篇/下一篇文章
            nav {
                class: "mt-12 pt-8 border-t border-gray-200 dark:border-gray-700 flex justify-between",
                div {
                    Link {
                        to: Route::BlogPost { slug: "previous-post".to_string() },
                        class: "text-blue-600 dark:text-blue-400 hover:underline",
                        "← 上一篇文章"
                    }
                }
                div {
                    Link {
                        to: Route::BlogPost { slug: "next-post".to_string() },
                        class: "text-blue-600 dark:text-blue-400 hover:underline",
                        "下一篇文章 →"
                    }
                }
            }
            
            // 提示信息
            div {
                class: "text-center mt-8 text-gray-500 dark:text-gray-500 text-sm",
                p { "文章内容渲染功能即将实现..." }
            }
        }
    }
}

