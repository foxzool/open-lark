/// 通用模块
///
/// 提供openlark-docs项目中通用的工具、宏和类型定义。
pub mod api_endpoints;
pub mod api_paths;
pub mod api_utils;
pub mod builders;

// 重新导出API端点枚举
pub use api_endpoints::{BaseApiV2, BitableApiV1, MinutesApiV1, SheetsApiV3};

/// 通用常量定义
pub mod constants {
    /// 默认分页大小
    pub const DEFAULT_PAGE_SIZE: i32 = 20;
    /// 最大分页大小
    pub const MAX_PAGE_SIZE: i32 = 100;
    /// 默认超时时间（秒）
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
}

/// 通用类型别名
pub mod types {
    /// 通用字符串类型
    pub type AppToken = String;
    pub type TableId = String;
    pub type RecordId = String;
    pub type FormId = String;
    pub type ViewId = String;
    pub type FieldId = String;
    pub type RoleId = String;
    pub type UserId = String;
}

/// 批量操作相关类型
pub mod batch {
    use serde::{Deserialize, Serialize};

    /// 通用批量操作参数
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct BatchCommonParams {
        /// 用户ID类型
        pub user_id_type: Option<String>,
        /// 客户端令牌
        pub client_token: Option<String>,
    }

    impl BatchCommonParams {
        pub fn new() -> Self {
            Self::default()
        }

        /// 设置用户ID类型
        pub fn with_user_id_type(mut self, user_id_type: impl ToString) -> Self {
            self.user_id_type = Some(user_id_type.to_string());
            self
        }

        /// 设置客户端令牌
        pub fn with_client_token(mut self, client_token: impl ToString) -> Self {
            self.client_token = Some(client_token.to_string());
            self
        }
    }

    /// 通用批量请求体
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct BatchCommonBody {
        /// 请求列表
        pub requests: Vec<serde_json::Value>,
    }

    /// 通用批量操作结果
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct BatchOperationResult {
        /// 成功标识
        pub success: bool,
        /// 结果列表
        pub results: Vec<BatchItemResult>,
        /// 错误信息（如果有）
        pub error: Option<String>,
    }

    impl BatchOperationResult {
        /// 创建成功的结果
        pub fn success() -> Self {
            Self {
                success: true,
                results: Vec::new(),
                error: None,
            }
        }

        /// 创建失败的结果
        pub fn failure(error: impl ToString) -> Self {
            Self {
                success: false,
                results: Vec::new(),
                error: Some(error.to_string()),
            }
        }
    }

    /// 批量操作中的单项结果
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct BatchItemResult {
        /// 成功标识
        pub success: bool,
        /// 数据（如果有）
        pub data: Option<serde_json::Value>,
        /// 错误信息（如果有）
        pub error: Option<String>,
    }
}

/// 通用特征定义
pub mod traits {
    /// API请求基础特征
    pub trait ApiRequest {
        type Response;
        fn validate(&self) -> openlark_core::SDKResult<()>;
        fn build_path(&self) -> String;
    }

    /// 可分页请求特征
    pub trait PaginatedRequest {
        fn page_token(self, token: impl Into<String>) -> Self;
        fn page_size(self, size: i32) -> Self;
    }

    /// 可筛选请求特征
    pub trait FilterableRequest {
        fn add_filter(self, filter: serde_json::Value) -> Self;
    }
}
