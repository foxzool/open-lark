/// 批量增加协作者权限
///
/// 批量为文件或文件夹添加协作者权限
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/batch_create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::PermissionMember;

/// 批量增加协作者权限请求
#[derive(Debug, Clone)]
pub struct BatchCreatePermissionMemberRequest {
    config: Config,
    /// 文件token
    pub token: String,
    /// 云文档类型（query 参数 `type`，需要与 token 匹配）
    pub file_type: String,
    /// 是否发送通知（query 参数 `need_notification`，默认 false）
    pub need_notification: Option<bool>,
    /// 成员权限列表
    pub members: Vec<PermissionMember>,
}

#[derive(Debug, Serialize)]
struct BatchCreatePermissionMemberBody {
    members: Vec<PermissionMember>,
}

impl BatchCreatePermissionMemberRequest {
    /// 创建批量增加协作者权限请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `file_type` - 云文档类型（query 参数 `type`）
    /// * `members` - 成员权限列表
    pub fn new(
        config: Config,
        token: impl Into<String>,
        file_type: impl Into<String>,
        members: Vec<PermissionMember>,
    ) -> Self {
        Self {
            config,
            token: token.into(),
            file_type: file_type.into(),
            need_notification: None,
            members,
        }
    }

    /// 设置是否发送通知（默认 false）
    pub fn need_notification(mut self, need_notification: bool) -> Self {
        self.need_notification = Some(need_notification);
        self
    }

    pub async fn execute(self) -> SDKResult<BatchCreatePermissionMemberResponse> {
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
        if self.members.is_empty() {
            return Err(openlark_core::error::validation_error(
                "members",
                "members 不能为空",
            ));
        }
        for member in &self.members {
            if member.member_type.is_empty() {
                return Err(openlark_core::error::validation_error(
                    "members.member_type",
                    "member_type 不能为空",
                ));
            }
            match member.member_type.as_str() {
                "email" | "openid" | "unionid" | "openchat" | "opendepartmentid" | "userid"
                | "groupid" | "wikispaceid" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "members.member_type",
                        "member_type 必须为 email/openid/unionid/openchat/opendepartmentid/userid/groupid/wikispaceid",
                    ));
                }
            }
            if member.member_id.is_empty() {
                return Err(openlark_core::error::validation_error(
                    "members.member_id",
                    "member_id 不能为空",
                ));
            }
            if member.perm.is_empty() {
                return Err(openlark_core::error::validation_error(
                    "members.perm",
                    "perm 不能为空",
                ));
            }
            match member.perm.as_str() {
                "view" | "edit" | "full_access" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "members.perm",
                        "perm 必须为 view/edit/full_access",
                    ));
                }
            }
            if self.file_type == "minutes" && member.perm == "full_access" {
                return Err(openlark_core::error::validation_error(
                    "members.perm",
                    "当 file_type=minutes 时，不支持 full_access",
                ));
            }
            if let Some(perm_type) = &member.perm_type {
                match perm_type.as_str() {
                    "container" | "single_page" => {}
                    _ => {
                        return Err(openlark_core::error::validation_error(
                            "members.perm_type",
                            "perm_type 必须为 container/single_page",
                        ));
                    }
                }
            }
            if let Some(member_kind) = &member.r#type {
                match member_kind.as_str() {
                    "user" | "chat" | "department" | "group" | "wiki_space_member"
                    | "wiki_space_viewer" | "wiki_space_editor" => {}
                    _ => {
                        return Err(openlark_core::error::validation_error(
                            "members.type",
                            "type 必须为 user/chat/department/group/wiki_space_member/wiki_space_viewer/wiki_space_editor",
                        ));
                    }
                }
            }
            if member.member_type == "wikispaceid" {
                match member.r#type.as_deref() {
                    Some("wiki_space_member" | "wiki_space_viewer" | "wiki_space_editor") => {}
                    _ => {
                        return Err(openlark_core::error::validation_error(
                            "members.type",
                            "当 member_type=wikispaceid 时，type 必须为 wiki_space_member/wiki_space_viewer/wiki_space_editor",
                        ));
                    }
                }
            }
        }

        let api_endpoint = DriveApi::BatchCreatePermissionMember(self.token.clone());

        let body = BatchCreatePermissionMemberBody {
            members: self.members,
        };

        let mut api_request: ApiRequest<BatchCreatePermissionMemberResponse> =
            ApiRequest::post(&api_endpoint.to_url()).query("type", &self.file_type);

        if let Some(need_notification) = self.need_notification {
            api_request = api_request.query("need_notification", need_notification.to_string());
        }

        api_request = api_request.body(serialize_params(&body, "批量增加协作者权限")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "批量增加协作者权限")
    }
}

/// 批量增加协作者权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreatePermissionMemberResponse {
    /// 本次新增的协作者列表
    #[serde(default)]
    pub members: Vec<PermissionMember>,
}

impl ApiResponseTrait for BatchCreatePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_create_permission_member_request_builder() {
        let config = Config::default();
        let member = PermissionMember::new("openid", "ou_123", "view");
        let request =
            BatchCreatePermissionMemberRequest::new(config, "file_token", "docx", vec![member])
                .need_notification(true);

        assert_eq!(request.token, "file_token");
        assert_eq!(request.members.len(), 1);
        assert_eq!(request.need_notification, Some(true));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            BatchCreatePermissionMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
