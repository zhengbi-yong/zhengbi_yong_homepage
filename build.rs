use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// 简化的文章元数据结构（用于构建时）
#[derive(serde::Serialize, Clone)]
struct PostMetadataJson {
    title: String,
    date: Option<String>,
    author: Option<String>,
    tags: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    summary: Option<String>,
    cover_image: Option<String>,
    slug: String,
    draft: bool,
    updated: Option<String>,
    layout: Option<String>,
    file_path: String,
}

/// 文章索引 JSON 结构
#[derive(serde::Serialize)]
struct PostIndexJson {
    posts: Vec<PostMetadataJson>,
    tags: HashMap<String, Vec<usize>>,
    categories: HashMap<String, Vec<usize>>,
    sorted_by_date: Vec<usize>,
}

fn main() {
    println!("cargo:rerun-if-changed=blogs");
    
    // 检查 blogs 目录是否存在
    let blogs_dir = Path::new("blogs");
    if !blogs_dir.exists() {
        println!("cargo:warning=blogs 目录不存在，跳过内容处理");
        // 创建空的索引文件
        create_empty_index();
        return;
    }
    
    // 扫描并处理 Markdown 文件
    match scan_and_process_blogs(blogs_dir) {
        Ok(index) => {
            // 生成 JSON 索引文件
            if let Err(e) = write_index_json(&index) {
                eprintln!("cargo:warning=无法写入索引文件: {}", e);
            } else {
                println!("cargo:warning=成功生成文章索引，共 {} 篇文章", index.posts.len());
            }
        }
        Err(e) => {
            eprintln!("cargo:warning=处理 blogs 目录时出错: {}", e);
            create_empty_index();
        }
    }
}

/// 扫描并处理 blogs 目录
fn scan_and_process_blogs(blogs_dir: &Path) -> Result<PostIndexJson, Box<dyn std::error::Error>> {
    let mut index = PostIndexJson {
        posts: Vec::new(),
        tags: HashMap::new(),
        categories: HashMap::new(),
        sorted_by_date: Vec::new(),
    };
    
    let mut posts_with_dates: Vec<(usize, Option<chrono::NaiveDate>)> = Vec::new();
    
    // 遍历 blogs 目录
    for entry in WalkDir::new(blogs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|ext| ext == "md")
                .unwrap_or(false)
        })
    {
        let file_path = entry.path();
        
        // 处理 Markdown 文件
        match parse_markdown_frontmatter(file_path) {
            Ok(Some(metadata)) => {
                // 跳过草稿
                if metadata.draft {
                    continue;
                }
                
                let post_idx = index.posts.len();
                index.posts.push(metadata.clone());
                
                // 构建标签索引
                if let Some(ref tags) = metadata.tags {
                    for tag in tags {
                        index
                            .tags
                            .entry(tag.clone())
                            .or_insert_with(Vec::new)
                            .push(post_idx);
                    }
                }
                
                // 构建分类索引
                if let Some(ref categories) = metadata.categories {
                    for category in categories {
                        index
                            .categories
                            .entry(category.clone())
                            .or_insert_with(Vec::new)
                            .push(post_idx);
                    }
                }
                
                // 记录日期用于排序
                let date = metadata.date.as_ref().and_then(|d| {
                    chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()
                });
                posts_with_dates.push((post_idx, date));
            }
            Ok(None) => {
                // 没有 frontmatter，跳过
                continue;
            }
            Err(e) => {
                eprintln!(
                    "cargo:warning=处理文件失败 {}: {}",
                    file_path.display(),
                    e
                );
                // 继续处理其他文件
            }
        }
    }
    
    // 按日期排序（从新到旧）
    posts_with_dates.sort_by(|a, b| {
        match (a.1, b.1) {
            (Some(date_a), Some(date_b)) => date_b.cmp(&date_a),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
    index.sorted_by_date = posts_with_dates.into_iter().map(|(idx, _)| idx).collect();
    
    Ok(index)
}

/// 解析 Markdown 文件的 Frontmatter
fn parse_markdown_frontmatter(
    path: &Path,
) -> Result<Option<PostMetadataJson>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    
    // 检查是否以 --- 开头
    if !content.starts_with("---") {
        return Ok(None);
    }
    
    // 查找第二个 ---
    let end_marker = content[3..]
        .find("---")
        .ok_or("Frontmatter 格式错误：未找到结束标记")?;
    
    // 提取 Frontmatter YAML
    let frontmatter_str = &content[3..end_marker + 3];
    
    // 解析 YAML
    let metadata: serde_yaml::Value = serde_yaml::from_str(frontmatter_str)?;
    
    // 提取字段
    let title = metadata
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("未命名文章")
        .to_string();
    
    let date = metadata
        .get("date")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let author = metadata
        .get("author")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let tags = metadata
        .get("tags")
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        });
    
    let categories = metadata
        .get("categories")
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        });
    
    let summary = metadata
        .get("summary")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let cover_image = metadata
        .get("cover_image")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let slug = metadata
        .get("slug")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            // 从文件名生成 slug
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("untitled");
            
            // 移除日期前缀（如果存在）
            let slug = if stem.len() > 11 && stem.chars().take(4).all(|c| c.is_ascii_digit()) {
                if let Some(dash_pos) = stem[4..].find('-') {
                    if dash_pos == 0 && stem.len() > 10 && &stem[7..8] == "-" {
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
            
            slug.to_lowercase()
                .replace(' ', "-")
                .replace('_', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect()
        });
    
    let draft = metadata
        .get("draft")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    let updated = metadata
        .get("updated")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let layout = metadata
        .get("layout")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let file_path = path
        .strip_prefix("blogs")
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    
    Ok(Some(PostMetadataJson {
        title,
        date,
        author,
        tags,
        categories,
        summary,
        cover_image,
        slug,
        draft,
        updated,
        layout,
        file_path,
    }))
}

/// 写入索引 JSON 文件
fn write_index_json(index: &PostIndexJson) -> Result<(), Box<dyn std::error::Error>> {
    // 确保 assets 目录存在
    let assets_dir = Path::new("assets");
    if !assets_dir.exists() {
        fs::create_dir_all(assets_dir)?;
    }
    
    // 写入 JSON 文件
    let json_path = assets_dir.join("posts_index.json");
    let json_content = serde_json::to_string_pretty(index)?;
    fs::write(&json_path, json_content)?;
    
    println!("cargo:warning=索引文件已生成: {}", json_path.display());
    
    Ok(())
}

/// 创建空的索引文件
fn create_empty_index() {
    let index = PostIndexJson {
        posts: Vec::new(),
        tags: HashMap::new(),
        categories: HashMap::new(),
        sorted_by_date: Vec::new(),
    };
    
    if let Err(e) = write_index_json(&index) {
        eprintln!("cargo:warning=无法创建空索引文件: {}", e);
    }
}
