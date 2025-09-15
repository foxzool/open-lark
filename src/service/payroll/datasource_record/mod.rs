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
        req_option::RequestOption,
        SDKResult,
    },
    service::payroll::models::{
        DatasourceRecord, DatasourceRecordQueryRequest, DatasourceRecordSaveRequest, PageResponse,
    },
};

/// 外部数据源记录服务
pub struct DatasourceRecordService {
    pub config: Config,
}

/// 外部数据源记录保存响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DatasourceRecordSaveResponse {
    /// 保存结果
    pub success: bool,
    /// 保存的记录数量
    pub record_count: Option<u32>,
    /// 失败的记录
    pub failed_records: Option<Vec<FailedRecord>>,
    /// 处理消息
    pub message: Option<String>,
}

impl ApiResponseTrait for DatasourceRecordSaveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 失败记录信息
#[derive(Debug, Serialize, Deserialize)]
pub struct FailedRecord {
    /// 员工ID
    pub employee_id: String,
    /// 失败原因
    pub error_message: String,
    /// 错误代码
    pub error_code: Option<String>,
}

/// 外部数据源记录查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DatasourceRecordQueryResponse {
    /// 数据源记录列表
    #[serde(flatten)]
    pub records: PageResponse<DatasourceRecord>,
}

impl ApiResponseTrait for DatasourceRecordQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DatasourceRecordService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建/更新外部算薪数据
    ///
    /// 该接口用于向飞书发薪系统创建或更新外部算薪数据记录。
    /// 支持批量操作，可以同时处理多个员工的算薪数据。
    /// 外部算薪数据通常用于与第三方薪酬系统集成。
    ///
    /// # 参数
    ///
    /// - `request`: 外部数据源记录保存请求参数，包括：
    ///   - `datasource_id`: 数据源ID（必填）
    ///   - `employee_id`: 员工ID（必填）
    ///   - `user_id_type`: 用户ID类型
    ///   - `records`: 数据记录列表（必填）
    ///   - `payment_period`: 发薪周期（必填）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回保存操作结果，包括：
    /// - `success`: 操作是否成功
    /// - `record_count`: 成功保存的记录数量
    /// - `failed_records`: 失败的记录列表及原因
    /// - `message`: 处理结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::payroll::models::{DatasourceRecordSaveRequest, DatasourceRecord};
    /// use std::collections::HashMap;
    ///
    /// let mut field_values = HashMap::new();
    /// field_values.insert("base_salary", serde_json::Value::Number(serde_json::Number::from(10000)));
    /// field_values.insert("overtime_hours", serde_json::Value::Number(serde_json::Number::from(20)));
    ///
    /// let record = DatasourceRecord {
    ///     record_id: None,
    ///     employee_id: "emp_123".to_string(),
    ///     field_values,
    ///     payment_period: "2024-01".to_string(),
    ///     created_time: None,
    ///     updated_time: None,
    /// };
    ///
    /// let request = DatasourceRecordSaveRequest {
    ///     datasource_id: "ds_456".to_string(),
    ///     employee_id: "emp_123".to_string(),
    ///     user_id_type: Some("open_id".to_string()),
    ///     records: vec![record],
    ///     payment_period: "2024-01".to_string(),
    /// };
    ///
    /// let response = client.payroll.datasource_record.save_records(request, None).await?;
    /// ```
    pub async fn save_records(
        &self,
        request: DatasourceRecordSaveRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DatasourceRecordSaveResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::PAYROLL_V1_DATASOURCE_RECORDS_SAVE,
                "datasource_id",
                &request.datasource_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // 添加查询参数
        if let Some(user_id_type) = request.user_id_type {
            api_req.query_params.insert("user_id_type", user_id_type);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量查询外部算薪数据记录
    ///
    /// 该接口用于批量查询指定数据源和员工的外部算薪数据记录。
    /// 支持按发薪周期筛选，分页查询大量数据记录。
    /// 适用于数据验证、报表生成等场景。
    ///
    /// # 参数
    ///
    /// - `request`: 外部数据源记录查询请求参数，包括：
    ///   - `datasource_id`: 数据源ID（必填）
    ///   - `employee_ids`: 员工ID列表（必填）
    ///   - `user_id_type`: 用户ID类型
    ///   - `payment_period`: 发薪周期（必填）
    ///   - `page_size`: 分页大小
    ///   - `page_token`: 分页标记
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的数据源记录列表，包括：
    /// - 指定员工和周期的数据记录
    /// - 数据字段值映射信息
    /// - 记录的创建和更新时间
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::payroll::models::DatasourceRecordQueryRequest;
    ///
    /// let request = DatasourceRecordQueryRequest {
    ///     datasource_id: "ds_456".to_string(),
    ///     employee_ids: vec![
    ///         "emp_001".to_string(),
    ///         "emp_002".to_string(),
    ///         "emp_003".to_string(),
    ///     ],
    ///     user_id_type: Some("open_id".to_string()),
    ///     payment_period: "2024-01".to_string(),
    ///     page_size: Some(50),
    ///     page_token: None,
    /// };
    ///
    /// let response = client.payroll.datasource_record.query_records(request, None).await?;
    /// ```
    pub async fn query_records(
        &self,
        request: DatasourceRecordQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DatasourceRecordQueryResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::PAYROLL_V1_DATASOURCE_RECORDS_QUERY,
                "datasource_id",
                &request.datasource_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // 添加查询参数
        if let Some(user_id_type) = request.user_id_type {
            api_req.query_params.insert("user_id_type", user_id_type);
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
