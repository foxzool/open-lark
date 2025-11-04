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
    service::apass::models::{
        RecordPermissionMemberAuthorizationRequest, RoleMember, RoleMemberAuthorizationRequest,
    },
};

/// 权限管理服务
pub struct PermissionService {
    pub config: Config,
}

/// 操作成功响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OperationSuccessResponse {
    /// 操作是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 操作消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ApiResponseTrait for OperationSuccessResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 角色成员信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleMemberGetResponse {
    /// 角色成员信息
    #[serde(flatten)]
    pub role_member: RoleMember,
}

impl ApiResponseTrait for RoleMemberGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PermissionService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量删除角色成员授权
    ///
    /// 该接口用于批量删除角色成员的授权。
    ///
    /// # 参数
    ///
    /// - `request`: 角色成员授权操作请求参数
    /// - `option`: 可选的请求配置
    pub async fn batch_remove_role_member_authorization(
        &self,
        request: RoleMemberAuthorizationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OperationSuccessResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_PERMISSION_ROLE_MEMBERS_BATCH_REMOVE,
                &[
                    ("app_id", &request.app_id),
                    ("role_api_name", &request.role_api_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "user_ids": request.user_ids
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量创建角色成员授权
    ///
    /// 该接口用于批量创建角色成员的授权。
    ///
    /// # 参数
    ///
    /// - `request`: 角色成员授权操作请求参数
    /// - `option`: 可选的请求配置
    pub async fn batch_create_role_member_authorization(
        &self,
        request: RoleMemberAuthorizationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OperationSuccessResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_PERMISSION_ROLE_MEMBERS_BATCH_CREATE,
                &[
                    ("app_id", &request.app_id),
                    ("role_api_name", &request.role_api_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "user_ids": request.user_ids
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询角色成员信息
    ///
    /// 该接口用于查询指定角色的成员信息。
    ///
    /// # 参数
    ///
    /// - `app_id`: 应用ID
    /// - `role_api_name`: 角色API名称
    /// - `user_id`: 用户ID
    /// - `option`: 可选的请求配置
    pub async fn get_role_member(
        &self,
        app_id: String,
        role_api_name: String,
        user_id: String,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RoleMemberGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_PERMISSION_ROLE_MEMBER_GET,
                &[
                    ("app_id", &app_id),
                    ("role_api_name", &role_api_name),
                    ("user_id", &user_id),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量删除记录权限用户授权
    ///
    /// 该接口用于批量删除记录权限用户的授权。
    ///
    /// # 参数
    ///
    /// - `request`: 记录权限用户授权操作请求参数
    /// - `option`: 可选的请求配置
    pub async fn batch_remove_record_permission_member_authorization(
        &self,
        request: RecordPermissionMemberAuthorizationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OperationSuccessResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_PERMISSION_RECORD_MEMBERS_BATCH_REMOVE,
                &[
                    ("app_id", &request.app_id),
                    (
                        "record_permission_api_name",
                        &request.record_permission_api_name,
                    ),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "user_ids": request.user_ids,
                "record_ids": request.record_ids
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量创建记录权限用户授权
    ///
    /// 该接口用于批量创建记录权限用户的授权。
    ///
    /// # 参数
    ///
    /// - `request`: 记录权限用户授权操作请求参数
    /// - `option`: 可选的请求配置
    pub async fn batch_create_record_permission_member_authorization(
        &self,
        request: RecordPermissionMemberAuthorizationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OperationSuccessResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_PERMISSION_RECORD_MEMBERS_BATCH_CREATE,
                &[
                    ("app_id", &request.app_id),
                    (
                        "record_permission_api_name",
                        &request.record_permission_api_name,
                    ),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "user_ids": request.user_ids,
                "record_ids": request.record_ids
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
