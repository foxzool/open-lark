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
    service::acs::models::{Device, DeviceStatus, DeviceType, PageResponse},
};

/// 设备管理服务
pub struct DeviceService {
    pub config: Config,
}

impl DeviceService {
    /// 创建设备管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取门禁设备列表
    ///
    /// 分页获取门禁设备列表，支持多种筛选条件。
    ///
    /// # Arguments
    ///
    /// * `request` - 设备列表查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回设备列表
    pub async fn list_devices(
        &self,
        request: DeviceListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeviceListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/acs/v1/devices".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }

        if let Some(device_type) = request.device_type {
            api_req.query_params.insert(
                "device_type".to_string(),
                serde_json::to_string(&device_type)?,
            );
        }

        if let Some(status) = request.status {
            api_req
                .query_params
                .insert("status".to_string(), serde_json::to_string(&status)?);
        }

        if let Some(location) = request.location {
            api_req
                .query_params
                .insert("location".to_string(), location);
        }

        Transport::request(api_req, &self.config, option).await
    }
}

/// 设备列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 设备类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<DeviceType>,
    /// 设备状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DeviceStatus>,
    /// 设备位置筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// 设备列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceListResponse {
    /// 设备列表
    #[serde(flatten)]
    pub devices: PageResponse<Device>,
}

impl ApiResponseTrait for DeviceListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
