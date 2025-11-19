//! 存储层模块
//!
//! 提供缓存、内存存储和存储特征定义

pub mod cache;
pub mod memory;
pub mod traits;

// Re-export commonly used types
pub use cache::{Cache, CacheConfig};
pub use memory::{MemoryCache, MemoryCacheBuilder};
pub use traits::{Storage, StorageError, StorageResult};