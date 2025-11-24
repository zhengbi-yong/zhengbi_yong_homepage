use crate::content::load_embedded_blogs;
use crate::routes::Route;
use dioxus::prelude::*;

/// 博客列表页组件
#[component]
pub fn BlogList() -> Element {
    // 加载嵌入的博客文章
    let post_index = use_signal(|| load_embedded_blogs().unwrap_or_default());
    let posts = post_index.read();
    let all_posts = posts.get_all_posts();

    rsx! {
        div { class: "container mx-auto px-4 py-8",
            // 页面标题
            div { class: "mb-8",
                h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-4",
                    "所有文章"
                }
                p { class: "text-gray-600 dark:text-gray-400",
                    {format!("共找到 {} 篇文章", all_posts.len())}
                }
            }

            // 文章列表
            div { class: "space-y-6",
                for post in all_posts.iter() {
                    div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-lg transition-shadow p-6",
                        div { class: "flex flex-col md:flex-row gap-4",
                            // 封面图片占位
                            // div {
                            //     class: "w-full md:w-48 h-32 bg-gray-200 dark:bg-gray-700 rounded flex-shrink-0",
                            // }
                            // 文章信息
                            div { class: "flex-1",
                                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-2",
                                    Link {
                                        to: Route::BlogPost {
                                            slug: post.metadata.slug.clone().unwrap_or_else(|| "unknown".to_string()),
                                        },
                                        class: "hover:text-blue-600 dark:hover:text-blue-400 transition-colors",
                                        "{post.metadata.title}"
                                    }
                                }
                                div {
                                    class: "flex items-center gap-4 mb-2 text-sm text-gray-500 dark:text-gray-500",
                                    if let Some(date) = post.metadata.date {
                                        span { "{date}" }
                                        span { "•" }
                                    }
                                                                // span { "5 分钟阅读" }
                                }
                                p { class: "text-gray-600 dark:text-gray-400 mb-4",
                                    "{post.metadata.summary.clone().unwrap_or_default()}"
                                }
                                div { class: "flex flex-wrap gap-2",
                                    if let Some(tags) = &post.metadata.tags {
                                        for tag in tags {
                                            span { class: "px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded text-sm",
                                                "{tag}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if all_posts.is_empty() {
                div { class: "text-center mt-12 text-gray-500 dark:text-gray-500",
                    p { "暂无文章" }
                }
            }
        }
    }
}
