use crate::{
    api::ApiRequest, api::ApiResponseTrait, config::Config, constants::AccessTokenType,
    endpoints::EndpointBuilder, http::Transport,
};
use serde::{Deserialize, Serialize};

/// 角色管理服务
#[derive(Debug)]
pub struct FunctionalRoleService {
    config: Config,
}

impl FunctionalRoleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 创建角色
    pub async fn create(
        &self,
        req: &CreateFunctionalRoleRequest,
    ) -> crate::SDKResult<CreateFunctionalRoleResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_method(reqwest::Method::POST);
        api_req.set_api_path(crate::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(req)?;
        let resp =
            Transport::<CreateFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 修改角色名称
    pub async fn update(
        &self,
        role_id: &str,
        req: &UpdateFunctionalRoleRequest,
    ) -> crate::SDKResult<UpdateFunctionalRoleResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_method(reqwest::Method::PUT);
        api_req.set_api_path(EndpointBuilder::replace_param(
                crate::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLE_GET,
                "role_id",
                role_id,
            ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(req)?;
        let resp =
            Transport::<UpdateFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/contact/get


    /// 获取单个角色信息
    pub async fn get(&self, role_id: &str) -> crate::SDKResult<GetFunctionalRoleResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_method(reqwest::Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
                crate::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLE_GET,
                "role_id",
                role_id,
            ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        let resp =
            Transport::<GetFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取角色列表
    pub async fn list(
        &self,
        req: &ListFunctionalRolesRequest,
    ) -> crate::SDKResult<ListFunctionalRolesResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_method(reqwest::Method::GET);
        api_req.set_api_path(crate::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.query_params = std::collections::HashMap::new();
        // 添加查询参数
        if let Some(page_size) = req.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = &req.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token.clone());
        }
        let resp =
            Transport::<ListFunctionalRolesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除角色
    pub async fn delete(
        &self,
        role_id: &str,
    ) -> crate::SDKResult<DeleteFunctionalRoleResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_method(reqwest::Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(
                crate::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLE_GET,
                "role_id",
                role_id,
            ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        let resp =
            Transport::<DeleteFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFunctionalRoleRequest {
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFunctionalRoleResponse {
    pub role_id: String,
}

impl ApiResponseTrait for CreateFunctionalRoleResponse {
    fn data_format() -> crate::api::ResponseFormat {
        crate::api::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFunctionalRoleRequest {
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFunctionalRoleResponse {}

impl ApiResponseTrait for UpdateFunctionalRoleResponse {
    fn data_format() -> crate::api::ResponseFormat {
        crate::api::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetFunctionalRoleResponse {
    pub role: FunctionalRole,
}

impl ApiResponseTrait for GetFunctionalRoleResponse {
    fn data_format() -> crate::api::ResponseFormat {
        crate::api::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFunctionalRolesRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListFunctionalRolesResponse {
    pub roles: Vec<FunctionalRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListFunctionalRolesResponse {
    fn data_format() -> crate::api::ResponseFormat {
        crate::api::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionalRole {
    /// 角色ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFunctionalRoleResponse {}

impl ApiResponseTrait for DeleteFunctionalRoleResponse {
    fn data_format() -> crate::api::ResponseFormat {
        crate::api::ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_functional_role_service_creation() {
        let config = Config::default();
        let service = FunctionalRoleService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_functional_role_service_creation_with_custom_config() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_secret")
            .build();
        let service = FunctionalRoleService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_create_functional_role_request_construction() {
        let request = CreateFunctionalRoleRequest {
            role_name: "测试角色".to_string(),
        };
        assert_eq!(request.role_name, "测试角色");
        assert!(format!("{:?}", request).contains("测试角色"));
    }

    #[test]
    fn test_create_functional_role_request_with_empty_name() {
        let request = CreateFunctionalRoleRequest {
            role_name: "".to_string(),
        };
        assert_eq!(request.role_name, "");
    }

    #[test]
    fn test_create_functional_role_request_with_long_name() {
        let long_name = "a".repeat(500);
        let request = CreateFunctionalRoleRequest {
            role_name: long_name.clone(),
        };
        assert_eq!(request.role_name, long_name);
    }

    #[test]
    fn test_create_functional_role_response_default() {
        let response = CreateFunctionalRoleResponse::default();
        assert_eq!(response.role_id, "");
    }

    #[test]
    fn test_create_functional_role_response_construction() {
        let response = CreateFunctionalRoleResponse {
            role_id: "role_123".to_string(),
        };
        assert_eq!(response.role_id, "role_123");
    }

    #[test]
    fn test_create_functional_role_response_data_format() {
        assert_eq!(
            CreateFunctionalRoleResponse::data_format(),
            crate::api::ResponseFormat::Data
        );
    }

    #[test]
    fn test_update_functional_role_request_construction() {
        let request = UpdateFunctionalRoleRequest {
            role_name: "更新后角色".to_string(),
        };
        assert_eq!(request.role_name, "更新后角色");
        assert!(format!("{:?}", request).contains("更新后角色"));
    }

    #[test]
    fn test_update_functional_role_request_with_empty_name() {
        let request = UpdateFunctionalRoleRequest {
            role_name: "".to_string(),
        };
        assert_eq!(request.role_name, "");
    }

    #[test]
    fn test_update_functional_role_request_with_long_name() {
        let long_name = "b".repeat(1000);
        let request = UpdateFunctionalRoleRequest {
            role_name: long_name.clone(),
        };
        assert_eq!(request.role_name, long_name);
    }

    #[test]
    fn test_update_functional_role_response_default() {
        let response = UpdateFunctionalRoleResponse::default();
        assert!(!format!("{:?}", response).is_empty());
    }

    #[test]
    fn test_update_functional_role_response_data_format() {
        assert_eq!(
            UpdateFunctionalRoleResponse::data_format(),
            crate::api::ResponseFormat::Data
        );
    }

    #[test]
    fn test_get_functional_role_response_default() {
        let response = GetFunctionalRoleResponse::default();
        assert_eq!(response.role.role_id, None);
        assert_eq!(response.role.role_name, None);
    }

    #[test]
    fn test_get_functional_role_response_construction() {
        let role = FunctionalRole {
            role_id: Some("role_456".to_string()),
            role_name: Some("管理员角色".to_string()),
        };
        let response = GetFunctionalRoleResponse { role };
        assert_eq!(response.role.role_id, Some("role_456".to_string()));
        assert_eq!(response.role.role_name, Some("管理员角色".to_string()));
    }

    #[test]
    fn test_get_functional_role_response_data_format() {
        assert_eq!(
            GetFunctionalRoleResponse::data_format(),
            crate::api::ResponseFormat::Data
        );
    }

    #[test]
    fn test_list_functional_roles_request_default() {
        let request = ListFunctionalRolesRequest {
            page_size: None,
            page_token: None,
        };
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_list_functional_roles_request_with_pagination() {
        let request = ListFunctionalRolesRequest {
            page_size: Some(50),
            page_token: Some("token_123".to_string()),
        };
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token_123".to_string()));
    }

    #[test]
    fn test_list_functional_roles_request_with_large_page_size() {
        let request = ListFunctionalRolesRequest {
            page_size: Some(10000),
            page_token: Some("large_page_token".to_string()),
        };
        assert_eq!(request.page_size, Some(10000));
        assert_eq!(request.page_token, Some("large_page_token".to_string()));
    }

    #[test]
    fn test_list_functional_roles_request_with_zero_page_size() {
        let request = ListFunctionalRolesRequest {
            page_size: Some(0),
            page_token: None,
        };
        assert_eq!(request.page_size, Some(0));
    }

    #[test]
    fn test_list_functional_roles_request_with_negative_page_size() {
        let request = ListFunctionalRolesRequest {
            page_size: Some(-1),
            page_token: None,
        };
        assert_eq!(request.page_size, Some(-1));
    }

    #[test]
    fn test_list_functional_roles_request_with_empty_token() {
        let request = ListFunctionalRolesRequest {
            page_size: Some(20),
            page_token: Some("".to_string()),
        };
        assert_eq!(request.page_token, Some("".to_string()));
    }

    #[test]
    fn test_list_functional_roles_request_with_long_token() {
        let long_token = "x".repeat(2000);
        let request = ListFunctionalRolesRequest {
            page_size: Some(100),
            page_token: Some(long_token.clone()),
        };
        assert_eq!(request.page_token, Some(long_token));
    }

    #[test]
    fn test_list_functional_roles_response_default() {
        let response = ListFunctionalRolesResponse::default();
        assert_eq!(response.roles.len(), 0);
        assert_eq!(response.has_more, None);
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn test_list_functional_roles_response_with_roles() {
        let roles = vec![
            FunctionalRole {
                role_id: Some("role_1".to_string()),
                role_name: Some("角色1".to_string()),
            },
            FunctionalRole {
                role_id: Some("role_2".to_string()),
                role_name: Some("角色2".to_string()),
            },
        ];
        let response = ListFunctionalRolesResponse {
            roles,
            has_more: Some(true),
            page_token: Some("next_page".to_string()),
        };
        assert_eq!(response.roles.len(), 2);
        assert_eq!(response.has_more, Some(true));
        assert_eq!(response.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_list_functional_roles_response_with_large_role_list() {
        let mut roles = Vec::new();
        for i in 0..1000 {
            roles.push(FunctionalRole {
                role_id: Some(format!("role_{}", i)),
                role_name: Some(format!("角色{}", i)),
            });
        }
        let response = ListFunctionalRolesResponse {
            roles,
            has_more: Some(false),
            page_token: None,
        };
        assert_eq!(response.roles.len(), 1000);
        assert_eq!(response.has_more, Some(false));
    }

    #[test]
    fn test_list_functional_roles_response_data_format() {
        assert_eq!(
            ListFunctionalRolesResponse::data_format(),
            crate::api::ResponseFormat::Data
        );
    }

    #[test]
    fn test_functional_role_default() {
        let role = FunctionalRole::default();
        assert_eq!(role.role_id, None);
        assert_eq!(role.role_name, None);
    }

    #[test]
    fn test_functional_role_construction() {
        let role = FunctionalRole {
            role_id: Some("role_789".to_string()),
            role_name: Some("超级管理员".to_string()),
        };
        assert_eq!(role.role_id, Some("role_789".to_string()));
        assert_eq!(role.role_name, Some("超级管理员".to_string()));
    }

    #[test]
    fn test_functional_role_with_empty_values() {
        let role = FunctionalRole {
            role_id: Some("".to_string()),
            role_name: Some("".to_string()),
        };
        assert_eq!(role.role_id, Some("".to_string()));
        assert_eq!(role.role_name, Some("".to_string()));
    }

    #[test]
    fn test_functional_role_with_long_values() {
        let long_id = "id_".repeat(500);
        let long_name = "name_".repeat(500);
        let role = FunctionalRole {
            role_id: Some(long_id.clone()),
            role_name: Some(long_name.clone()),
        };
        assert_eq!(role.role_id, Some(long_id));
        assert_eq!(role.role_name, Some(long_name));
    }

    #[test]
    fn test_functional_role_partial_none() {
        let role1 = FunctionalRole {
            role_id: Some("role_only_id".to_string()),
            role_name: None,
        };
        let role2 = FunctionalRole {
            role_id: None,
            role_name: Some("name_only".to_string()),
        };
        assert_eq!(role1.role_id, Some("role_only_id".to_string()));
        assert_eq!(role1.role_name, None);
        assert_eq!(role2.role_id, None);
        assert_eq!(role2.role_name, Some("name_only".to_string()));
    }

    #[test]
    fn test_delete_functional_role_response_default() {
        let response = DeleteFunctionalRoleResponse::default();
        assert!(!format!("{:?}", response).is_empty());
    }

    #[test]
    fn test_delete_functional_role_response_data_format() {
        assert_eq!(
            DeleteFunctionalRoleResponse::data_format(),
            crate::api::ResponseFormat::Data
        );
    }

    #[test]
    fn test_config_independence() {
        let config1 = Config::default();
        let config2 = Config::default();
        let service1 = FunctionalRoleService::new(config1);
        let service2 = FunctionalRoleService::new(config2);
        assert!(!format!("{:?}", service1).is_empty());
        assert!(!format!("{:?}", service2).is_empty());
    }

    #[test]
    fn test_all_structs_debug_trait() {
        let request1 = CreateFunctionalRoleRequest {
            role_name: "test".to_string(),
        };
        let request2 = UpdateFunctionalRoleRequest {
            role_name: "test".to_string(),
        };
        let request3 = ListFunctionalRolesRequest {
            page_size: Some(10),
            page_token: Some("test".to_string()),
        };
        let response1 = CreateFunctionalRoleResponse::default();
        let response2 = UpdateFunctionalRoleResponse::default();
        let response3 = GetFunctionalRoleResponse::default();
        let response4 = ListFunctionalRolesResponse::default();
        let response5 = DeleteFunctionalRoleResponse::default();
        let role = FunctionalRole::default();

        assert!(format!("{:?}", request1).contains("test"));
        assert!(format!("{:?}", request2).contains("test"));
        assert!(format!("{:?}", request3).contains("test"));
        assert!(!format!("{:?}", response1).is_empty());
        assert!(!format!("{:?}", response2).is_empty());
        assert!(!format!("{:?}", response3).is_empty());
        assert!(!format!("{:?}", response4).is_empty());
        assert!(!format!("{:?}", response5).is_empty());
        assert!(!format!("{:?}", role).is_empty());
    }
}