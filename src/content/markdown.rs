use crate::content::PostMetadata;
use anyhow::{Context, Result};
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::Path;

/// 文章结构，包含元数据和内容
#[derive(Debug, Clone)]
pub struct Post {
    /// 文章元数据
    pub metadata: PostMetadata,
    /// Markdown 原始内容
    pub content: String,
    /// 渲染后的 HTML 内容
    pub html_content: String,
}

/// 解析 Markdown 文件，分离 Frontmatter 和正文
pub fn parse_markdown_file(path: &Path) -> Result<Post> {
    // 读取文件内容
    let content = fs::read_to_string(path)
        .with_context(|| format!("无法读取文件: {}", path.display()))?;

    // 分离 Frontmatter 和正文
    let (metadata, markdown_content) = parse_frontmatter(&content)
        .with_context(|| format!("解析 Frontmatter 失败: {}", path.display()))?;

    Ok(Post {
        metadata,
        content: markdown_content.clone(),
        html_content: String::new(), // 稍后渲染
    })
}

/// 渲染 Markdown 为 HTML
pub fn render_markdown(markdown: &str) -> String {
    // 启用所有扩展选项
    let options = Options::all();
    let parser = Parser::new_ext(markdown, options);
    
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    html_output
}

/// 处理 Markdown 文件：解析并渲染
pub fn process_markdown_file(path: &Path) -> Result<Post> {
    // 解析文件
    let mut post = parse_markdown_file(path)?;
    
    // 渲染 Markdown 为 HTML
    post.html_content = render_markdown(&post.content);
    
    Ok(post)
}

/// 解析 Frontmatter（YAML 格式，位于 --- 之间）
fn parse_frontmatter(content: &str) -> Result<(PostMetadata, String)> {
    // 检查是否以 --- 开头
    if !content.starts_with("---") {
        // 如果没有 Frontmatter，创建默认元数据
        return Ok((
            PostMetadata::new(),
            content.to_string(),
        ));
    }

    // 查找第二个 ---
    let end_marker = content[3..]
        .find("---")
        .context("Frontmatter 格式错误：未找到结束标记")?;

    // 提取 Frontmatter YAML
    let frontmatter_str = &content[3..end_marker + 3];
    
    // 解析 Frontmatter
    let metadata = PostMetadata::from_yaml(frontmatter_str)
        .context("Frontmatter YAML 解析失败")?;

    // 提取正文（跳过第二个 --- 和换行符）
    let body_start = end_marker + 6;
    let body = if body_start < content.len() {
        // 跳过可能的换行符
        let body = &content[body_start..];
        if body.starts_with('\n') {
            &body[1..]
        } else if body.starts_with("\r\n") {
            &body[2..]
        } else {
            body
        }
    } else {
        ""
    };

    Ok((metadata, body.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
title: "测试文章"
date: 2025-11-23
tags: ["Rust", "Dioxus"]
---

这是文章正文内容。
"#;
        let (metadata, body) = parse_frontmatter(content).unwrap();
        assert_eq!(metadata.title, "测试文章");
        assert!(body.contains("这是文章正文内容"));
    }

    #[test]
    fn test_render_markdown() {
        let markdown = "# 标题\n\n这是**粗体**文本。";
        let html = render_markdown(markdown);
        assert!(html.contains("<h1>标题</h1>"));
        assert!(html.contains("<strong>粗体</strong>"));
    }
}

