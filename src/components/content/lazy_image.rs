use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

// 生成唯一 ID 的计数器
static IMAGE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// 懒加载图片组件
#[component]
pub fn LazyImage(
    src: String,
    alt: String,
    class: Option<String>,
) -> Element {
    let mut loaded = use_signal(|| false);
    
    // 生成唯一 ID
    let image_id = format!("lazy-img-{}", IMAGE_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
    let image_id_for_js = image_id.clone();
    let src_for_js = src.clone();
    
    // 使用 Intersection Observer 检测图片是否进入视口
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    // 使用 script 标签执行 JavaScript
                    if let Ok(script) = document.create_element("script") {
                        let js_code = format!(
                            r#"
                            (function() {{
                                const initLazyImage = () => {{
                                    const img = document.getElementById('{}');
                                    if (!img) {{
                                        setTimeout(initLazyImage, 100);
                                        return;
                                    }}
                                    
                                    // 设置 data-src 属性
                                    img.setAttribute('data-src', '{}');
                                    
                                    if ('IntersectionObserver' in window) {{
                                        const observer = new IntersectionObserver((entries) => {{
                                            entries.forEach(entry => {{
                                                if (entry.isIntersecting) {{
                                                    const targetImg = entry.target;
                                                    if (targetImg.dataset.src) {{
                                                        targetImg.src = targetImg.dataset.src;
                                                        targetImg.removeAttribute('data-src');
                                                    }}
                                                    targetImg.classList.add('loaded');
                                                    observer.disconnect();
                                                }}
                                            }});
                                        }}, {{
                                            rootMargin: '50px'
                                        }});
                                        observer.observe(img);
                                    }} else {{
                                        if (img.dataset.src) {{
                                            img.src = img.dataset.src;
                                            img.removeAttribute('data-src');
                                        }}
                                        img.classList.add('loaded');
                                    }}
                                }};
                                initLazyImage();
                            }})();
                            "#,
                            image_id_for_js,
                            src_for_js
                        );
                        script.set_text_content(Some(&js_code));
                        if let Some(head) = document.head() {
                            let _ = head.append_child(&script);
                            // 延迟移除 script 标签
                            let head_clone = head.clone();
                            let script_clone = script.clone();
                            let js_cleanup = format!(
                                r#"
                                setTimeout(function() {{
                                    const head = document.head;
                                    const script = document.getElementById('cleanup-{}');
                                    if (script && head) {{
                                        head.removeChild(script);
                                    }}
                                }}, 200);
                                "#,
                                image_id_for_js
                            );
                            if let Ok(cleanup_script) = document.create_element("script") {
                                cleanup_script.set_id(&format!("cleanup-{}", image_id_for_js));
                                cleanup_script.set_text_content(Some(&js_cleanup));
                                let _ = head.append_child(&cleanup_script);
                            }
                        }
                    }
                }
            }
        }
    });
    
    // 占位符 SVG（1x1 透明图片）
    let placeholder = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1 1'%3E%3C/svg%3E";
    let image_class = class.as_deref().unwrap_or("");
    
    rsx! {
        div {
            class: "lazy-image-container",
            img {
                id: "{image_id}",
                // 使用 data-src 属性（通过 dangerous_inner_html 或直接设置）
                src: placeholder,
                alt: alt.clone(),
                class: "{image_class} lazy-image",
                loading: "lazy",
                onload: move |_| {
                    loaded.set(true);
                }
            }
        }
    }
}
