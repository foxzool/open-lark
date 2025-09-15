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
        query_params::QueryParams,
        req_option::RequestOption,
        SDKResult,
    },
    service::trust_party::models::{
        CollaborationDepartment, CollaborationOrganization, CollaborationUser,
        OrganizationStructure, PageResponse, SharedMemberScope,
    },
};

/// 关联组织管理服务
pub struct CollaborationOrganizationService {
    pub config: Config,
}

impl CollaborationOrganizationService {
    /// 创建关联组织管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取可见关联组织的列表
    ///
    /// 获取当前用户可见的关联组织列表，支持分页查询。
    ///
    /// # Arguments
    ///
    /// * `request` - 组织列表查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回关联组织列表
    pub async fn list_organizations(
        &self,
        request: OrganizationListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OrganizationListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取关联组织的部门和成员信息
    ///
    /// 获取指定关联组织的组织架构信息，包括部门和成员。
    ///
    /// # Arguments
    ///
    /// * `org_id` - 关联组织ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回组织架构信息
    pub async fn get_organization_structure(
        &self,
        org_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OrganizationStructureResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_VISIBLE,
                "org_id",
                org_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取关联组织详情
    ///
    /// 获取指定关联组织的详细信息。
    ///
    /// # Arguments
    ///
    /// * `org_id` - 关联组织ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回关联组织详情
    pub async fn get_organization(
        &self,
        org_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OrganizationGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_GET,
                "org_id",
                org_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取关联组织成员详情
    ///
    /// 获取指定关联组织中指定成员的详细信息。
    ///
    /// # Arguments
    ///
    /// * `org_id` - 关联组织ID
    /// * `user_id` - 成员用户ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回成员详情
    pub async fn get_organization_user(
        &self,
        org_id: &str,
        user_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OrganizationUserGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_USER_GET,
                &[("org_id", org_id), ("user_id", user_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取关联组织部门详情
    ///
    /// 获取指定关联组织中指定部门的详细信息。
    ///
    /// # Arguments
    ///
    /// * `org_id` - 关联组织ID
    /// * `department_id` - 部门ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回部门详情
    pub async fn get_organization_department(
        &self,
        org_id: &str,
        department_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OrganizationDepartmentGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET,
                &[("org_id", org_id), ("department_id", department_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取关联组织双方共享成员范围
    ///
    /// 获取与指定关联组织之间的共享成员范围信息。
    ///
    /// # Arguments
    ///
    /// * `org_id` - 关联组织ID
    /// * `request` - 共享成员范围查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回共享成员范围信息
    pub async fn list_shared_member_scope(
        &self,
        org_id: &str,
        request: SharedMemberScopeListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SharedMemberScopeListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_SHARED_MEMBER_SCOPES,
                "org_id",
                org_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 管理员获取所有关联组织列表
    ///
    /// 管理员权限获取所有关联组织的列表，包括不可见的组织。
    ///
    /// # Arguments
    ///
    /// * `request` - 管理员组织列表查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回所有关联组织列表
    pub async fn admin_list_organizations(
        &self,
        request: AdminOrganizationListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AdminOrganizationListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS_ADMIN.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }

        if let Some(status) = request.status {
            api_req.query_params.insert(QueryParams::STATUS, status);
        }

        Transport::request(api_req, &self.config, option).await
    }
}

/// 组织列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrganizationListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 组织列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationListResponse {
    /// 关联组织列表
    #[serde(flatten)]
    pub organizations: PageResponse<CollaborationOrganization>,
}

impl ApiResponseTrait for OrganizationListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 组织架构查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationStructureResponse {
    /// 组织架构信息
    pub organization_structure: OrganizationStructure,
}

impl ApiResponseTrait for OrganizationStructureResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 组织详情查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationGetResponse {
    /// 关联组织详情
    pub organization: CollaborationOrganization,
}

impl ApiResponseTrait for OrganizationGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 组织成员详情查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationUserGetResponse {
    /// 成员详情
    pub user: CollaborationUser,
}

impl ApiResponseTrait for OrganizationUserGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 组织部门详情查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationDepartmentGetResponse {
    /// 部门详情
    pub department: CollaborationDepartment,
}

impl ApiResponseTrait for OrganizationDepartmentGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 共享成员范围列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SharedMemberScopeListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 共享成员范围列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SharedMemberScopeListResponse {
    /// 共享成员范围列表
    #[serde(flatten)]
    pub shared_scopes: PageResponse<SharedMemberScope>,
}

impl ApiResponseTrait for SharedMemberScopeListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 管理员组织列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdminOrganizationListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 组织状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 管理员组织列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminOrganizationListResponse {
    /// 关联组织列表
    #[serde(flatten)]
    pub organizations: PageResponse<CollaborationOrganization>,
}

impl ApiResponseTrait for AdminOrganizationListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
