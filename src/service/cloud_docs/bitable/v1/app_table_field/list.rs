use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::bitable::v1::app_table_field::AppTableField,
};

/// 列出字段请求
#[derive(Debug, Serialize, Default, Clone)]
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
                .insert("view_id", view_id.clone());
        }
        if let Some(text_field_as_array) = &self.request.text_field_as_array {
            self.request
                .api_request
                .query_params
                .insert("text_field_as_array", text_field_as_array.to_string());
        }
        if let Some(page_token) = &self.request.page_token {
            self.request
                .api_request
                .query_params
                .insert("page_token", page_token.clone());
        }
        if let Some(page_size) = &self.request.page_size {
            self.request
                .api_request
                .query_params
                .insert("page_size", page_size.to_string());
        }
        self.request
    }
}

// 应用ExecutableBuilder trait到ListFieldRequestBuilder
crate::impl_executable_builder_owned!(
    ListFieldRequestBuilder,
    super::AppTableFieldService,
    ListFieldRequest,
    BaseResponse<ListFieldResponse>,
    list
);

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
    api_req.api_path = BITABLE_V1_FIELDS
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
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
