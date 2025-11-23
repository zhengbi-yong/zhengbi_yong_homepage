use dioxus::prelude::*;

/// 响应式图片组件
#[component]
pub fn ResponsiveImage(
    src: String,
    alt: String,
    srcset: Option<String>,
    sizes: Option<String>,
    class: Option<String>,
) -> Element {
    let image_class = class.as_deref().unwrap_or("");
    
    // 构建完整的 HTML 字符串以支持 srcset 和 sizes
    let html_content = if let (Some(srcset_val), Some(sizes_val)) = (srcset.as_ref(), sizes.as_ref()) {
        format!(
            r#"<img src="{}" alt="{}" srcset="{}" sizes="{}" class="{}" loading="lazy" />"#,
            src, alt, srcset_val, sizes_val, image_class
        )
    } else if let Some(srcset_val) = srcset.as_ref() {
        format!(
            r#"<img src="{}" alt="{}" srcset="{}" class="{}" loading="lazy" />"#,
            src, alt, srcset_val, image_class
        )
    } else {
        format!(
            r#"<img src="{}" alt="{}" class="{}" loading="lazy" />"#,
            src, alt, image_class
        )
    };
    
    rsx! {
        div {
            dangerous_inner_html: html_content
        }
    }
}

