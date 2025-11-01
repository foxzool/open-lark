use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
/// 文档版本服务,
pub struct FileVersionService {
    config: Config}
impl FileVersionService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建文档版本,
    ///,
/// 该接口用于创建文档版本。,
    ///,
/// # API文档,
    ///,
/// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme,
    pub async fn create_version(
        &self,
        request: CreateVersionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateVersionRespData>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: DRIVE_V1_FILE_VERSIONS.replace("{}", &request.file_token),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant]
            ..Default::default()
};
let body = serde_json::json!({,
            "name": request.name,
            "obj_type": request.obj_type});
api_req.body = serde_json::to_vec(&body)?;
        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 删除文档版本,
    ///,
/// 该接口用于删除文档版本。,
    ///,
/// # API文档,
    ///,
/// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme,
    pub async fn delete_version(
        &self,
        request: DeleteVersionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteVersionRespData>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::DELETE,
            api_path: DRIVE_V1_FILE_VERSION_GET
                .replace("{}", &request.file_token)
                .replace()
            ..Default::default()
};
api_req
            .set_supported_access_token_types(vec![AccessTokenType::User, AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 获取文档版本,
    ///,
/// 该接口用于获取文档版本信息。,
    ///,
/// # API文档,
    ///,
/// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme,
    pub async fn get_version(
        &self,
        request: GetVersionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetVersionRespData>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: DRIVE_V1_FILE_VERSION_GET
                .replace("{}", &request.file_token)
                .replace()
            ..Default::default()
};
api_req
            .set_supported_access_token_types(vec![AccessTokenType::User, AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 获取文档版本列表,
    ///,
/// 该接口用于获取文档的版本列表。,
    ///,
/// # API文档,
    ///,
/// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme,
    pub async fn list_versions(
        &self,
        request: ListVersionsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListVersionsRespData>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: DRIVE_V1_FILE_VERSIONS.replace("{}", &request.file_token),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant]
            ..Default::default()
};
// 添加查询参数,
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
if let Some(page_size) = request.page_size {,
            api_req
.query_params
                .insert("page_size", page_size.to_string());

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
// === 数据结构定义 ===,
/// 创建文档版本请求参数
#[derive(Debug, Clone)]
pub struct CreateVersionRequest {
    /// 文档token
    pub file_token: String,
    /// 版本名称
    pub name: String,
    /// 文档类型
    pub obj_type: String,
impl CreateVersionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建文档版本请求构建器,
#[derive(Default)]
pub struct CreateVersionRequestBuilder {
    request: CreateVersionRequest}
impl CreateVersionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    CreateVersionRequestBuilder,
    FileVersionService,
    CreateVersionRequest,
    BaseResponse<CreateVersionRespData>,
    create_version,
);
/// 创建文档版本响应数据
#[derive(Debug, Clone)]
pub struct CreateVersionRespData {
    /// 版本ID
    pub version_id: String,
    /// 版本名称
    pub name: String,
    /// 版本创建时间
    pub create_time: String,
    /// 创建者ID
    pub creator_id: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 删除文档版本请求参数,
#[derive(Debug, Clone)]
pub struct DeleteVersionRequest {
    /// 文档token
    pub file_token: String,
    /// 版本ID
    pub version_id: String,
impl DeleteVersionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除文档版本请求构建器,
#[derive(Default)]
pub struct DeleteVersionRequestBuilder {
    request: DeleteVersionRequest}
impl DeleteVersionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    DeleteVersionRequestBuilder,
    FileVersionService,
    DeleteVersionRequest,
    BaseResponse<DeleteVersionRespData>,
    delete_version,
);
/// 删除文档版本响应数据
#[derive(Debug, Clone)]
pub struct DeleteVersionRespData {
    /// 删除结果
    pub success: bool,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 获取文档版本请求参数,
#[derive(Debug, Clone)]
pub struct GetVersionRequest {
    /// 文档token
    pub file_token: String,
    /// 版本ID
    pub version_id: String,
impl GetVersionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取文档版本请求构建器,
#[derive(Default)]
pub struct GetVersionRequestBuilder {
    request: GetVersionRequest}
impl GetVersionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    GetVersionRequestBuilder,
    FileVersionService,
    GetVersionRequest,
    BaseResponse<GetVersionRespData>,
    get_version,
);
/// 获取文档版本响应数据
#[derive(Debug, Clone)]
pub struct GetVersionRespData {
    /// 版本信息
    pub version: FileVersion,
/// 获取文档版本列表请求参数,
#[derive(Debug, Clone)]
pub struct ListVersionsRequest {
    /// 文档token
    pub file_token: String,
    /// 分页token
    pub page_token: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>}
impl ListVersionsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}
    pub fn with_page_token(mut self, page_token: impl Into<String>) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.page_token = Some(page_token.into());
        self}

    pub fn with_page_size(mut self, page_size: i32) -> Self {
self.page_size = Some(page_size);
        self}
/// 获取文档版本列表请求构建器,
#[derive(Default)]
pub struct ListVersionsRequestBuilder {
    request: ListVersionsRequest}
impl ListVersionsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    ListVersionsRequestBuilder,
    FileVersionService,
    ListVersionsRequest,
    BaseResponse<ListVersionsRespData>,
    list_versions,
);
/// 获取文档版本列表响应数据
#[derive(Debug, Clone)]
pub struct ListVersionsRespData {
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页token
    pub page_token: Option<String>,
    /// 版本列表
    pub items: Vec<FileVersion>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 文档版本信息,
#[derive(Debug, Clone)]
pub struct FileVersion {
    /// 版本ID
    pub version_id: String,
    /// 版本名称
    pub name: String,
    /// 文档token
    pub obj_token: String,
    /// 文档类型
    pub obj_type: String,
    /// 创建者ID
    pub creator_id: String,
    /// 创建时间
    pub create_time: String,
    /// 版本状态
    pub status: String,
    /// 版本父ID
    pub parent_version_id: Option<String>}
