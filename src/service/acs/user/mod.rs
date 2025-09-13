use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::acs::models::{AcsUser, FaceImage, PageResponse, UserStatus, UserType},
};

/// 门禁用户管理服务
pub struct UserService {
    pub config: Config,
}

impl UserService {
    /// 创建门禁用户管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 修改用户部分信息
    ///
    /// 修改指定用户的部分信息，如姓名、部门、状态等。
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 用户信息修改请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回修改后的用户信息
    pub async fn patch_user(
        &self,
        user_id: &str,
        request: UserPatchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserPatchResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!("/open-apis/acs/v1/users/{user_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取单个用户信息
    ///
    /// 根据用户ID获取门禁用户的详细信息。
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回用户详细信息
    pub async fn get_user(
        &self,
        user_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/acs/v1/users/{user_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取用户列表
    ///
    /// 分页获取门禁用户列表，支持多种筛选条件。
    ///
    /// # Arguments
    ///
    /// * `request` - 用户列表查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回用户列表
    pub async fn list_users(
        &self,
        request: UserListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/acs/v1/users".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(user_type) = request.user_type {
            api_req
                .query_params
                .insert("user_type", serde_json::to_string(&user_type)?);
        }

        if let Some(status) = request.status {
            api_req
                .query_params
                .insert("status", serde_json::to_string(&status)?);
        }

        if let Some(department) = request.department {
            api_req.query_params.insert("department", department);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 上传人脸图片
    ///
    /// 为指定用户上传人脸图片，用于人脸识别门禁。
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 人脸图片上传请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回上传结果
    pub async fn upload_face_image(
        &self,
        user_id: &str,
        request: FaceImageUploadRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FaceImageUploadResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: format!("/open-apis/acs/v1/users/{user_id}/face_image"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 下载人脸图片
    ///
    /// 下载指定用户的人脸图片。
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回人脸图片信息
    pub async fn download_face_image(
        &self,
        user_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FaceImageDownloadResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/acs/v1/users/{user_id}/face_image"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 用户信息修改请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPatchRequest {
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    /// 用户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// 电话号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 邮箱地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// 用户信息修改响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPatchResponse {
    /// 修改后的用户信息
    pub user: AcsUser,
}

impl ApiResponseTrait for UserPatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户详情查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserGetResponse {
    /// 用户详细信息
    pub user: AcsUser,
}

impl ApiResponseTrait for UserGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 用户类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    /// 用户状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 部门筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
}

/// 用户列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponse {
    /// 用户列表
    #[serde(flatten)]
    pub users: PageResponse<AcsUser>,
}

impl ApiResponseTrait for UserListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人脸图片上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceImageUploadRequest {
    /// 图片内容 (base64编码)
    pub image_content: String,
    /// 图片格式 (如: jpg, png)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_format: Option<String>,
}

/// 人脸图片上传响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FaceImageUploadResponse {
    /// 上传成功标识
    pub success: bool,
    /// 图片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

impl ApiResponseTrait for FaceImageUploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人脸图片下载响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FaceImageDownloadResponse {
    /// 人脸图片信息
    pub face_image: FaceImage,
}

impl ApiResponseTrait for FaceImageDownloadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
