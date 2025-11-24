pub mod metadata;
pub mod markdown;
pub mod index;

use include_dir::{include_dir, Dir};

// 强制重新编译以包含最新的博客文章
pub static BLOGS_DIR: Dir = include_dir!("blogs");

pub use metadata::PostMetadata;
pub use markdown::{Post, parse_markdown_file, parse_markdown_content, render_markdown, process_markdown_file};
pub use index::{PostIndex, scan_blogs_directory, load_embedded_blogs};

