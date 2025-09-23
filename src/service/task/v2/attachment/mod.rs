use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::task::models::{Attachment, UserIdType},
};

/// 附件服务
#[derive(Debug)]
pub struct AttachmentService {
    pub config: Config,
}

/// 上传附件请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadAttachmentRequest {
    /// 文件名
    pub name: String,
    /// 文件大小
    pub size: i64,
    /// 父类型 (task)
    pub parent_type: String,
    /// 父ID (任务GUID)
    pub parent_id: String,
}

/// 上传附件响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadAttachmentResponse {
    /// 附件
    pub attachment: Attachment,
}

impl ApiResponseTrait for UploadAttachmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 附件列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListAttachmentsResponse {
    /// 附件列表
    pub items: Vec<Attachment>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListAttachmentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取附件响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAttachmentResponse {
    /// 附件详情
    pub attachment: Attachment,
}

impl ApiResponseTrait for GetAttachmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AttachmentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 上传附件
    pub async fn upload(
        &self,
        request: UploadAttachmentRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UploadAttachmentResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::TASK_V2_ATTACHMENTS_UPLOAD.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 列取附件
    pub async fn list(
        &self,
        parent_type: &str,
        parent_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListAttachmentsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        query_params.insert("parent_type", parent_type.to_string());
        query_params.insert("parent_id", parent_id.to_string());
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::TASK_V2_ATTACHMENTS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取附件
    pub async fn get(
        &self,
        attachment_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetAttachmentResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_ATTACHMENT_GET,
                "attachment_guid",
                attachment_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除附件
    pub async fn delete(
        &self,
        attachment_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_ATTACHMENT_GET,
                "attachment_guid",
                attachment_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_attachment_service_creation() {
        let config = create_test_config();
        let service = AttachmentService::new(config);
    }

    #[test]
    fn test_upload_attachment_request_serialization() {
        let request = UploadAttachmentRequest {
            name: "test_file.pdf".to_string(),
            size: 1024,
            parent_type: "task".to_string(),
            parent_id: "task_123".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: UploadAttachmentRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.size, deserialized.size);
        assert_eq!(request.parent_type, deserialized.parent_type);
        assert_eq!(request.parent_id, deserialized.parent_id);
    }

    #[test]
    fn test_upload_attachment_response_serialization() {
        let attachment = Attachment {
            guid: Some("attachment_123".to_string()),
            name: Some("test.pdf".to_string()),
            size: Some(1024),
            ..Default::default()
        };

        let response = UploadAttachmentResponse { attachment };
        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: UploadAttachmentResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.attachment.guid, deserialized.attachment.guid);
        assert_eq!(response.attachment.name, deserialized.attachment.name);
        assert_eq!(response.attachment.size, deserialized.attachment.size);
    }

    #[test]
    fn test_list_attachments_response_serialization() {
        let attachment = Attachment {
            guid: Some("attachment_123".to_string()),
            name: Some("test.pdf".to_string()),
            ..Default::default()
        };

        let response = ListAttachmentsResponse {
            items: vec![attachment],
            page_token: Some("next_page".to_string()),
            has_more: Some(true),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ListAttachmentsResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.items.len(), deserialized.items.len());
        assert_eq!(response.page_token, deserialized.page_token);
        assert_eq!(response.has_more, deserialized.has_more);
    }

    #[test]
    fn test_get_attachment_response_serialization() {
        let attachment = Attachment {
            guid: Some("attachment_123".to_string()),
            name: Some("test.pdf".to_string()),
            size: Some(2048),
            ..Default::default()
        };

        let response = GetAttachmentResponse { attachment };
        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetAttachmentResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.attachment.guid, deserialized.attachment.guid);
        assert_eq!(response.attachment.name, deserialized.attachment.name);
        assert_eq!(response.attachment.size, deserialized.attachment.size);
    }

    #[test]
    fn test_response_format_traits() {
        assert_eq!(
            UploadAttachmentResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(ListAttachmentsResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetAttachmentResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_upload_attachment_request_fields() {
        let request = UploadAttachmentRequest {
            name: "document.docx".to_string(),
            size: 5120,
            parent_type: "task".to_string(),
            parent_id: "task_456".to_string(),
        };

        // Test field access
        assert_eq!(request.name, "document.docx");
        assert_eq!(request.size, 5120);
        assert_eq!(request.parent_type, "task");
        assert_eq!(request.parent_id, "task_456");
    }

    #[test]
    fn test_list_attachments_response_optional_fields() {
        // Test with optional fields as None
        let response = ListAttachmentsResponse {
            items: vec![],
            page_token: None,
            has_more: None,
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ListAttachmentsResponse = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.items.is_empty());
        assert!(deserialized.page_token.is_none());
        assert!(deserialized.has_more.is_none());
    }

    #[test]
    fn test_attachment_service_debug() {
        let config = create_test_config();
        let service = AttachmentService::new(config);

        // Test Debug trait implementation
        let debug_string = format!("{:?}", service);
        assert!(debug_string.contains("AttachmentService"));
    }
}
