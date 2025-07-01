use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::acs::models::Visitor,
};

/// 访客管理服务
pub struct VisitorService {
    pub config: Config,
}

impl VisitorService {
    /// 创建访客管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 添加访客
    ///
    /// 添加新的访客记录，包括访客基本信息和访问权限。
    ///
    /// # Arguments
    ///
    /// * `request` - 访客添加请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回添加的访客信息
    pub async fn create_visitor(
        &self,
        request: VisitorCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<VisitorCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/acs/v1/visitors".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除访客
    ///
    /// 删除指定的访客记录，撤销访客的门禁权限。
    ///
    /// # Arguments
    ///
    /// * `visitor_id` - 访客ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回删除结果
    pub async fn delete_visitor(
        &self,
        visitor_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<VisitorDeleteResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: format!("/open-apis/acs/v1/visitors/{visitor_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 访客添加请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitorCreateRequest {
    /// 访客姓名
    pub name: String,
    /// 访客电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 访客公司
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 访问目的
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// 接待人员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_user_id: Option<String>,
    /// 访问开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 访问结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 允许访问的设备ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<String>>,
}

/// 访客添加响应
#[derive(Debug, Serialize, Deserialize)]
pub struct VisitorCreateResponse {
    /// 添加的访客信息
    pub visitor: Visitor,
}

impl ApiResponseTrait for VisitorCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 访客删除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct VisitorDeleteResponse {
    /// 删除成功标识
    pub success: bool,
}

impl ApiResponseTrait for VisitorDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
