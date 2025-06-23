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
    service::bitable::v1::app_table_field::AppTableField,
};

/// 列出字段请求
#[derive(Debug, Serialize, Default)]
pub struct ListFieldRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 视图 ID
    #[serde(skip)]
    view_id: Option<String>,
    /// 控制字段描述数据的返回格式
    #[serde(skip)]
    text_field_as_array: Option<bool>,
    /// 分页标记
    #[serde(skip)]
    page_token: Option<String>,
    /// 分页大小
    #[serde(skip)]
    page_size: Option<i32>,
}

impl ListFieldRequest {
    pub fn builder() -> ListFieldRequestBuilder {
        ListFieldRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct ListFieldRequestBuilder {
    request: ListFieldRequest,
}

impl ListFieldRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 数据表的唯一标识符
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 视图 ID
    pub fn view_id(mut self, view_id: impl ToString) -> Self {
        self.request.view_id = Some(view_id.to_string());
        self
    }

    /// 控制字段描述（多行文本格式）数据的返回格式
    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.request.text_field_as_array = Some(text_field_as_array);
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn build(mut self) -> ListFieldRequest {
        if let Some(view_id) = &self.request.view_id {
            self.request
                .api_request
                .query_params
                .insert("view_id".to_string(), view_id.clone());
        }
        if let Some(text_field_as_array) = &self.request.text_field_as_array {
            self.request.api_request.query_params.insert(
                "text_field_as_array".to_string(),
                text_field_as_array.to_string(),
            );
        }
        if let Some(page_token) = &self.request.page_token {
            self.request
                .api_request
                .query_params
                .insert("page_token".to_string(), page_token.clone());
        }
        if let Some(page_size) = &self.request.page_size {
            self.request
                .api_request
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }
        self.request
    }

    /// 发起列出字段请求
    pub async fn execute(
        self,
        service: &crate::service::cloud_docs::bitable::v1::app_table_field::AppTableFieldService,
    ) -> SDKResult<BaseResponse<ListFieldResponse>> {
        service.list(self.build(), None).await
    }

    /// 发起列出字段请求（带选项）
    pub async fn execute_with_options(
        self,
        service: &crate::service::cloud_docs::bitable::v1::app_table_field::AppTableFieldService,
        option: RequestOption,
    ) -> SDKResult<BaseResponse<ListFieldResponse>> {
        service.list(self.build(), Some(option)).await
    }
}

/// 列出字段响应
#[derive(Debug, Deserialize)]
pub struct ListFieldResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 字段信息列表
    pub items: Vec<AppTableField>,
}

impl ApiResponseTrait for ListFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出字段
pub async fn list_field(
    request: ListFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListFieldResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path = format!(
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields",
        app_token = request.app_token,
        table_id = request.table_id
    );
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_field_request_builder() {
        let request = ListFieldRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .page_size(20)
            .text_field_as_array(true)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.text_field_as_array, Some(true));
    }
}
