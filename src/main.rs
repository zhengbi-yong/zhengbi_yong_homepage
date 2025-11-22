use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let external_url = "https://www.bilibili.com";
    rsx! {      
        document::Stylesheet { href: CSS }  
             
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
        // div { id: "dogview",
        //     img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        // }
        div { iframe {
                src: "{external_url}",
                width: "100%", // 宽度占满父容器
                height: "600px", // 固定的高度
                // 推荐添加一个 title 属性用于可访问性
                title: "嵌入的外部网页",
                // 允许一些功能，例如全屏
                allow: "fullscreen",
                // 可选：设置边框为 0
                frame_border: "0"
            }
        }
        div { id: "buttons",
            button { id: "skip", "skip" }
            button { id: "save", "save!" }
        }
}
}
