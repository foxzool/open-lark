use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::report::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::report::models::{PageResponse, ReportRule},
};

/// 规则管理服务
pub struct RuleService {
    pub config: Config,
}

impl RuleService {
    /// 创建规则管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询规则
    ///
    /// 查询汇报规则列表，支持分页和条件筛选。
    ///
    /// # Arguments
    ///
    /// * `request` - 规则查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回规则列表
    pub async fn query(
        &self,
        request: RuleQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleQueryResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: REPORT_V1_RULES_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(rule_type) = request.rule_type {
            api_req.query_params.insert("rule_type", rule_type);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(creator) = request.creator {
            api_req.query_params.insert("creator", creator);
        }

        if let Some(name) = request.name {
            api_req.query_params.insert("name", name);
        }

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert("start_time", start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert("end_time", end_time.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }
}

/// 规则查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleQueryRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 规则类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建者筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 规则名称筛选（支持模糊匹配）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 开始时间戳筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间戳筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// 规则查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleQueryResponse {
    /// 规则列表
    #[serde(flatten)]
    pub rules: PageResponse<ReportRule>,
}

impl ApiResponseTrait for RuleQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
