//! 帖子（post）

pub mod get;

// get 模块显式导出

pub use get::{GetPostRequest, GetPostResponse, Post};
