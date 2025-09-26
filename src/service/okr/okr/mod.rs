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
    service::okr::models::{Okr, OkrStatus, PageResponse},
};

/// OKR 内容管理服务
pub struct OkrContentService {
    pub config: Config,
}

/// 用户 OKR 列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserOkrListResponse {
    /// OKR 列表
    #[serde(flatten)]
    pub okrs: PageResponse<Okr>,
}

impl ApiResponseTrait for UserOkrListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量 OKR 响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOkrResponse {
    /// OKR 列表
    pub okrs: Vec<Okr>,
}

impl ApiResponseTrait for BatchOkrResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl OkrContentService {
    /// 创建 OKR 内容管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取用户的 OKR 列表
    ///
    /// 查询指定用户在指定周期的 OKR 列表，包括 Objective 和 Key Result 信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 查询请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回用户的 OKR 列表
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::okr::okr::*;
    /// use open_lark::service::okr::models::OkrStatus;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret")
    ///         .build();
    ///
    ///     let request = UserOkrListRequest {
    ///         user_id: Some("user_123".to_string()),
    ///         period_id: Some("period_456".to_string()),
    ///         status: Some(OkrStatus::Normal),
    ///         ..Default::default()
    ///     };
    ///
    ///     let response = client.okr.okr.list_user_okrs(request, None).await?;
    ///     if let Some(data) = response.data {
    ///         for okr in data.okrs.items {
    ///             println!("OKR ID: {}, 用户: {}", okr.okr_id, okr.user_id);
    ///             if let Some(objectives) = okr.objectives {
    ///                 for obj in objectives {
    ///                     println!("  目标: {:?}", obj.content);
    ///                 }
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_user_okrs(
        &self,
        request: UserOkrListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserOkrListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: OKR_V1_OKRS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(user_id) = request.user_id {
            api_req.query_params.insert("user_id", user_id);
        }

        if let Some(period_id) = request.period_id {
            api_req.query_params.insert("period_id", period_id);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", format!("{status:?}"));
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

    /// 批量获取 OKR
    ///
    /// 根据提供的 OKR ID 列表批量获取 OKR 详细信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 批量查询请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回 OKR 列表
    pub async fn batch_get_okrs(
        &self,
        request: BatchOkrRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchOkrResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: OKR_V1_OKRS_BATCH_GET.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 用户 OKR 列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserOkrListRequest {
    /// 用户ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 周期ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    /// 状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OkrStatus>,
    /// 页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 批量获取 OKR 请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOkrRequest {
    /// OKR ID 列表
    pub okr_ids: Vec<String>,
    /// 是否包含详细信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_details: Option<bool>,
}
