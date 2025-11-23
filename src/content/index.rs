use crate::content::{Post, process_markdown_file};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 文章索引，包含所有文章和分类信息
#[derive(Debug, Clone)]
pub struct PostIndex {
    /// 所有文章列表
    pub posts: Vec<Post>,
    /// 标签到文章索引的映射
    pub tags: HashMap<String, Vec<usize>>,
    /// 分类到文章索引的映射
    pub categories: HashMap<String, Vec<usize>>,
    /// 按日期排序的文章索引（从新到旧）
    pub sorted_by_date: Vec<usize>,
}

impl PostIndex {
    /// 创建空的 PostIndex
    pub fn new() -> Self {
        Self {
            posts: Vec::new(),
            tags: HashMap::new(),
            categories: HashMap::new(),
            sorted_by_date: Vec::new(),
        }
    }

    /// 根据标签获取文章列表
    pub fn get_posts_by_tag(&self, tag: &str) -> Vec<&Post> {
        self.tags
            .get(tag)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&idx| self.posts.get(idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 根据分类获取文章列表
    pub fn get_posts_by_category(&self, category: &str) -> Vec<&Post> {
        self.categories
            .get(category)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&idx| self.posts.get(idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 获取最近的文章（按日期排序）
    pub fn get_recent_posts(&self, count: usize) -> Vec<&Post> {
        self.sorted_by_date
            .iter()
            .take(count)
            .filter_map(|&idx| self.posts.get(idx))
            .collect()
    }

    /// 获取所有文章
    pub fn get_all_posts(&self) -> &[Post] {
        &self.posts
    }

    /// 根据 slug 查找文章
    pub fn get_post_by_slug(&self, slug: &str) -> Option<&Post> {
        self.posts.iter().find(|post| {
            let post_slug = post.metadata.get_slug("");
            post_slug == slug || post.metadata.slug.as_ref().map(|s| s == slug).unwrap_or(false)
        })
    }
}

impl Default for PostIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// 扫描 blogs 目录，生成文章索引
pub fn scan_blogs_directory(path: &Path) -> Result<PostIndex> {
    let mut index = PostIndex::new();
    let mut posts_with_dates: Vec<(usize, chrono::NaiveDate)> = Vec::new();

    // 遍历 blogs 目录
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
    {
        let file_path = entry.path();
        
        // 处理 Markdown 文件
        match process_markdown_file(file_path) {
            Ok(post) => {
                // 跳过草稿
                if post.metadata.is_draft() {
                    continue;
                }

                let post_idx = index.posts.len();
                index.posts.push(post);

                // 获取当前文章的引用（用于构建索引）
                let current_post = &index.posts[post_idx];

                // 构建标签索引
                if let Some(ref tags) = current_post.metadata.tags {
                    for tag in tags {
                        index.tags.entry(tag.clone()).or_insert_with(Vec::new).push(post_idx);
                    }
                }

                // 构建分类索引
                if let Some(ref categories) = current_post.metadata.categories {
                    for category in categories {
                        index.categories.entry(category.clone()).or_insert_with(Vec::new).push(post_idx);
                    }
                }

                // 记录日期用于排序
                let date = current_post.metadata.date.unwrap_or_else(|| {
                    // 如果没有日期，尝试从文件名提取
                    extract_date_from_filename(file_path)
                        .unwrap_or_else(|| chrono::Local::now().date_naive())
                });
                posts_with_dates.push((post_idx, date));
            }
            Err(e) => {
                eprintln!("警告：处理文件失败 {}: {}", file_path.display(), e);
                // 继续处理其他文件
            }
        }
    }

    // 按日期排序（从新到旧）
    posts_with_dates.sort_by(|a, b| b.1.cmp(&a.1));
    index.sorted_by_date = posts_with_dates.into_iter().map(|(idx, _)| idx).collect();

    Ok(index)
}

/// 从文件名提取日期（格式：YYYY-MM-DD-xxx.md）
fn extract_date_from_filename(path: &Path) -> Option<chrono::NaiveDate> {
    let filename = path.file_stem()?.to_str()?;
    
    // 检查是否以日期开头（YYYY-MM-DD）
    if filename.len() >= 10 {
        let date_str = &filename[..10];
        if date_str.chars().take(4).all(|c| c.is_ascii_digit())
            && date_str.chars().nth(4) == Some('-')
            && date_str.chars().skip(5).take(2).all(|c| c.is_ascii_digit())
            && date_str.chars().nth(7) == Some('-')
            && date_str.chars().skip(8).take(2).all(|c| c.is_ascii_digit())
        {
            return chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok();
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_index() {
        let mut index = PostIndex::new();
        
        // 测试空索引
        assert_eq!(index.get_all_posts().len(), 0);
        assert_eq!(index.get_recent_posts(5).len(), 0);
    }

    #[test]
    fn test_extract_date_from_filename() {
        let path = Path::new("2025-11-23-test-article.md");
        let date = extract_date_from_filename(path);
        assert!(date.is_some());
        assert_eq!(date.unwrap().format("%Y-%m-%d").to_string(), "2025-11-23");
    }
}

