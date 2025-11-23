use dioxus::prelude::*;
use crate::routes::Route;

/// 博客列表页组件
#[component]
pub fn BlogList() -> Element {
    // TODO: 使用 use_context 获取文章索引
    // 暂时显示占位内容
    
    rsx! {
        div {
            class: "container mx-auto px-4 py-8",
            // 页面标题
            div {
                class: "mb-8",
                h1 {
                    class: "text-4xl font-bold text-gray-900 dark:text-white mb-4",
                    "所有文章"
                }
                p {
                    class: "text-gray-600 dark:text-gray-400",
                    "浏览所有已发布的文章"
                }
            }
            
            // 筛选器（占位）
            div {
                class: "mb-6 flex flex-wrap gap-4",
                button {
                    class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors",
                    "全部"
                }
                button {
                    class: "px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors",
                    "Rust"
                }
                button {
                    class: "px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors",
                    "Dioxus"
                }
            }
            
            // 文章列表（占位）
            div {
                class: "space-y-6",
                for i in 0..10 {
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-lg transition-shadow p-6",
                        div {
                            class: "flex flex-col md:flex-row gap-4",
                            // 封面图片占位
                            div {
                                class: "w-full md:w-48 h-32 bg-gray-200 dark:bg-gray-700 rounded flex-shrink-0",
                            }
                            // 文章信息
                            div {
                                class: "flex-1",
                                h2 {
                                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-2",
                                    Link {
                                        to: Route::BlogPost { slug: format!("article-{}", i + 1) },
                                        class: "hover:text-blue-600 dark:hover:text-blue-400 transition-colors",
                                        "文章标题 {i + 1}"
                                    }
                                }
                                div {
                                    class: "flex items-center gap-4 mb-2 text-sm text-gray-500 dark:text-gray-500",
                                    span { "2025-11-23" }
                                    span { "•" }
                                    span { "5 分钟阅读" }
                                }
                                p {
                                    class: "text-gray-600 dark:text-gray-400 mb-4",
                                    "这是文章的摘要内容，展示文章的主要信息和要点。文章摘要应该简洁明了，能够吸引读者点击阅读全文..."
                                }
                                div {
                                    class: "flex flex-wrap gap-2",
                                    for tag in ["Rust", "Dioxus", "Web"] {
                                        span {
                                            class: "px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded text-sm",
                                            "{tag}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // 提示信息
            div {
                class: "text-center mt-12 text-gray-500 dark:text-gray-500",
                p { "文章列表功能即将实现..." }
            }
        }
    }
}

