use dioxus::prelude::*;

/// 标签页组件
#[component]
pub fn BlogTag(tag: String) -> Element {
    rsx! {
        div {
            class: "container mx-auto px-4 py-8",
            h1 {
                class: "text-4xl font-bold mb-8",
                "标签: {tag}"
            }
            div {
                class: "text-center py-12",
                p { "标签筛选功能即将实现..." }
            }
        }
    }
}

