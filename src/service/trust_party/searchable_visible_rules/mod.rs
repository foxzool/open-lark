use crate::core::SDKResult;use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use crate::,
{,
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait,
    config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints,
};
        http::Transport,
        query_params::QueryParams,
        req_option::RequestOption,
        SDKResult,
    }
    service::trust_party::models::{PageResponse, RuleConfig, SearchableVisibleRule}
};
/// 可搜可见规则管理服务
#[derive(Debug, Clone)]
pub struct SearchableVisibleRulesService {
}

impl SearchableVisibleRulesService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 新增可搜可见规则
    ///,
/// 创建新的可搜可见规则，用于控制用户在关联组织中的可见性和搜索权限。
    ///,
/// # Arguments
    ///,
/// * `request` - 规则创建请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回创建的规则信息
pub async fn create_rule(,
        &self,
        request: RuleCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleCreateResponse>> {,
let mut api_req = ApiRequest::default();
        api_req.set_api_path(Endpoints::TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
api_req.set_body(serde_json::to_vec(&request)?);
        Transport::request(api_req, &self.config, option).await,
/// 更新可搜可见规则
    ///,
/// 更新指定的可搜可见规则配置。
    ///,
/// # Arguments
    ///,
/// * `rule_id` - 规则ID
    /// * `request` - 规则更新请求
/// * `option` - 请求选项，可选
    ///,
/// # Returns
    ///,
/// 返回更新后的规则信息
    pub async fn update_rule(
        &self,
        rule_id: &str,
        request: RuleUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleUpdateResponse>> {,
let mut api_req = ApiRequest::default();
        api_req.set_api_path(EndpointBuilder::replace_param(
            Endpoints::TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULE_OPERATION,
            "rule_id",
            rule_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
api_req.set_body(serde_json::to_vec(&request)?);
        Transport::request(api_req, &self.config, option).await,
/// 查询可搜可见规则
    ///,
/// 查询可搜可见规则列表，支持分页和条件筛选。
    ///,
/// # Arguments
    ///,
/// * `request` - 规则查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回规则列表
pub async fn list_rules(,
        &self,
        request: RuleListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleListResponse>> {,
let mut api_req = ApiRequest::default();
        api_req.set_api_path(Endpoints::TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
// 添加查询参数
        if let Some(page_token) = request.page_token {,
api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
if let Some(page_size) = request.page_size {,
            api_req
.query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
if let Some(org_id) = request.org_id {,
            api_req.query_params.insert(QueryParams::ORG_ID, org_id);
if let Some(rule_type) = request.rule_type {,
            api_req
.query_params
                .insert(QueryParams::RULE_TYPE, rule_type);
if let Some(status) = request.status {,
            api_req.query_params.insert(QueryParams::STATUS, status);
        Transport::request(api_req, &self.config, option).await,
/// 删除可搜可见规则
    ///,
/// 删除指定的可搜可见规则。
    ///,
/// # Arguments
    ///,
/// * `rule_id` - 规则ID
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回删除结果
pub async fn delete_rule(,
        &self,
        rule_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleDeleteResponse>> {,
let mut api_req = ApiRequest::default();
        api_req.set_api_path(EndpointBuilder::replace_param(
            Endpoints::TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULE_OPERATION,
            "rule_id",
            rule_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        Transport::request(api_req, &self.config, option).await,
/// 规则创建请求
#[derive(Debug, Clone)]
pub struct RuleCreateRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 规则更新请求
#[derive(Debug, Clone)]
pub struct RuleUpdateRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 规则列表查询请求
#[derive(Debug, Clone)]
pub struct RuleListRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 规则删除响应
#[derive(Debug, Clone)]
pub struct RuleDeleteResponse {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}}}}}}}}}}}}}