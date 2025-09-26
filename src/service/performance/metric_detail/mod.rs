use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::performance::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::performance::models::MetricDetail,
};

/// 指标数据管理服务
pub struct MetricDetailService {
    pub config: Config,
}

impl MetricDetailService {
    /// 创建指标数据管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取被评估人关键指标结果
    ///
    /// 查询指定被评估人在指定项目中的关键指标数据。
    ///
    /// # Arguments
    ///
    /// * `request` - 指标详情查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回指标详情列表
    pub async fn query_metric_details(
        &self,
        request: MetricDetailQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricDetailQueryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_V1_METRIC_DETAIL_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 录入被评估人关键指标数据
    ///
    /// 批量录入或更新被评估人的关键指标数据。
    ///
    /// # Arguments
    ///
    /// * `request` - 指标数据导入请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回导入结果
    pub async fn import_metric_details(
        &self,
        request: MetricDetailImportRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricDetailImportResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_V1_METRIC_DETAIL_IMPORT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 指标详情查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDetailQueryRequest {
    /// 项目ID
    pub activity_id: String,
    /// 被评估人ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewee_ids: Option<Vec<String>>,
    /// 指标ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_ids: Option<Vec<String>>,
}

/// 指标详情查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricDetailQueryResponse {
    /// 指标详情列表
    pub metric_details: Vec<MetricDetail>,
}

impl ApiResponseTrait for MetricDetailQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 指标数据导入请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDetailImportRequest {
    /// 项目ID
    pub activity_id: String,
    /// 指标数据列表
    pub metric_details: Vec<MetricDetail>,
}

/// 指标数据导入响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricDetailImportResponse {
    /// 导入成功标识
    pub success: bool,
    /// 导入数量
    pub imported_count: i32,
    /// 失败信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<String>>,
}

impl ApiResponseTrait for MetricDetailImportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
