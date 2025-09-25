use crate::{
    core::{
        api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
    },
    service::contact::models::*,
};
use serde::{Deserialize, Serialize};

/// 用户组成员服务
#[derive(Debug)]
pub struct GroupMemberService {
    config: Config,
}

impl GroupMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 添加用户组成员
    pub async fn add(
        &self,
        group_id: &str,
        req: &AddGroupMemberRequest,
    ) -> crate::core::SDKResult<AddGroupMemberResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_GROUP_MEMBERS_ADD,
                "group_id",
                group_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<AddGroupMemberResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量添加用户组成员
    pub async fn batch_add(
        &self,
        group_id: &str,
        req: &BatchAddGroupMembersRequest,
    ) -> crate::core::SDKResult<BatchAddGroupMembersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_GROUP_MEMBERS_BATCH_ADD,
                "group_id",
                group_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BatchAddGroupMembersResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询用户组成员列表
    pub async fn simplelist(
        &self,
        group_id: &str,
        _req: &ListGroupMembersRequest,
    ) -> crate::core::SDKResult<ListGroupMembersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_GROUP_MEMBERS_SIMPLELIST,
                "group_id",
                group_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp =
            Transport::<ListGroupMembersResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 移除用户组成员
    pub async fn remove(
        &self,
        group_id: &str,
        req: &RemoveGroupMemberRequest,
    ) -> crate::core::SDKResult<RemoveGroupMemberResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_GROUP_MEMBERS_REMOVE,
                "group_id",
                group_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<RemoveGroupMemberResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量移除用户组成员
    pub async fn batch_remove(
        &self,
        group_id: &str,
        req: &BatchRemoveGroupMembersRequest,
    ) -> crate::core::SDKResult<BatchRemoveGroupMembersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_GROUP_MEMBERS_BATCH_REMOVE,
                "group_id",
                group_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BatchRemoveGroupMembersResponse>::request(api_req, &self.config, None)
                .await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddGroupMemberRequest {
    pub member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddGroupMemberResponse {}

impl ApiResponseTrait for AddGroupMemberResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAddGroupMembersRequest {
    pub members: Vec<GroupMemberInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchAddGroupMembersResponse {
    pub results: Vec<GroupMemberResult>,
}

impl ApiResponseTrait for BatchAddGroupMembersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListGroupMembersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListGroupMembersResponse {
    pub memberlist: Vec<GroupMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListGroupMembersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveGroupMemberRequest {
    pub member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveGroupMemberResponse {}

impl ApiResponseTrait for RemoveGroupMemberResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRemoveGroupMembersRequest {
    pub members: Vec<GroupMemberInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchRemoveGroupMembersResponse {
    pub results: Vec<GroupMemberResult>,
}

impl ApiResponseTrait for BatchRemoveGroupMembersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    #[test]
    fn test_group_member_service_creation() {
        let config = Config::default();
        let service = GroupMemberService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_group_member_service_creation_with_custom_config() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_secret")
            .build();
        let service = GroupMemberService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_add_group_member_request_construction() {
        let request = AddGroupMemberRequest {
            member_id: "user123".to_string(),
            member_id_type: Some("user_id".to_string()),
            member_type: Some("user".to_string()),
        };
        assert_eq!(request.member_id, "user123");
        assert_eq!(request.member_id_type, Some("user_id".to_string()));
        assert_eq!(request.member_type, Some("user".to_string()));
    }

    #[test]
    fn test_add_group_member_request_with_none_values() {
        let request = AddGroupMemberRequest {
            member_id: "user456".to_string(),
            member_id_type: None,
            member_type: None,
        };
        assert_eq!(request.member_id, "user456");
        assert_eq!(request.member_id_type, None);
        assert_eq!(request.member_type, None);
    }

    #[test]
    fn test_add_group_member_request_with_empty_id() {
        let request = AddGroupMemberRequest {
            member_id: "".to_string(),
            member_id_type: Some("user_id".to_string()),
            member_type: Some("user".to_string()),
        };
        assert_eq!(request.member_id, "");
    }

    #[test]
    fn test_add_group_member_request_with_long_values() {
        let long_id = "a".repeat(1000);
        let long_type = "b".repeat(500);
        let request = AddGroupMemberRequest {
            member_id: long_id.clone(),
            member_id_type: Some(long_type.clone()),
            member_type: Some("user".to_string()),
        };
        assert_eq!(request.member_id, long_id);
        assert_eq!(request.member_id_type, Some(long_type));
    }

    #[test]
    fn test_add_group_member_response_default() {
        let response = AddGroupMemberResponse::default();
        assert!(!format!("{:?}", response).is_empty());
    }

    #[test]
    fn test_add_group_member_response_data_format() {
        assert_eq!(
            AddGroupMemberResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
    }

    #[test]
    fn test_batch_add_group_members_request_construction() {
        let members = vec![
            GroupMemberInfo {
                member_id: "user1".to_string(),
                member_id_type: Some("user_id".to_string()),
                member_type: Some("user".to_string()),
            },
            GroupMemberInfo {
                member_id: "user2".to_string(),
                member_id_type: Some("user_id".to_string()),
                member_type: Some("user".to_string()),
            },
        ];
        let request = BatchAddGroupMembersRequest { members };
        assert_eq!(request.members.len(), 2);
        assert_eq!(request.members[0].member_id, "user1");
        assert_eq!(request.members[1].member_id, "user2");
    }

    #[test]
    fn test_batch_add_group_members_request_with_empty_members() {
        let request = BatchAddGroupMembersRequest {
            members: Vec::new(),
        };
        assert_eq!(request.members.len(), 0);
    }

    #[test]
    fn test_batch_add_group_members_request_with_large_list() {
        let mut members = Vec::new();
        for i in 0..1000 {
            members.push(GroupMemberInfo {
                member_id: format!("user_{}", i),
                member_id_type: Some("user_id".to_string()),
                member_type: Some("user".to_string()),
            });
        }
        let request = BatchAddGroupMembersRequest { members };
        assert_eq!(request.members.len(), 1000);
    }

    #[test]
    fn test_batch_add_group_members_response_default() {
        let response = BatchAddGroupMembersResponse::default();
        assert_eq!(response.results.len(), 0);
    }

    #[test]
    fn test_batch_add_group_members_response_data_format() {
        assert_eq!(
            BatchAddGroupMembersResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
    }

    #[test]
    fn test_list_group_members_request_default() {
        let request = ListGroupMembersRequest::default();
        assert_eq!(request.member_id_type, None);
        assert_eq!(request.member_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_list_group_members_request_with_pagination() {
        let request = ListGroupMembersRequest {
            member_id_type: Some("user_id".to_string()),
            member_type: Some("user".to_string()),
            page_size: Some(50),
            page_token: Some("token123".to_string()),
        };
        assert_eq!(request.member_id_type, Some("user_id".to_string()));
        assert_eq!(request.member_type, Some("user".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_list_group_members_request_with_large_page_size() {
        let request = ListGroupMembersRequest {
            member_id_type: None,
            member_type: None,
            page_size: Some(10000),
            page_token: None,
        };
        assert_eq!(request.page_size, Some(10000));
    }

    #[test]
    fn test_list_group_members_request_with_negative_page_size() {
        let request = ListGroupMembersRequest {
            member_id_type: None,
            member_type: None,
            page_size: Some(-1),
            page_token: None,
        };
        assert_eq!(request.page_size, Some(-1));
    }

    #[test]
    fn test_list_group_members_request_with_empty_values() {
        let request = ListGroupMembersRequest {
            member_id_type: Some("".to_string()),
            member_type: Some("".to_string()),
            page_size: Some(0),
            page_token: Some("".to_string()),
        };
        assert_eq!(request.member_id_type, Some("".to_string()));
        assert_eq!(request.member_type, Some("".to_string()));
        assert_eq!(request.page_size, Some(0));
        assert_eq!(request.page_token, Some("".to_string()));
    }

    #[test]
    fn test_list_group_members_response_default() {
        let response = ListGroupMembersResponse::default();
        assert_eq!(response.memberlist.len(), 0);
        assert_eq!(response.has_more, None);
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn test_list_group_members_response_with_members() {
        let members = vec![
            GroupMember {
                member_id: Some("member1".to_string()),
                member_type: Some("user".to_string()),
                member_id_type: Some("user_id".to_string()),
            },
            GroupMember {
                member_id: Some("member2".to_string()),
                member_type: Some("user".to_string()),
                member_id_type: Some("user_id".to_string()),
            },
        ];
        let response = ListGroupMembersResponse {
            memberlist: members,
            has_more: Some(true),
            page_token: Some("next_page".to_string()),
        };
        assert_eq!(response.memberlist.len(), 2);
        assert_eq!(response.has_more, Some(true));
        assert_eq!(response.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_list_group_members_response_data_format() {
        assert_eq!(
            ListGroupMembersResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
    }

    #[test]
    fn test_remove_group_member_request_construction() {
        let request = RemoveGroupMemberRequest {
            member_id: "user789".to_string(),
            member_id_type: Some("user_id".to_string()),
            member_type: Some("user".to_string()),
        };
        assert_eq!(request.member_id, "user789");
        assert_eq!(request.member_id_type, Some("user_id".to_string()));
        assert_eq!(request.member_type, Some("user".to_string()));
    }

    #[test]
    fn test_remove_group_member_request_with_none_values() {
        let request = RemoveGroupMemberRequest {
            member_id: "user101".to_string(),
            member_id_type: None,
            member_type: None,
        };
        assert_eq!(request.member_id, "user101");
        assert_eq!(request.member_id_type, None);
        assert_eq!(request.member_type, None);
    }

    #[test]
    fn test_remove_group_member_response_default() {
        let response = RemoveGroupMemberResponse::default();
        assert!(!format!("{:?}", response).is_empty());
    }

    #[test]
    fn test_remove_group_member_response_data_format() {
        assert_eq!(
            RemoveGroupMemberResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
    }

    #[test]
    fn test_batch_remove_group_members_request_construction() {
        let members = vec![
            GroupMemberInfo {
                member_id: "user1".to_string(),
                member_id_type: Some("user_id".to_string()),
                member_type: Some("user".to_string()),
            },
            GroupMemberInfo {
                member_id: "user2".to_string(),
                member_id_type: Some("user_id".to_string()),
                member_type: Some("user".to_string()),
            },
        ];
        let request = BatchRemoveGroupMembersRequest { members };
        assert_eq!(request.members.len(), 2);
        assert_eq!(request.members[0].member_id, "user1");
        assert_eq!(request.members[1].member_id, "user2");
    }

    #[test]
    fn test_batch_remove_group_members_request_with_empty_members() {
        let request = BatchRemoveGroupMembersRequest {
            members: Vec::new(),
        };
        assert_eq!(request.members.len(), 0);
    }

    #[test]
    fn test_batch_remove_group_members_response_default() {
        let response = BatchRemoveGroupMembersResponse::default();
        assert_eq!(response.results.len(), 0);
    }

    #[test]
    fn test_batch_remove_group_members_response_data_format() {
        assert_eq!(
            BatchRemoveGroupMembersResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
    }

    #[test]
    fn test_config_independence() {
        let config1 = Config::default();
        let config2 = Config::default();
        let service1 = GroupMemberService::new(config1);
        let service2 = GroupMemberService::new(config2);
        assert!(!format!("{:?}", service1).is_empty());
        assert!(!format!("{:?}", service2).is_empty());
    }

    #[test]
    fn test_all_structs_debug_trait() {
        let add_request = AddGroupMemberRequest {
            member_id: "test".to_string(),
            member_id_type: Some("test".to_string()),
            member_type: Some("test".to_string()),
        };
        let batch_add_request = BatchAddGroupMembersRequest {
            members: vec![GroupMemberInfo {
                member_id: "test".to_string(),
                member_id_type: Some("test".to_string()),
                member_type: Some("test".to_string()),
            }],
        };
        let list_request = ListGroupMembersRequest::default();
        let remove_request = RemoveGroupMemberRequest {
            member_id: "test".to_string(),
            member_id_type: Some("test".to_string()),
            member_type: Some("test".to_string()),
        };
        let batch_remove_request = BatchRemoveGroupMembersRequest {
            members: vec![GroupMemberInfo {
                member_id: "test".to_string(),
                member_id_type: Some("test".to_string()),
                member_type: Some("test".to_string()),
            }],
        };

        assert!(format!("{:?}", add_request).contains("test"));
        assert!(format!("{:?}", batch_add_request).contains("test"));
        assert!(!format!("{:?}", list_request).is_empty());
        assert!(format!("{:?}", remove_request).contains("test"));
        assert!(format!("{:?}", batch_remove_request).contains("test"));

        let add_response = AddGroupMemberResponse::default();
        let batch_add_response = BatchAddGroupMembersResponse::default();
        let list_response = ListGroupMembersResponse::default();
        let remove_response = RemoveGroupMemberResponse::default();
        let batch_remove_response = BatchRemoveGroupMembersResponse::default();

        assert!(!format!("{:?}", add_response).is_empty());
        assert!(!format!("{:?}", batch_add_response).is_empty());
        assert!(!format!("{:?}", list_response).is_empty());
        assert!(!format!("{:?}", remove_response).is_empty());
        assert!(!format!("{:?}", batch_remove_response).is_empty());
    }
}
