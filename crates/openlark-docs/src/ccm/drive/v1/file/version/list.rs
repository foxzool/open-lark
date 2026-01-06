//! 获取文档版本列表
//!
//! 获取指定源文档的版本文档列表。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/list

use crate::common::{api_endpoints::DriveApi, api_utils::*};
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use super::models::ListFileVersionsData;

/// 获取文档版本列表请求
#[derive(Debug, Clone)]
pub struct ListFileVersionsRequest {
    config: Config,
    /// 源文档 token
    pub file_token: String,
    /// 分页大小（1~100）
    pub page_size: i32,
    /// 源文档类型（docx/sheet）
    pub obj_type: String,
    /// 分页标记
    pub page_token: Option<String>,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
}

impl ListFileVersionsRequest {
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        page_size: i32,
        obj_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            page_size,
            obj_type: obj_type.into(),
            page_token: None,
            user_id_type: None,
        }
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListFileVersionsResponse> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }
        if !(1..=100).contains(&self.page_size) {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "page_size 必须在 1~100 之间",
            ));
        }
        match self.obj_type.as_str() {
            "docx" | "sheet" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "obj_type",
                    "obj_type 仅支持 docx/sheet",
                ))
            }
        }

        let api_endpoint = DriveApi::ListFileVersions(self.file_token);
        let request = ApiRequest::<ListFileVersionsResponse>::get(&api_endpoint.to_url())
            .query("page_size", self.page_size.to_string())
            .query("obj_type", self.obj_type)
            .query_opt("page_token", self.page_token)
            .query_opt("user_id_type", self.user_id_type);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取文档版本列表")
    }
}

/// 获取文档版本列表响应（data）
pub type ListFileVersionsResponse = ListFileVersionsData;

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::api::ApiResponseTrait;

    #[test]
    fn test_list_file_versions_request_builder() {
        let config = Config::default();
        let request = ListFileVersionsRequest::new(config, "file_token", 20, "docx")
            .page_token("token123")
            .user_id_type("open_id");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.page_size, 20);
        assert_eq!(request.obj_type, "docx");
        assert_eq!(request.page_token, Some("token123".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            <ListFileVersionsData as ApiResponseTrait>::data_format(),
            openlark_core::api::ResponseFormat::Data
        );
    }
}
