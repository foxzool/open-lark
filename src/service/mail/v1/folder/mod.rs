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
    service::mail::models::{Folder, UserIdType},
};

/// 邮箱文件夹服务
pub struct FolderService {
    pub config: Config,
}

/// 创建邮箱文件夹请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    /// 文件夹名称
    pub folder_name: String,
    /// 上级文件夹ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

/// 创建邮箱文件夹响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFolderResponse {
    /// 创建的文件夹信息
    pub folder: Folder,
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 修改邮箱文件夹请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFolderRequest {
    /// 文件夹名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_name: Option<String>,
    /// 上级文件夹ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

/// 修改邮箱文件夹响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFolderResponse {
    /// 修改后的文件夹信息
    pub folder: Folder,
}

impl ApiResponseTrait for UpdateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出邮箱文件夹响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListFoldersResponse {
    /// 文件夹列表
    pub folders: Vec<Folder>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListFoldersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl FolderService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建邮箱文件夹
    ///
    /// 该接口用于创建用户邮箱中的文件夹，可以指定上级文件夹ID来创建子文件夹。
    ///
    /// # 参数
    ///
    /// - `user_mailbox_id`: 用户邮箱ID
    /// - `request`: 创建文件夹请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 邮箱不存在
    pub async fn create(
        &self,
        user_mailbox_id: &str,
        request: CreateFolderRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateFolderResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_FOLDERS,
                "user_mailbox_id",
                user_mailbox_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除邮箱文件夹
    ///
    /// 该接口用于删除指定的邮箱文件夹。
    ///
    /// # 参数
    ///
    /// - `user_mailbox_id`: 用户邮箱ID
    /// - `folder_id`: 文件夹ID
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 文件夹不存在
    pub async fn delete(
        &self,
        user_mailbox_id: &str,
        folder_id: &str,
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
                &EndpointBuilder::replace_param(
                    Endpoints::MAIL_V1_USER_MAILBOX_FOLDER,
                    "user_mailbox_id",
                    user_mailbox_id,
                ),
                "folder_id",
                folder_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 修改邮箱文件夹
    ///
    /// 该接口用于修改指定的邮箱文件夹信息，如名称、上级文件夹等。
    ///
    /// # 参数
    ///
    /// - `user_mailbox_id`: 用户邮箱ID
    /// - `folder_id`: 文件夹ID
    /// - `request`: 修改文件夹请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 文件夹不存在
    pub async fn patch(
        &self,
        user_mailbox_id: &str,
        folder_id: &str,
        request: UpdateFolderRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateFolderResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                &EndpointBuilder::replace_param(
                    Endpoints::MAIL_V1_USER_MAILBOX_FOLDER,
                    "user_mailbox_id",
                    user_mailbox_id,
                ),
                "folder_id",
                folder_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 列出邮箱文件夹
    ///
    /// 该接口用于获取用户邮箱中的所有文件夹列表。
    ///
    /// # 参数
    ///
    /// - `user_mailbox_id`: 用户邮箱ID
    /// - `page_size`: 分页大小，最大值100
    /// - `page_token`: 分页标识
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 邮箱不存在
    pub async fn list(
        &self,
        user_mailbox_id: &str,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListFoldersResponse>> {
        let mut query_params = HashMap::new();
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_FOLDERS,
                "user_mailbox_id",
                user_mailbox_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
