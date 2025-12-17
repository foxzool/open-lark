/// Bitable 列出表单字段API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/form/field/list
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    SDKResult,
};

use serde::{Deserialize, Serialize};


/// 表单字段问题
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormFieldQuestion {
    /// 字段ID
    pub field_id: String,
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    pub field_type: String,
    /// 是否必填
    pub required: Option<bool>,
    /// 字段描述
    pub description: Option<String>,
    /// 字段选项（用于单选、多选等）
    pub options: Option<Vec<FormFieldOption>>,
    /// 字段默认值
    pub default_value: Option<serde_json::Value>,
    /// 字段验证规则
    pub validation: Option<FormFieldValidation>,
}

/// 字段选项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormFieldOption {
    /// 选项ID
    pub option_id: String,
    /// 选项名称
    pub name: String,
    /// 选项值
    pub value: Option<serde_json::Value>,
}

/// 字段验证规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormFieldValidation {
    /// 是否必填
    pub required: Option<bool>,
    /// 最小长度
    pub min_length: Option<i32>,
    /// 最大长度
    pub max_length: Option<i32>,
    /// 正则表达式
    pub pattern: Option<String>,
}

/// 列出表单字段请求
#[derive(Debug, Clone)]
pub struct ListFormFieldQuestionRequest {
    api_request: ApiRequest<ListFormFieldQuestionResponse>,
    /// 应用token
    app_token: String,
    /// 表格ID
    table_id: String,
    /// 表单ID
    form_id: String,
    /// 页面大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

/// 列出表单字段响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFormFieldQuestionResponse {
    /// 表单字段列表
    pub data: Option<ListFormFieldQuestionData>,
}

/// 列出表单字段数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFormFieldQuestionData {
    /// 表单字段列表
    pub items: Option<Vec<FormFieldQuestion>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListFormFieldQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListFormFieldQuestionRequest {
    /// 创建列出表单字段请求
    pub fn new(app_token: String, table_id: String, form_id: String) -> Self {
        let endpoint = format!(
            "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/fields",
            app_token,
            table_id,
            form_id
        );
        let api_request = ApiRequest::get(endpoint)
            .header("Content-Type", "application/json");

        Self {
            api_request,
            app_token,
            table_id,
            form_id,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self, config: &Config) -> SDKResult<ListFormFieldQuestionData> {
        let mut request = self.api_request;

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(page_size) = self.page_size {
            query_params.push(("page_size", page_size.to_string()));
        }
        if let Some(page_token) = self.page_token {
            query_params.push(("page_token", page_token));
        }

        if !query_params.is_empty() {
            request = request.query_params(query_params);
        }

        // 发送请求
        let response: openlark_core::api::Response<ListFormFieldQuestionResponse> =
            openlark_core::http::Transport::request(request, config, None).await?;

        // 解析响应
        let resp_data = response.data.ok_or_else(|| {
            validation_error("response_data", "Response data is missing")
        })?;
        resp_data.data.ok_or_else(|| {
            validation_error("data", "List form field question data is missing")
        })
    }
}

/// 列出表单字段
pub fn list_form_field_questions(
    app_token: String,
    table_id: String,
    form_id: String,
) -> ListFormFieldQuestionRequest {
    ListFormFieldQuestionRequest::new(app_token, table_id, form_id)
}
