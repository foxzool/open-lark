/// 批量增加协作者权限
///
/// 批量为文件或文件夹添加协作者权限
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/batch_create
/// doc: https://open.feishu.cn/document/docs/permission/permission-member/batch_create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::MemberPermission;

/// 批量增加协作者权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreatePermissionMemberRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub token: String,
    /// 成员权限列表
    pub members: Vec<MemberPermission>,
    /// 是否发送通知
    pub notify: Option<bool>,
}
#[derive(Debug, Serialize)]
struct BatchCreatePermissionMemberBody {
    members: Vec<MemberPermission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify: Option<bool>,
}

impl BatchCreatePermissionMemberRequest {
    /// 创建批量增加协作者权限请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `members` - 成员权限列表
    pub fn new(config: Config, token: impl Into<String>, members: Vec<MemberPermission>) -> Self {
        Self {
            config,
            token: token.into(),
            members,
            notify: None,
        }
    }

    /// 设置是否发送通知
    pub fn notify(mut self, notify: bool) -> Self {
        self.notify = Some(notify);
        self
    }

    pub async fn execute(self) -> SDKResult<BatchCreatePermissionMemberResponse> {
        if self.token.is_empty() {
            return Err(openlark_core::error::validation_error("token", "token 不能为空"));
        }
        if self.members.is_empty() {
            return Err(openlark_core::error::validation_error(
                "members",
                "members 不能为空",
            ));
        }
        for member in &self.members {
            if member.member.user_id.is_empty() {
                return Err(openlark_core::error::validation_error(
                    "members.member.user_id",
                    "user_id 不能为空",
                ));
            }
            if member.member.user_type.is_empty() {
                return Err(openlark_core::error::validation_error(
                    "members.member.user_type",
                    "user_type 不能为空",
                ));
            }
            if member.r#type.is_empty() {
                return Err(openlark_core::error::validation_error(
                    "members.type",
                    "type 不能为空",
                ));
            }
        }

        let api_endpoint = DriveApi::BatchCreatePermissionMember(self.token.clone());

        let body = BatchCreatePermissionMemberBody {
            members: self.members,
            notify: self.notify,
        };

        let api_request: ApiRequest<BatchCreatePermissionMemberResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&body, "批量增加协作者权限")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "批量增加协作者权限")
    }
}

/// 批量增加协作者权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreatePermissionMemberResponse {
    /// 批量操作结果
    pub batch_result: BatchOperationResult,
}

/// 批量操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResult {
    /// 成功数量
    pub success_count: i32,
    /// 失败数量
    pub failed_count: i32,
    /// 总数量
    pub total_count: i32,
    /// 结果列表
    pub results: Vec<BatchItemResult>,
}

/// 批量操作单项结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchItemResult {
    /// 是否成功
    pub success: bool,
    /// 用户ID
    pub user_id: Option<String>,
    /// 错误信息
    pub error: Option<String>,
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
        let member = MemberInfo::new("user_123", "user");
        let permission = MemberPermission::new(member, "view");
        let request =
            BatchCreatePermissionMemberRequest::new(config, "file_token", vec![permission])
                .notify(true);

        assert_eq!(request.token, "file_token");
        assert_eq!(request.members.len(), 1);
        assert_eq!(request.notify, Some(true));
    }

    #[test]
    fn test_member_permission_builder() {
        let member = MemberInfo::new("user_123", "user");
        let permission = MemberPermission::new(member, "edit");

        assert_eq!(permission.member.user_id, "user_123");
        assert_eq!(permission.r#type, "edit");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            BatchCreatePermissionMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
