use dioxus::prelude::*;
use super::Route;

/// 404 页面组件
#[component]
pub fn NotFound() -> Element {
    rsx! {
        div {
            class: "container mx-auto px-4 py-8 text-center",
            h1 {
                class: "text-6xl font-bold mb-4",
                "404"
            }
            p {
                class: "text-xl text-gray-600 mb-8",
                "页面未找到"
            }
            Link {
                to: Route::Home {},
                class: "text-blue-600 hover:underline",
                "返回首页"
            }
        }
    }
}

