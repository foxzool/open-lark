use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints_original::WIKI_V2_SPACE_GET,
    error::SDKError,
    req_option::RequestOption,
    SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取知识空间信息请求
///
/// 用于根据知识空间ID查询知识空间的详细信息，包括空间名称、描述、
/// 创建时间、可见性等属性。
///
/// # 权限要求
/// 需要具备知识空间的成员权限（管理员或普通成员）
///
/// # 示例
/// ```rust
/// use open_lark::service::cloud_docs::wiki::v2::space::get::GetSpaceInfoRequest;
///
/// let request = GetSpaceInfoRequest::builder()
///     .space_id("6870403571079249922")
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct GetSpaceInfoRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 知识空间ID
    /// 目标知识空间的唯一标识符
    pub space_id: String,
}

impl Default for GetSpaceInfoRequest {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::default(),
            space_id: String::new(),
        }
    }
}

impl GetSpaceInfoRequest {
    /// 创建新的获取知识空间信息请求
    ///
    /// # 参数
    /// * `space_id` - 知识空间ID
    pub fn new(space_id: impl Into<String>) -> Self {
        Self {
            api_req: ApiRequest::default(),
            space_id: space_id.into(),
        }
    }

    /// 创建获取知识空间信息请求的构建器
    pub fn builder() -> GetSpaceInfoBuilder {
        GetSpaceInfoBuilder::default()
    }

    /// 构建请求验证
    ///
    /// 验证请求参数的有效性
    ///
    /// # 返回
    /// 成功返回空值，失败返回错误信息
    pub fn build(&self) -> SDKResult<()> {
        // 验证知识空间ID
        if self.space_id.trim().is_empty() {
            return Err(SDKError::ValidationError("知识空间ID不能为空".to_string()));
        }

        // 验证知识空间ID长度（飞书知识空间ID通常为数字字符串）
        if self.space_id.len() < 10 {
            return Err(SDKError::ValidationError("知识空间ID长度无效".to_string()));
        }

        // 验证知识空间ID字符（仅允许数字）
        if !self.space_id.chars().all(|c| c.is_ascii_digit()) {
            return Err(SDKError::ValidationError("知识空间ID包含无效字符".to_string()));
        }

        Ok(())
    }
}

/// 获取知识空间信息请求构建器
///
/// 提供链式调用接口来配置请求参数
///
/// # 示例
/// ```rust
/// use open_lark::service::cloud_docs::wiki::v2::space::get::GetSpaceInfoRequest;
///
/// let request = GetSpaceInfoRequest::builder()
///     .space_id("6870403571079249922")
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct GetSpaceInfoBuilder {
    request: GetSpaceInfoRequest,
}

impl GetSpaceInfoBuilder {
    /// 设置知识空间ID
    ///
    /// # 参数
    /// * `space_id` - 目标知识空间的唯一标识符
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.request.space_id = space_id.into();
        self
    }

    /// 构建请求对象
    ///
    /// # 返回
    /// 成功返回请求对象，失败返回验证错误
    ///
    /// # 错误
    /// * 知识空间ID为空或格式无效时返回验证错误
    pub fn build(self) -> SDKResult<GetSpaceInfoRequest> {
        self.request.build()?;
        Ok(self.request)
    }
}

/// 知识空间详细信息
///
/// 包含知识空间的完整属性信息，包括基本信息、权限设置、时间戳等
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpaceInfo {
    /// 知识空间ID
    /// 知识空间的唯一标识符
    pub space_id: String,
    /// 知识空间名称
    /// 知识空间的显示名称
    pub name: String,
    /// 知识空间描述
    /// 知识空间的详细描述信息，可能为空
    #[serde(default)]
    pub description: Option<String>,
    /// 知识空间类型
    /// personal(个人空间)、team(团队空间)
    #[serde(default)]
    pub space_type: Option<String>,
    /// 知识空间可见性
    /// private(私有)、public(公开)、partial_public(部分公开)
    #[serde(default)]
    pub visibility: Option<String>,
    /// 创建时间戳（秒）
    /// 知识空间创建的时间戳
    #[serde(default)]
    pub create_time: Option<i64>,
    /// 更新时间戳（秒）
    /// 知识空间最后更新的时间戳
    #[serde(default)]
    pub update_time: Option<i64>,
}

/// 获取知识空间信息响应
///
/// 包含请求成功后返回的知识空间详细信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSpaceInfoResponse {
    /// 知识空间信息
    /// 请求的知识空间的详细属性信息
    pub space: SpaceInfo,
}

impl ApiResponseTrait for GetSpaceInfoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间信息
///
/// 根据知识空间ID查询知识空间的详细信息，包括空间名称、描述、
/// 创建时间、可见性等属性。
///
/// # 参数
/// * `request` - 获取知识空间信息请求
/// * `config` - SDK配置信息
/// * `option` - 可选请求配置
///
/// # 返回
/// 成功返回知识空间信息响应，失败返回错误信息
///
/// # 权限要求
/// 需要具备目标知识空间的成员权限
///
/// # 示例
/// ```rust,no_run
/// use open_lark::service::cloud_docs::wiki::v2::space::get::{get_space_info, GetSpaceInfoRequest};
/// use open_lark::core::config::Config;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = openlark_core::config::Config::new("app_id", "app_secret");
///     let request = GetSpaceInfoRequest::new("6870403571079249922");
///
///     let response = get_space_info(request, &config, None).await?;
///     println!("知识空间名称: {}", response.data.space.name);
///     println!("知识空间描述: {:?}", response.data.space.description);
///
///     Ok(())
/// }
/// ```
pub async fn get_space_info(
    request: GetSpaceInfoRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<GetSpaceInfoResponse>> {
    // 创建API请求
    let mut api_req = request.api_req;
    api_req.set_http_method(Method::GET);

    // 设置API路径，替换路径参数
    let endpoint = WIKI_V2_SPACE_GET.replace("{}", &request.space_id);
    api_req.set_api_path(&endpoint);

    // 设置支持的访问令牌类型
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 发送HTTP请求
    let api_resp = openlark_core::http::Transport::request(api_req, config, option).await?;

    Ok(api_resp)
}

/// 获取知识空间信息构建器
///
/// 提供现代化的链式调用接口，支持与服务实例的集成
///
/// # 示例
/// ```rust,no_run
/// use open_lark::service::cloud_docs::wiki::v2::space::SpaceService;
/// use std::sync::Arc;
///
/// let service = Arc::new(SpaceService::new(config));
/// let response = service
///     .get_space_info_builder()
///     .space_id("6870403571079249922")
///     .execute()
///     .await?;
/// ```
pub struct GetSpaceInfoBuilder {
    service: std::sync::Arc<super::SpaceService>,
    request: GetSpaceInfoRequest,
}

impl GetSpaceInfoBuilder {
    /// 创建新的获取知识空间信息构建器
    ///
    /// # 参数
    /// * `service` - 知识空间服务实例
    /// * `request` - 获取知识空间信息请求
    pub fn new(service: std::sync::Arc<super::SpaceService>, request: GetSpaceInfoRequest) -> Self {
        Self { service, request }
    }

    /// 设置知识空间ID
    ///
    /// # 参数
    /// * `space_id` - 目标知识空间的唯一标识符
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.request.space_id = space_id.into();
        self
    }

    /// 执行获取知识空间信息请求
    ///
    /// # 返回
    /// 成功返回知识空间信息响应，失败返回错误信息
    ///
    /// # 错误
    /// * 知识空间ID无效时返回验证错误
    /// * 权限不足时返回授权错误
    /// * 网络错误时返回连接错误
    pub async fn execute(self) -> SDKResult<Response<GetSpaceInfoResponse>> {
        // 验证请求参数
        self.request.build()?;

        // 执行请求
        get_space_info(self.request, self.service.config(), None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_space_info_request_builder() {
        let request = GetSpaceInfoRequest::builder()
            .space_id("6870403571079249922")
            .build()
            .unwrap();

        assert_eq!(request.space_id, "6870403571079249922");
    }

    #[test]
    fn test_get_space_info_request_new() {
        let request = GetSpaceInfoRequest::new("6870403571079249922");
        assert_eq!(request.space_id, "6870403571079249922");
    }

    #[test]
    fn test_get_space_info_request_default() {
        let request = GetSpaceInfoRequest::default();
        assert_eq!(request.space_id, "");
    }

    #[test]
    fn test_get_space_info_request_validation() {
        // 测试有效的知识空间ID
        let valid_request = GetSpaceInfoRequest::new("6870403571079249922");
        assert!(valid_request.build().is_ok());

        // 测试空知识空间ID
        let empty_request = GetSpaceInfoRequest::new("");
        assert!(empty_request.build().is_err());

        // 测试过短的知识空间ID
        let short_request = GetSpaceInfoRequest::new("123456789");
        assert!(short_request.build().is_err());

        // 测试包含无效字符的知识空间ID
        let invalid_request = GetSpaceInfoRequest::new("6870403571079249922a");
        assert!(invalid_request.build().is_err());

        // 测试包含特殊字符的知识空间ID
        let special_char_request = GetSpaceInfoRequest::new("6870403571-079249922");
        assert!(special_char_request.build().is_err());
    }

    #[test]
    fn test_get_space_info_builder_validation() {
        // 测试有效构建器
        let valid_builder = GetSpaceInfoRequest::builder()
            .space_id("6870403571079249922")
            .build();
        assert!(valid_builder.is_ok());

        // 测试空知识空间ID构建器
        let empty_builder = GetSpaceInfoRequest::builder()
            .space_id("")
            .build();
        assert!(empty_builder.is_err());
    }

    #[test]
    fn test_space_info_serialization() {
        let space_info = SpaceInfo {
            space_id: "6870403571079249922".to_string(),
            name: "测试知识空间".to_string(),
            description: Some("这是一个测试知识空间".to_string()),
            space_type: Some("team".to_string()),
            visibility: Some("public".to_string()),
            create_time: Some(1609459200),
            update_time: Some(1609545600),
        };

        // 测试序列化
        let serialized = serde_json::to_string(&space_info).unwrap();
        assert!(serialized.contains("6870403571079249922"));
        assert!(serialized.contains("测试知识空间"));

        // 测试反序列化
        let deserialized: SpaceInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.space_id, space_info.space_id);
        assert_eq!(deserialized.name, space_info.name);
        assert_eq!(deserialized.description, space_info.description);
    }

    #[test]
    fn test_get_space_info_response() {
        let space_info = SpaceInfo {
            space_id: "6870403571079249922".to_string(),
            name: "测试知识空间".to_string(),
            description: Some("测试描述".to_string()),
            space_type: Some("team".to_string()),
            visibility: Some("public".to_string()),
            create_time: Some(1609459200),
            update_time: Some(1609545600),
        };

        let response = GetSpaceInfoResponse { space: space_info };

        // 测试响应结构
        assert_eq!(response.space.space_id, "6870403571079249922");
        assert_eq!(response.space.name, "测试知识空间");
        assert_eq!(response.space.description, Some("测试描述".to_string()));
    }

    #[test]
    fn test_response_format() {
        assert_eq!(GetSpaceInfoResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_multiple_space_requests() {
        // 测试多个不同的知识空间ID
        let space_ids = vec![
            "6870403571079249922",
            "6870403571079249923",
            "6870403571079249924",
        ];

        for space_id in space_ids {
            let request = GetSpaceInfoRequest::new(space_id);
            assert!(request.build().is_ok());
        }
    }

    #[test]
    fn test_edge_cases() {
        // 测试边界情况
        let edge_cases = vec![
            "0".repeat(19), // 19位数字（正常长度）
            "1".repeat(10), // 最小有效长度
        ];

        for case in edge_cases {
            let request = GetSpaceInfoRequest::new(case);
            assert!(request.build().is_ok());
        }

        // 测试无效长度
        let invalid_lengths = vec![
            "1".repeat(9), // 过短
            "1".repeat(25), // 过长
        ];

        for case in invalid_lengths {
            let request = GetSpaceInfoRequest::new(case);
            assert!(request.build().is_err());
        }
    }

    #[test]
    fn test_space_info_optional_fields() {
        // 测试可选字段为None的情况
        let space_info = SpaceInfo {
            space_id: "6870403571079249922".to_string(),
            name: "测试知识空间".to_string(),
            description: None,
            space_type: None,
            visibility: None,
            create_time: None,
            update_time: None,
        };

        // 序列化和反序列化
        let serialized = serde_json::to_string(&space_info).unwrap();
        let deserialized: SpaceInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.space_id, space_info.space_id);
        assert_eq!(deserialized.name, space_info.name);
        assert!(deserialized.description.is_none());
        assert!(deserialized.space_type.is_none());
        assert!(deserialized.visibility.is_none());
        assert!(deserialized.create_time.is_none());
        assert!(deserialized.update_time.is_none());
    }

    #[test]
    fn test_builder_pattern() {
        // 测试构建器模式的链式调用
        let request = GetSpaceInfoRequest::builder()
            .space_id("6870403571079249922")
            .build()
            .unwrap();

        assert_eq!(request.space_id, "6870403571079249922");
    }
}