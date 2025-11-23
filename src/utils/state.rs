use crate::content::{PostIndex, Post};
use dioxus::prelude::*;

/// 应用全局状态
#[derive(Clone)]
pub struct AppState {
    /// 文章索引
    pub post_index: PostIndex,
    /// 主题：light 或 dark
    pub theme: Signal<String>,
}

impl AppState {
    /// 创建新的 AppState
    pub fn new() -> Self {
        Self {
            post_index: PostIndex::new(),
            theme: Signal::new("light".to_string()),
        }
    }

    /// 根据 slug 获取文章
    pub fn get_post_by_slug(&self, slug: &str) -> Option<&Post> {
        self.post_index.get_post_by_slug(slug)
    }

    /// 根据标签获取文章列表
    pub fn get_posts_by_tag(&self, tag: &str) -> Vec<&Post> {
        self.post_index.get_posts_by_tag(tag)
    }

    /// 根据分类获取文章列表
    pub fn get_posts_by_category(&self, category: &str) -> Vec<&Post> {
        self.post_index.get_posts_by_category(category)
    }

    /// 获取最近的文章
    pub fn get_recent_posts(&self, count: usize) -> Vec<&Post> {
        self.post_index.get_recent_posts(count)
    }

    /// 切换主题
    pub fn toggle_theme(&mut self) {
        let current = self.theme.read().clone();
        let new_theme = if current == "light" {
            "dark"
        } else {
            "light"
        };
        *self.theme.write() = new_theme.to_string();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

