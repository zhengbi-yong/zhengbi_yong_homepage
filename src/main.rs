use dioxus::prelude::*;
use crate::routes::Route;
use crate::utils::AppState;

mod routes;
mod content;
mod components;
mod utils;

static CSS: Asset = asset!("/assets/main.css");
static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
static THEME_JS: Asset = asset!("/assets/js/theme.js");
static KATEX_INIT_JS: Asset = asset!("/assets/js/katex-init.js");
static MERMAID_INIT_JS: Asset = asset!("/assets/js/mermaid-init.js");
static HIGHLIGHT_INIT_JS: Asset = asset!("/assets/js/highlight-init.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 创建全局状态
    let app_state = use_signal(|| AppState::new());
    
    // 提供状态给子组件
    use_context_provider(|| app_state);
    
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        document::Stylesheet { href: CSS }
        document::Script { src: THEME_JS }
        document::Script { src: KATEX_INIT_JS }
        document::Script { src: MERMAID_INIT_JS }
        document::Script { src: HIGHLIGHT_INIT_JS }
        Router::<Route> {}
    }
}
