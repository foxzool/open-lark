//! 获取云文档协作者
//!
//! 获取文件或文件夹的协作者列表。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::PermissionMemberItem;

/// 获取协作者列表请求
#[derive(Debug, Clone)]
pub struct ListPermissionMembersRequest {
    config: Config,
    /// 文件token
    pub token: String,
    /// 云文档类型（query 参数 `type`，需要与 token 匹配）
    pub file_type: String,
    /// 返回字段（query 参数 `fields`）
    pub fields: Option<String>,
    /// 权限角色类型（query 参数 `perm_type`）
    pub perm_type: Option<String>,
}

impl ListPermissionMembersRequest {
    /// 创建获取协作者列表请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `file_type` - 云文档类型（query 参数 `type`）
    pub fn new(config: Config, token: impl Into<String>, file_type: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
            file_type: file_type.into(),
            fields: None,
            perm_type: None,
        }
    }

    /// 设置返回字段（如 `*`）
    pub fn fields(mut self, fields: impl Into<String>) -> Self {
        self.fields = Some(fields.into());
        self
    }

    /// 设置权限角色类型（知识库文档有效）
    pub fn perm_type(mut self, perm_type: impl Into<String>) -> Self {
        self.perm_type = Some(perm_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListPermissionMembersResponse> {
        if self.token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "token",
                "token 不能为空",
            ));
        }
        if self.file_type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_type",
                "file_type 不能为空",
            ));
        }
        match self.file_type.as_str() {
            "doc" | "sheet" | "file" | "wiki" | "bitable" | "docx" | "folder" | "mindnote"
            | "minutes" | "slides" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "file_type",
                    "file_type 必须为 doc/sheet/file/wiki/bitable/docx/folder/mindnote/minutes/slides",
                ));
            }
        }
        if let Some(perm_type) = &self.perm_type {
            match perm_type.as_str() {
                "container" | "single_page" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "perm_type",
                        "perm_type 必须为 container/single_page",
                    ));
                }
            }
        }

        let api_endpoint = DriveApi::ListPermissionMembers(self.token.clone());

        let mut api_request =
            ApiRequest::<ListPermissionMembersResponse>::get(&api_endpoint.to_url());

        api_request = api_request.query("type", &self.file_type);
        if let Some(fields) = &self.fields {
            api_request = api_request.query("fields", fields);
        }
        if let Some(perm_type) = &self.perm_type {
            api_request = api_request.query("perm_type", perm_type);
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取云文档协作者")
    }
}

/// 获取协作者列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPermissionMembersResponse {
    /// 协作者列表
    #[serde(default)]
    pub items: Vec<PermissionMemberItem>,
}

impl ApiResponseTrait for ListPermissionMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_permission_members_request_builder() {
        let config = Config::default();
        let request = ListPermissionMembersRequest::new(config, "file_token", "docx");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.file_type, "docx");
    }

    #[test]
    fn test_list_permission_members_request_with_params() {
        let config = Config::default();
        let request = ListPermissionMembersRequest::new(config, "file_token", "docx")
            .fields("*")
            .perm_type("container");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.file_type, "docx");
        assert_eq!(request.fields.unwrap(), "*");
        assert_eq!(request.perm_type.unwrap(), "container");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            ListPermissionMembersResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
