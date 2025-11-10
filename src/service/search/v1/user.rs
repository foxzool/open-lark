#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 搜索用户服务 V1
//!
//! 提供企业级的用户搜索功能，支持：
//! - 高精度用户搜索，支持多种搜索条件
//! - 分页查询和批量结果处理
//! - 迭代器模式简化大数据集处理
//! - 完整的Builder模式API设计
//! - 自动分页验证和错误处理
//!
//! # 核心功能
//!
//! ## 基础搜索
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::search::v1::user::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 基础搜索
//!     let response = client.search.v1.user
//!         .search_user_builder()
//!         .query("张三")
//!         .page_size(20)
//!         .execute(&client.search.v1.user)
//!         .await?;
//!
//!     println!("搜索到 {} 个用户", response.data?.users.len());
//!     Ok(())
//! }
//! ```
//!
//! ## 迭代器模式
//! ```rust,no_run
//! // 使用迭代器处理大量数据
//! let mut iter = client.search.v1.user.search_user_iter(
//!     SearchUserRequest::builder()
//!         .query("技术部")
//!         .page_size(50)
//!         .build(),
//!     None
//! );
//!
//! while let Some(users) = iter.next().await {
//!     println!("获取到 {} 个用户", users.len());
//!     for user in users {
//!         println!("用户: {} ({})", user.name, user.open_id);
//!     }
//! }
//! ```
//!
//! ## 安全分页搜索
//! ```rust,no_run
//! // 带自动验证的分页搜索
//! let response = client.search.v1.user
//!     .search_user_with_validated_pagination(
//!         "李四",
//!         Some(20),  // 页面大小
//!         None,     // 页面令牌
//!         None
//!     )
//!     .await?;
//! ```
//!
//! # API文档
//!
//! 详细API文档请参考：[飞书开放平台 - 搜索用户](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search)
// 导入核心依赖
use open_lark_core::api_req::ApiRequest;
use reqwest::Method;
use serde::{Deserialize, Serialize};

// 导入SDK核心模块
use crate::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

// 导入端点常量
use crate::service::endpoints::SEARCH_V1_USER;

/// 搜索用户服务 V1
///
/// 提供企业级的用户搜索功能，支持高精度的用户查询和分页处理。
/// 专为企业应用设计，具备完善的错误处理和性能优化。
///
/// # 主要特性
///
/// - **高精度搜索**：支持按姓名、邮箱、手机号等多种字段搜索
/// - **分页处理**：自动处理大数据集的分页逻辑
/// - **迭代器支持**：提供Stream API简化数据处理
/// - **Builder模式**：现代化的流式API设计
/// - **类型安全**：完整的Rust类型系统支持
///
/// # 使用示例
///
/// ```rust,no_run
/// use open_lark::prelude::*;
/// use open_lark::service::search::v1::user::UserService;
///
/// let config = Config::new("app_id", "app_secret");
/// let service = UserService::new(config);
/// ```
#[derive(Debug, Clone)]
pub struct UserService {
    pub config: Config,
}

impl UserService {
    /// 创建用户搜索服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息，包含应用ID、密钥等
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::search::v1::user::UserService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = UserService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 搜索用户
    ///
    /// 根据查询条件搜索用户，支持多种搜索字段和分页处理。
    /// 这是用户搜索的核心方法，提供完整的搜索功能。
    ///
    /// # API文档
    ///
    /// 搜索用户接口，支持按姓名、邮箱、手机号等字段进行模糊搜索。
    /// 返回匹配的用户列表和分页信息。
    ///
    /// # 参数
    ///
    /// * `request` - 搜索请求参数，包含查询条件、分页信息等
    /// * `option` - 可选的请求配置，如超时时间、重试次数等
    ///
    /// # 返回值
    ///
    /// 返回搜索结果，包含用户列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::search::v1::user::*;
    ///
    /// let request = SearchUserRequest::builder()
    ///     .query("张三")
    ///     .page_size(20)
    ///     .page_token("next_page_token")
    ///     .build();
    ///
    /// let response = client.search.v1.user.search(&request, None).await?;
    /// println!("搜索到 {} 个用户", response.data?.users.len());
    /// ```
    pub async fn search(
        &self,
        request: &SearchUserRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchUserResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(SEARCH_V1_USER.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::User]);
        api_req.query_params = request.to_query_params();

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 搜索用户 (返回BaseResponse供迭代器使用)
    ///
    /// 内部方法，用于迭代器实现。直接返回BaseResponse以便访问分页信息。
    async fn search_with_base_response(
        &self,
        request: &SearchUserRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchUserResponse>> {
        self.search(request, option).await
    }

    /// 搜索用户迭代器
    ///
    /// 创建一个迭代器来自动处理分页逻辑，简化大数据集的处理。
    /// 迭代器会自动获取下一页数据，直到所有数据都被处理完毕。
    ///
    /// # 参数
    ///
    /// * `request` - 搜索请求参数
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回用户搜索迭代器，可以通过`next()`方法逐页获取数据
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::search::v1::user::*;
    ///
    /// let request = SearchUserRequest::builder()
    ///     .query("技术部")
    ///     .page_size(50)
    ///     .build();
    ///
    /// let mut iter = client.search.v1.user.search_user_iter(request, None);
    ///
    /// while let Some(users) = iter.next().await {
    ///     for user in users {
    ///         println!("用户: {} ({})", user.name, user.open_id);
    ///     }
    /// }
    /// ```
    pub fn search_user_iter(
        &self,
        request: SearchUserRequest,
        option: Option<RequestOption>,
    ) -> SearchUserIterator<'_> {
        SearchUserIterator {
            user_service: self,
            request,
            option,
            has_more: true,
        }
    }

    /// 使用分页验证搜索用户
    ///
    /// 提供一个更安全的方式来搜索用户，自动验证分页参数的有效性。
    /// 确保页面大小在合理范围内，避免API调用失败。
    ///
    /// # 参数
    ///
    /// * `query` - 搜索关键词
    /// * `page_size` - 页面大小（可选，默认值会自动设置）
    /// * `page_token` - 分页令牌（可选，用于获取指定页面）
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回搜索结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// let response = client.search.v1.user
    ///     .search_user_with_validated_pagination(
    ///         "李四",
    ///         Some(20),
    ///         None,
    ///         None
    ///     )
    ///     .await?;
    ///
    /// println!("搜索到 {} 个用户", response.data?.users.len());
    /// ```
    pub async fn search_user_with_validated_pagination(
        &self,
        query: impl ToString,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchUserResponse>> {
        // 创建请求构建器
        let builder = SearchUserRequest::builder()
            .query(query.to_string())
            .with_pagination(page_size, page_token)?;

        self.search(&builder.build(), option).await
    }

    // ==================== Builder模式实现 ====================

    /// 搜索用户构建器
    ///
    /// 提供流式API来构建搜索用户的请求参数。
    /// 支持链式调用，方便构建复杂的搜索请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::search::v1::user::*;
    ///
    /// let builder = client.search.v1.user
    ///     .search_user_builder()
    ///     .query("张三")
    ///     .page_size(20)
    ///     .page_token("next_token");
    ///
    /// let response = builder.execute(&client.search.v1.user).await?;
    /// ```
    pub fn search_user_builder(&self) -> SearchUserRequestBuilder {
        SearchUserRequestBuilder::new()
    }
}

// ==================== 请求和响应结构体 ====================

/// 搜索用户请求
///
/// 包含搜索用户所需的所有参数，支持查询条件和分页配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchUserRequest {
    /// 搜索关键词，支持用户姓名、邮箱、手机号等字段的模糊搜索
    pub query: Option<String>,
    /// 页面大小，范围1-200，默认值为20
    pub page_size: Option<u32>,
    /// 分页令牌，用于获取指定页的数据
    pub page_token: Option<String>,
}

impl SearchUserRequest {
    /// 创建搜索用户请求构建器
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::search::v1::user::SearchUserRequest;
    ///
    /// let builder = SearchUserRequest::builder()
    ///     .query("张三")
    ///     .page_size(20);
    ///
    /// let request = builder.build();
    /// ```
    pub fn builder() -> SearchUserRequestBuilder {
        SearchUserRequestBuilder::new()
    }

    /// 将请求转换为查询参数
    fn to_query_params(&self) -> std::collections::HashMap<&'static str, String> {
        let mut params = std::collections::HashMap::new();

        if let Some(query) = &self.query {
            params.insert("query", query.clone());
        }

        if let Some(page_size) = self.page_size {
            params.insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = &self.page_token {
            params.insert("page_token", page_token.clone());
        }

        params
    }
}

/// 搜索用户请求构建器
///
/// 提供流式API来构建搜索用户的请求参数。
/// 支持链式调用，方便配置各种搜索条件。
#[derive(Debug, Clone, Default)]
pub struct SearchUserRequestBuilder {
    request: SearchUserRequest,
}

impl SearchUserRequestBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchUserRequest::default(),
        }
    }

    /// 设置搜索关键词
    ///
    /// # 参数
    /// - `query` - 搜索关键词，支持用户姓名、邮箱、手机号等字段
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::search::v1::user::SearchUserRequest;
    ///
    /// let builder = SearchUserRequest::builder()
    ///     .query("张三");
    /// ```
    pub fn query(mut self, query: impl ToString) -> Self {
        self.request.query = Some(query.to_string());
        self
    }

    /// 设置页面大小
    ///
    /// # 参数
    /// - `page_size` - 页面大小，范围1-200
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::search::v1::user::SearchUserRequest;
    ///
    /// let builder = SearchUserRequest::builder()
    ///     .page_size(20);
    /// ```
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页令牌
    ///
    /// # 参数
    /// - `page_token` - 分页令牌，用于获取指定页的数据
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::search::v1::user::SearchUserRequest;
    ///
    /// let builder = SearchUserRequest::builder()
    ///     .page_token("next_page_token");
    /// ```
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 使用分页验证构建器设置分页参数
    ///
    /// 这个方法提供了一个更安全的分页参数设置方式，会自动验证参数的有效性。
    /// 搜索服务的分页大小限制为 1-200。
    ///
    /// # 参数
    /// - `page_size` - 页面大小（可选）
    /// - `page_token` - 分页令牌（可选）
    ///
    /// # 返回值
    ///
    /// 返回设置好的Builder实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::search::v1::user::SearchUserRequest;
    ///
    /// let builder = SearchUserRequest::builder()
    ///     .query("技术部")
    ///     .with_pagination(Some(20), None)?;
    /// ```
    ///
    /// # 错误
    ///
    /// 如果页面大小超出限制（1-200），会返回错误
    pub fn with_pagination(
        mut self,
        page_size: Option<u32>,
        page_token: Option<String>,
    ) -> SDKResult<Self> {
        if let Some(size) = page_size {
            // 搜索服务有更严格的分页大小限制（1-200）
            if size > 200 {
                return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                    "Page size {} exceeds maximum limit of 200 for search service",
                    size,
                )));
            }
            if size == 0 {
                return Err(crate::core::error::LarkAPIError::illegal_param(
                    "Page size cannot be 0, minimum value is 1".to_string(),
                ));
            }
            self.request.page_size = Some(size);
        }

        if let Some(token) = page_token {
            self.request.page_token = Some(token);
        }

        Ok(self)
    }

    /// 构建最终的请求对象
    ///
    /// # 返回值
    ///
    /// 返回构建完成的SearchUserRequest实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::search::v1::user::SearchUserRequest;
    ///
    /// let request = SearchUserRequest::builder()
    ///     .query("张三")
    ///     .page_size(20)
    ///     .build();
    /// ```
    pub fn build(self) -> SearchUserRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    SearchUserRequestBuilder,
//    UserService,
//    SearchUserRequest,
//    BaseResponse<SearchUserResponse>,
//    search
//);

/// 搜索用户响应
///
/// 包含搜索结果和分页信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUserResponse {
    /// 搜索到的用户列表
    pub users: Vec<UserInSearchResponse>,
    /// 是否还有更多用户，值为 true 表示存在下一页
    pub has_more: bool,
    /// 分页标识，存在下一页的时候返回。下次请求带上此标识可以获取下一页的用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索到的用户信息
///
/// 包含用户的基本信息和头像数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInSearchResponse {
    /// 用户的头像 URL
    pub avatar: UserAvatar,
    /// 用户的部门信息
    pub department_ids: Vec<String>,
    /// 用户的姓名
    pub name: String,
    /// 用户的 open_id
    pub open_id: String,
    /// 用户的 user_id（兼容字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// 用户的头像信息
///
/// 提供多种尺寸的头像URL，适用于不同的显示场景。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAvatar {
    /// 用户的头像图片 URL，72×72px
    pub avatar_72: String,
    /// 用户的头像图片 URL，240×240px
    pub avatar_240: String,
    /// 用户的头像图片 URL，640×640px
    pub avatar_640: String,
    /// 用户的头像图片 URL，原始大小
    pub avatar_origin: String,
}

/// 搜索用户迭代器
///
/// 提供流式访问搜索结果的能力，自动处理分页逻辑。
/// 适用于处理大量用户数据的场景。
///
/// # 示例
///
/// ```rust,no_run
/// use open_lark::prelude::*;
/// use open_lark::service::search::v1::user::*;
///
/// let request = SearchUserRequest::builder()
///     .query("技术部")
///     .page_size(50)
///     .build();
///
/// let mut iter = client.search.v1.user.search_user_iter(request, None);
///
/// while let Some(users) = iter.next().await {
///     println!("获取到 {} 个用户", users.len());
///     for user in users {
///         println!("用户: {} ({})", user.name, user.open_id);
///     }
/// }
/// ```
pub struct SearchUserIterator<'a> {
    user_service: &'a UserService,
    request: SearchUserRequest,
    option: Option<RequestOption>,
    has_more: bool,
}

impl<'a> SearchUserIterator<'a> {
    /// 获取下一页用户数据
    ///
    /// # 返回值
    ///
    /// - `Some(Vec<UserInSearchResponse>)` - 下一页的用户列表
    /// - `None` - 没有更多数据
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// while let Some(users) = iter.next().await {
    ///     for user in users {
    ///         println!("用户: {}", user.name);
    ///     }
    /// }
    /// ```
    pub async fn next(&mut self) -> Option<Vec<UserInSearchResponse>> {
        if !self.has_more {
            return None;
        }

        match self
            .user_service
            .search_with_base_response(&self.request, self.option.clone())
            .await
        {
            Ok(resp) => match resp.data {
                Some(data) => {
                    self.has_more = data.has_more;

                    if data.has_more {
                        if let Some(token) = data.page_token {
                            self.request.page_token = Some(token);
                        } else {
                            // has_more is true but no page_token. Stop iterating to avoid panic.
                            self.has_more = false;
                        }
                    }

                    if data.users.is_empty() {
                        None
                    } else {
                        Some(data.users)
                    }
                }
                None => None,
            },
            Err(e) => {
                log::error!("搜索用户迭代器错误: {:?}", e);
                None
            }
        }
    }
}
