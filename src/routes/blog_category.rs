use dioxus::prelude::*;

/// 分类页组件
#[component]
pub fn BlogCategory(category: String) -> Element {
    rsx! {
        div {
            class: "container mx-auto px-4 py-8",
            h1 {
                class: "text-4xl font-bold mb-8",
                "分类: {category}"
            }
            div {
                class: "text-center py-12",
                p { "分类筛选功能即将实现..." }
            }
        }
    }
}

