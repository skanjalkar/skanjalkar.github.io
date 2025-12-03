mod storage;
mod markdown;

pub use storage::{get_stored_theme, store_theme};
pub use markdown::parse_markdown;