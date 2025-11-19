//! 认证管理模块
//!
//! 提供令牌管理、缓存、刷新和验证功能

pub mod cache;
pub mod refresh;
pub mod token;
pub mod validator;

// Re-export commonly used types
pub use cache::{CacheConfig, CacheStats, MemoryTokenCache, TokenStorage};
pub use refresh::{RefreshTokenResponse, TokenRefresher};
pub use token::{TokenInfo, TokenManager, TokenRefreshConfig, TokenType, TokenValidationResult};
pub use validator::TokenValidator;
