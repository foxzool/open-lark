use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 添加协作者
///
/// 为文件或文件夹添加协作者权限
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create
/// doc: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/create
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::MemberInfo;

/// 添加协作者请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionMemberRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub token: String,
    /// 成员信息
    pub member: MemberInfo,
    /// 权限类型
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize)]
struct CreatePermissionMemberRequestBody {
    member: MemberInfo,
    #[serde(rename = "type")]
    r#type: String,
}

impl CreatePermissionMemberRequest {
    /// 创建添加协作者请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `member` - 成员信息
    /// * `type` - 权限类型
    pub fn new(
        config: Config,
        token: impl Into<String>,
        member: MemberInfo,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            token: token.into(),
            member,
            r#type: r#type.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<CreatePermissionMemberResponse> {
        if self.token.is_empty() {
            return Err(openlark_core::error::validation_error("token", "token 不能为空"));
        }
        if self.member.user_id.is_empty() {
            return Err(openlark_core::error::validation_error(
                "member.user_id",
                "user_id 不能为空",
            ));
        }
        if self.member.user_type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "member.user_type",
                "user_type 不能为空",
            ));
        }
        if self.r#type.is_empty() {
            return Err(openlark_core::error::validation_error("type", "type 不能为空"));
        }

        let api_endpoint = DriveApi::CreatePermissionMember(self.token.clone());

        let api_request = ApiRequest::<CreatePermissionMemberResponse>::post(&api_endpoint.to_url())
            .body(serialize_params(
                &CreatePermissionMemberRequestBody {
                member: self.member,
                r#type: self.r#type,
                },
                "增加协作者权限",
            )?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "增加协作者权限")
    }
}

/// 协作者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionMemberResponse {
    /// 成员ID
    pub member_id: String,
    /// 用户ID
    pub user_id: String,
    /// 权限类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 创建时间
    pub create_time: i64,
}

impl ApiResponseTrait for CreatePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_permission_member_request_builder() {
        let config = Config::default();
        let member = MemberInfo {
            user_id: "user_id".to_string(),
            user_type: "user".to_string(),
        };

        let request = CreatePermissionMemberRequest::new(config, "file_token", member, "admin");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.r#type, "admin");
    }

    #[test]
    fn test_member_info_structure() {
        let member = MemberInfo {
            user_id: "user_id".to_string(),
            user_type: "user".to_string(),
        };

        assert_eq!(member.user_id, "user_id");
        assert_eq!(member.user_type, "user");
    }

    #[test]
    fn test_permission_member_data_structure() {
        let permission_data = CreatePermissionMemberResponse {
            member_id: "member_id".to_string(),
            user_id: "user_id".to_string(),
            r#type: "admin".to_string(),
            create_time: 1640995200,
        };

        assert_eq!(permission_data.member_id, "member_id");
        assert_eq!(permission_data.user_id, "user_id");
        assert_eq!(permission_data.r#type, "admin");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreatePermissionMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
