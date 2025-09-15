use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        query_params::QueryParams,
        req_option::RequestOption,
        SDKResult,
    },
    service::acs::models::{
        AccessMethod, AccessRecord, AccessResult, AccessType, FaceImage, PageResponse,
    },
};

/// 访问记录服务
pub struct AccessRecordService {
    pub config: Config,
}

impl AccessRecordService {
    /// 创建访问记录服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取门禁记录列表
    ///
    /// 分页获取门禁访问记录列表，支持多种筛选条件。
    ///
    /// # Arguments
    ///
    /// * `request` - 访问记录查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回访问记录列表
    pub async fn list_access_records(
        &self,
        request: AccessRecordListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AccessRecordListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::ACS_V1_ACCESS_RECORDS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
        }

        if let Some(device_id) = request.device_id {
            api_req
                .query_params
                .insert(QueryParams::DEVICE_ID, device_id);
        }

        if let Some(access_type) = request.access_type {
            api_req.query_params.insert(
                QueryParams::ACCESS_TYPE,
                serde_json::to_string(&access_type)?,
            );
        }

        if let Some(access_method) = request.access_method {
            api_req.query_params.insert(
                QueryParams::ACCESS_METHOD,
                serde_json::to_string(&access_method)?,
            );
        }

        if let Some(result) = request.result {
            api_req
                .query_params
                .insert(QueryParams::RESULT, serde_json::to_string(&result)?);
        }

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert(QueryParams::START_TIME, start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert(QueryParams::END_TIME, end_time.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 下载开门时的人脸识别图片
    ///
    /// 下载指定访问记录的人脸识别图片。
    ///
    /// # Arguments
    ///
    /// * `record_id` - 访问记录ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回人脸识别图片信息
    pub async fn download_face_image(
        &self,
        record_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AccessRecordFaceImageResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::ACS_V1_ACCESS_RECORD_FACE_IMAGE,
                "record_id",
                record_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 访问记录查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessRecordListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 用户ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 设备ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// 访问类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<AccessType>,
    /// 访问方式筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_method: Option<AccessMethod>,
    /// 访问结果筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<AccessResult>,
    /// 开始时间筛选（时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间筛选（时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// 访问记录查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessRecordListResponse {
    /// 访问记录列表
    #[serde(flatten)]
    pub access_records: PageResponse<AccessRecord>,
}

impl ApiResponseTrait for AccessRecordListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 访问记录人脸图片响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessRecordFaceImageResponse {
    /// 人脸识别图片信息
    pub face_image: FaceImage,
}

impl ApiResponseTrait for AccessRecordFaceImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
