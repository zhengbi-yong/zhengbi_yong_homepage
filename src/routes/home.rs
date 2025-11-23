use dioxus::prelude::*;
use crate::routes::Route;

/// 首页组件
#[component]
pub fn Home() -> Element {
    // TODO: 使用 use_context 获取文章索引
    // 暂时显示占位内容
    
    rsx! {
        div {
            class: "container mx-auto px-4 py-8",
            // 网站标题和简介
            div {
                class: "text-center mb-12",
                h1 {
                    class: "text-5xl font-bold text-gray-900 dark:text-white mb-4",
                    "欢迎来到我的博客"
                }
                p {
                    class: "text-xl text-gray-600 dark:text-gray-400 max-w-2xl mx-auto",
                    "这里是我的技术博客，分享关于 Rust、Dioxus 和 Web 开发的内容。"
                }
            }
            
            // 最新文章预览卡片（占位）
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                // 示例文章卡片
                for i in 0..6 {
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-lg transition-shadow p-6",
                        div {
                            class: "h-48 bg-gray-200 dark:bg-gray-700 rounded mb-4",
                        }
                        h2 {
                            class: "text-xl font-semibold text-gray-900 dark:text-white mb-2",
                            "示例文章标题 {i + 1}"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-400 text-sm mb-4",
                            "这是文章的摘要内容，展示文章的主要信息..."
                        }
                        div {
                            class: "flex items-center justify-between",
                            span {
                                class: "text-sm text-gray-500 dark:text-gray-500",
                                "2025-11-23"
                            }
                            Link {
                                to: Route::BlogPost { slug: format!("example-post-{}", i + 1) },
                                class: "text-blue-600 dark:text-blue-400 hover:underline text-sm",
                                "阅读更多 →"
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

