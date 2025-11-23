use dioxus::prelude::*;

/// 主题切换组件
#[component]
pub fn ThemeToggle() -> Element {
    // 初始化时读取主题
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    // 使用 script 标签执行 JavaScript 来读取主题
                    if let Ok(script) = document.create_element("script") {
                        let js_code = r#"
                            (function() {
                                const html = document.documentElement;
                                return html.classList.contains('dark');
                            })();
                        "#;
                        script.set_text_content(Some(js_code));
                        if let Some(head) = document.head() {
                            let _ = head.append_child(&script);
                            // 立即移除 script 标签
                            let _ = head.remove_child(&script);
                        }
                    }
                }
            }
        }
    });
    
    rsx! {
        button {
            id: "theme-toggle-btn",
            class: "p-2 rounded-md text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors",
            title: "切换主题",
            onclick: move |_| {
                #[cfg(target_arch = "wasm32")]
                {
                    use web_sys::window;
                    if let Some(window) = window() {
                        if let Some(document) = window.document() {
                            // 使用 script 标签执行 JavaScript
                            if let Ok(script) = document.create_element("script") {
                                let js_code = r#"
                                    (function() {
                                        const html = document.documentElement;
                                        const isDark = html.classList.contains('dark');
                                        if (isDark) {
                                            html.classList.remove('dark');
                                            if (window.localStorage) {
                                                window.localStorage.setItem('theme', 'light');
                                            }
                                        } else {
                                            html.classList.add('dark');
                                            if (window.localStorage) {
                                                window.localStorage.setItem('theme', 'dark');
                                            }
                                        }
                                        // 更新按钮图标
                                        const btn = document.getElementById('theme-toggle-btn');
                                        if (btn) {
                                            const newIsDark = document.documentElement.classList.contains('dark');
                                            btn.textContent = newIsDark ? '☀️' : '🌙';
                                        }
                                    })();
                                "#;
                                script.set_text_content(Some(js_code));
                                if let Some(head) = document.head() {
                                    let _ = head.append_child(&script);
                                    // 立即移除 script 标签
                                    let _ = head.remove_child(&script);
                                }
                            }
                        }
                    }
                }
            },
            "🌙"
        }
    }
}
