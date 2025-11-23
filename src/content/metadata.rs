use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Deserializer};
use std::path::Path;

/// 自定义日期反序列化函数
fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(date_str) => {
            // 尝试多种日期格式
            if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                Ok(Some(date))
            } else if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y/%m/%d") {
                Ok(Some(date))
            } else {
                // 如果解析失败，返回 None
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

/// 自定义日期时间反序列化函数
fn deserialize_datetime<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(datetime_str) => {
            // 尝试多种日期时间格式
            if let Ok(dt) = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                Ok(Some(dt))
            } else if let Ok(dt) = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%dT%H:%M:%S") {
                Ok(Some(dt))
            } else if let Ok(date) = NaiveDate::parse_from_str(&datetime_str, "%Y-%m-%d") {
                // 如果只有日期，转换为日期时间（00:00:00）
                Ok(Some(date.and_hms_opt(0, 0, 0).unwrap()))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

/// 文章元数据结构，用于解析 Frontmatter
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PostMetadata {
    /// 文章标题（必需）
    pub title: String,
    /// 发布日期（必需）
    #[serde(deserialize_with = "deserialize_date")]
    pub date: Option<NaiveDate>,
    /// 作者名称
    pub author: Option<String>,
    /// 标签列表
    pub tags: Option<Vec<String>>,
    /// 分类列表
    pub categories: Option<Vec<String>>,
    /// 文章摘要
    pub summary: Option<String>,
    /// 封面图片 URL
    pub cover_image: Option<String>,
    /// URL 友好标识符
    pub slug: Option<String>,
    /// 是否为草稿
    #[serde(default = "default_false")]
    pub draft: bool,
    /// 最后更新时间
    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated: Option<NaiveDateTime>,
    /// 布局模板名称
    pub layout: Option<String>,
}

fn default_false() -> bool {
    false
}

impl PostMetadata {
    /// 创建默认的 PostMetadata
    pub fn new() -> Self {
        Self {
            title: String::new(),
            date: None,
            author: None,
            tags: None,
            categories: None,
            summary: None,
            cover_image: None,
            slug: None,
            draft: false,
            updated: None,
            layout: None,
        }
    }

    /// 从 YAML 字符串解析 Frontmatter
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    /// 检查是否为草稿
    pub fn is_draft(&self) -> bool {
        self.draft
    }

    /// 从文件名生成 slug
    /// 如果 metadata 中已有 slug，则使用已有的；否则从文件名生成
    pub fn get_slug(&self, filename: &str) -> String {
        if let Some(slug) = &self.slug {
            return slug.clone();
        }

        // 从文件名生成 slug
        let path = Path::new(filename);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(filename);

        // 移除日期前缀（如果存在，格式：YYYY-MM-DD-）
        let slug = if stem.len() > 11 && stem.chars().take(4).all(|c| c.is_ascii_digit()) {
            if let Some(dash_pos) = stem[4..].find('-') {
                if dash_pos == 4 && &stem[8..9] == "-" {
                    // 格式：YYYY-MM-DD-xxx
                    &stem[11..]
                } else {
                    stem
                }
            } else {
                stem
            }
        } else {
            stem
        };

        // 转换为小写并替换空格和特殊字符
        slug.to_lowercase()
            .replace(' ', "-")
            .replace('_', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect()
    }
}

impl Default for PostMetadata {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_yaml() {
        let yaml = r#"
title: "测试文章"
date: 2025-11-23
author: "测试作者"
tags: ["Rust", "Dioxus"]
draft: false
"#;
        let metadata = PostMetadata::from_yaml(yaml).unwrap();
        assert_eq!(metadata.title, "测试文章");
        assert_eq!(metadata.author, Some("测试作者".to_string()));
        assert_eq!(metadata.tags, Some(vec!["Rust".to_string(), "Dioxus".to_string()]));
        assert!(!metadata.is_draft());
    }

    #[test]
    fn test_get_slug() {
        let mut metadata = PostMetadata::new();
        metadata.title = "测试文章".to_string();
        
        // 测试从文件名生成 slug
        let slug = metadata.get_slug("2025-11-23-test-article.md");
        assert_eq!(slug, "test-article");
        
        // 测试使用已有的 slug
        metadata.slug = Some("custom-slug".to_string());
        let slug = metadata.get_slug("2025-11-23-test-article.md");
        assert_eq!(slug, "custom-slug");
    }
}

