use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::okr::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::okr::models::{PageResponse, PeriodRule},
};

/// 周期规则管理服务
pub struct PeriodRuleService {
    pub config: Config,
}

/// 周期规则列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodRuleListResponse {
    /// 规则列表
    #[serde(flatten)]
    pub rules: PageResponse<PeriodRule>,
}

impl ApiResponseTrait for PeriodRuleListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PeriodRuleService {
    /// 创建周期规则管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取 OKR 周期规则
    ///
    /// 查询指定周期的规则配置信息，包括评分规则、权限设置等。
    ///
    /// # Arguments
    ///
    /// * `request` - 查询请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回周期规则列表
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::okr::period_rule::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret")
    ///         .build();
    ///
    ///     let request = PeriodRuleListRequest {
    ///         period_id: Some("period_123".to_string()),
    ///         rule_type: Some("scoring".to_string()),
    ///         ..Default::default()
    ///     };
    ///
    ///     let response = client.okr.period_rule.list_period_rules(request, None).await?;
    ///     if let Some(data) = response.data {
    ///         for rule in data.rules.items {
    ///             println!("规则ID: {}, 类型: {:?}", rule.rule_id, rule.rule_type);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_period_rules(
        &self,
        request: PeriodRuleListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PeriodRuleListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: OKR_V1_PERIOD_RULES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(period_id) = request.period_id {
            api_req.query_params.insert("period_id", period_id);
        }

        if let Some(rule_type) = request.rule_type {
            api_req.query_params.insert("rule_type", rule_type);
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }
}

/// 周期规则查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PeriodRuleListRequest {
    /// 周期ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    /// 规则类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
