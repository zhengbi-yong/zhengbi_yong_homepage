use dioxus::prelude::*;
use crate::routes::Route;

/// 文章详情页组件
#[component]
pub fn BlogPost(slug: String) -> Element {
    // TODO: 使用 use_context 根据 slug 获取文章
    // 暂时显示占位内容
    
    // 在组件挂载后初始化数学公式和 Mermaid 图表
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            if let Some(window) = window() {
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
                                
                                // 重新渲染 Mermaid 图表
                                if (typeof window.renderAllMermaid === 'function') {
                                    try {
                                        window.renderAllMermaid().catch(function(e) {
                                            console.error('渲染 Mermaid 图表失败:', e);
                                        });
                                    } catch (e) {
                                        console.error('调用 Mermaid 渲染失败:', e);
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
                    "文章标题示例"
                }
                div {
                    class: "flex flex-wrap items-center gap-4 text-sm text-gray-600 dark:text-gray-400 mb-4",
                    span { "2025-11-23" }
                    span { "•" }
                    span { "作者名称" }
                    span { "•" }
                    span { "5 分钟阅读" }
                }
                div {
                    class: "flex flex-wrap gap-2 mb-4",
                    for tag in ["Rust", "Dioxus", "Web"] {
                        span {
                            class: "px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded-full text-sm",
                            "{tag}"
                        }
                    }
                }
                // 封面图片占位
                div {
                    class: "w-full h-64 bg-gray-200 dark:bg-gray-700 rounded-lg mb-8",
                }
            }
            
            // 文章正文内容
            article {
                class: "prose prose-lg dark:prose-invert max-w-none",
                // 使用 dangerous_inner_html 渲染 Markdown HTML
                div {
                    dangerous_inner_html: {
                        let html = "<h2>文章标题</h2><p>这是文章的正文内容。Markdown 渲染后的 HTML 将显示在这里。</p><h3>代码示例</h3><pre><code class=\"language-rust\">fn main() { println!(\"Hello, Dioxus!\"); }</code></pre><h3>数学公式示例</h3><p>行内公式：$E = mc^2$</p><p>块级公式：</p><p>$$\\int_{-\\infty}^{\\infty} e^{-x^2} dx = \\sqrt{\\pi}$$</p><h3>Mermaid 图表示例</h3><div class=\"mermaid\">graph TD; A[开始] --> B{判断}; B -->|是| C[执行]; B -->|否| D[跳过]; C --> E[结束]; D --> E</div>";
                        html
                    }
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

