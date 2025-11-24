use dioxus::prelude::*;
use crate::routes::Route;
use crate::content::load_embedded_blogs;

/// 首页组件
#[component]
pub fn Home() -> Element {
    // 加载嵌入的博客文章
    let post_index = use_signal(|| load_embedded_blogs().unwrap_or_default());
    let posts_read = post_index.read();
    let recent_posts = posts_read.get_recent_posts(6);
    
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
            
            // 最新文章预览卡片
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                for post in recent_posts.iter() {
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-lg transition-shadow p-6",
                        // 封面图片占位 (如果有)
                        // div {
                        //     class: "h-48 bg-gray-200 dark:bg-gray-700 rounded mb-4",
                        // }
                        h2 {
                            class: "text-xl font-semibold text-gray-900 dark:text-white mb-2",
                            "{post.metadata.title}"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-400 text-sm mb-4",
                            "{post.metadata.summary.clone().unwrap_or_default()}"
                        }
                        div {
                            class: "flex items-center justify-between",
                            span {
                                class: "text-sm text-gray-500 dark:text-gray-500",
                                if let Some(date) = post.metadata.date {
                                    "{date}"
                                } else {
                                    ""
                                }
                            }
                            Link {
                                to: Route::BlogPost { 
                                    slug: post.metadata.slug.clone().unwrap_or_else(|| "unknown".to_string()) 
                                },
                                class: "text-blue-600 dark:text-blue-400 hover:underline text-sm",
                                "阅读更多 →"
                            }
                        }
                    }
                }
            }
            
            if recent_posts.is_empty() {
                div {
                    class: "text-center mt-12 text-gray-500 dark:text-gray-500",
                    p { "暂无文章" }
                }
            }
        }
    }
}

