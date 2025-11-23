use dioxus::prelude::*;
use crate::routes::Route;
use crate::components::layout::ThemeToggle;

/// 主布局组件，包含导航栏和页脚
#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div {
            class: "min-h-screen flex flex-col",
            // 导航栏
            header {
                class: "bg-white dark:bg-gray-800 shadow-sm",
                nav {
                    class: "container mx-auto px-4 py-4",
                    div {
                        class: "flex items-center justify-between",
                        // 网站标题/Logo
                        Link {
                            to: Route::Home {},
                            class: "text-2xl font-bold text-gray-900 dark:text-white hover:text-blue-600 dark:hover:text-blue-400",
                            "我的博客"
                        }
                        // 导航链接
                        div {
                            class: "flex items-center space-x-6",
                            Link {
                                to: Route::Home {},
                                class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 transition-colors",
                                "首页"
                            }
                            Link {
                                to: Route::BlogList {},
                                class: "text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 transition-colors",
                                "博客"
                            }
                            // 暗色模式切换按钮
                            ThemeToggle {}
                        }
                    }
                }
            }
            // 主内容区域
            main {
                class: "flex-1",
                Outlet::<Route> {}
            }
            // 页脚
            footer {
                class: "bg-gray-100 dark:bg-gray-900 border-t border-gray-200 dark:border-gray-700 mt-auto",
                div {
                    class: "container mx-auto px-4 py-6",
                    div {
                        class: "text-center text-gray-600 dark:text-gray-400",
                        p {
                            "© 2025 我的博客. 保留所有权利."
                        }
                    }
                }
            }
        }
    }
}

