use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::admin,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::admin::models::{
        DataReport, DepartmentDataReportRequest, PageResponse, UserDataReportRequest,
    },
};

/// 数据报表管理服务
pub struct DataReportService {
    pub config: Config,
}

/// 部门维度数据报表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentDataReportResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<DataReport>,
}

impl ApiResponseTrait for DepartmentDataReportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户维度数据报表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataReportResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<DataReport>,
}

impl ApiResponseTrait for UserDataReportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DataReportService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取部门维度的用户活跃和功能使用数据
    ///
    /// 该接口用于获取部门维度的用户活跃度和功能使用统计数据。
    ///
    /// # 参数
    ///
    /// - `request`: 部门数据报表查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn get_department_data_report(
        &self,
        request: DepartmentDataReportRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DepartmentDataReportResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: admin::ADMIN_V1_DATA_REPORT_DEPARTMENT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req
            .query_params
            .insert("start_date", request.start_date);
        api_req.query_params.insert("end_date", request.end_date);

        if let Some(department_id_type) = request.department_id_type {
            api_req
                .query_params
                .insert("department_id_type", department_id_type);
        }
        if let Some(department_id) = request.department_id {
            api_req.query_params.insert("department_id", department_id);
        }
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取用户维度的用户活跃和功能使用数据
    ///
    /// 该接口用于获取用户维度的活跃度和功能使用统计数据。
    ///
    /// # 参数
    ///
    /// - `request`: 用户数据报表查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn get_user_data_report(
        &self,
        request: UserDataReportRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserDataReportResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: admin::ADMIN_V1_DATA_REPORT_USER.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req
            .query_params
            .insert("start_date", request.start_date);
        api_req.query_params.insert("end_date", request.end_date);

        if let Some(user_id_type) = request.user_id_type {
            api_req.query_params.insert("user_id_type", user_id_type);
        }
        if let Some(user_ids) = request.user_ids {
            api_req.query_params.insert("user_ids", user_ids.join(","));
        }
        if let Some(department_id_type) = request.department_id_type {
            api_req
                .query_params
                .insert("department_id_type", department_id_type);
        }
        if let Some(department_id) = request.department_id {
            api_req.query_params.insert("department_id", department_id);
        }
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
