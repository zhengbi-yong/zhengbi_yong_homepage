use dioxus::prelude::*;
use crate::components::layout::MainLayout;

mod home;
mod blog_list;
mod blog_post;
mod blog_tag;
mod blog_category;
mod not_found;

pub use home::Home;
pub use blog_list::BlogList;
pub use blog_post::BlogPost;
pub use blog_tag::BlogTag;
pub use blog_category::BlogCategory;
pub use not_found::NotFound;

/// 路由枚举定义
#[derive(Routable, Clone, PartialEq)]
#[layout(MainLayout)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog")]
    BlogList {},
    #[route("/blog/:slug")]
    BlogPost { slug: String },
    #[route("/blog/tag/:tag")]
    BlogTag { tag: String },
    #[route("/blog/category/:category")]
    BlogCategory { category: String },
    #[route("/404")]
    NotFound {},
}

