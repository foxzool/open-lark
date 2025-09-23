//! # 实名认证服务
//!
//! 飞书实名认证服务提供身份验证和人脸识别功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **身份信息录入**：录入用户身份证、姓名等基本信息
//! - **人脸图片管理**：上传和裁剪人脸基准图片
//! - **身份验证**：基于人脸识别的身份验证流程
//! - **结果查询**：查询人脸认证进度和结果
//!
//! ## 安全说明
//!
//! 本服务处理敏感的身份信息和生物识别数据，请确保：
//! - 遵循相关法律法规和隐私保护要求
//! - 妥善保管和传输敏感数据
//! - 仅在合法合规的场景下使用
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - 身份信息录入 - 创建和管理身份认证记录
//! - 人脸图片处理 - 上传基准图片和图片裁剪
//! - 认证结果查询 - 查询验证进度和结果
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::human_authentication::*;
//! use open_lark::service::human_authentication::models::{IdType, ImageType};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 录入身份信息
//!     let identity_request = IdentityCreateRequest {
//!         name: "张三".to_string(),
//!         id_number: "110101199001010001".to_string(),
//!         id_type: Some(IdType::IdCard),
//!     };
//!     
//!     let identity_response = client.human_authentication
//!         .create_identity(identity_request, None).await?;
//!     
//!     // 上传人脸基准图片
//!     let upload_request = FaceImageUploadRequest {
//!         identity_id: identity_response.data.unwrap().identity_id,
//!         image_content: "base64_encoded_image".to_string(),
//!         image_type: ImageType::Jpeg,
//!     };
//!     
//!     let upload_response = client.human_authentication
//!         .upload_face_image(upload_request, None).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod models;

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
    service::human_authentication::models::*,
};

/// 实名认证服务
///
/// 提供身份验证、人脸识别等实名认证功能
pub struct HumanAuthenticationService {
    pub config: Config,
}

impl HumanAuthenticationService {
    /// 创建实名认证服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 录入身份信息
    ///
    /// 创建新的身份认证记录，录入用户的身份证号、姓名等基本信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 身份信息创建请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回创建的身份认证记录信息
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::human_authentication::*;
    /// use open_lark::service::human_authentication::models::IdType;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret")
    ///         .build();
    ///
    ///     let request = IdentityCreateRequest {
    ///         name: "张三".to_string(),
    ///         id_number: "110101199001010001".to_string(),
    ///         id_type: Some(IdType::IdCard),
    ///     };
    ///
    ///     let response = client.human_authentication.create_identity(request, None).await?;
    ///     println!("身份记录ID: {}", response.data.unwrap().identity_id);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_identity(
        &self,
        request: IdentityCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<IdentityCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::HUMAN_AUTHENTICATION_V1_IDENTITIES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 上传人脸基准图片
    ///
    /// 为指定的身份记录上传人脸基准图片，用于后续的人脸识别比对。
    ///
    /// # Arguments
    ///
    /// * `request` - 人脸图片上传请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回上传的图片信息
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::human_authentication::*;
    /// use open_lark::service::human_authentication::models::ImageType;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret")
    ///         .build();
    ///
    ///     let request = FaceImageUploadRequest {
    ///         identity_id: "identity_123".to_string(),
    ///         image_content: "base64_encoded_image_data".to_string(),
    ///         image_type: ImageType::Jpeg,
    ///     };
    ///
    ///     let response = client.human_authentication.upload_face_image(request, None).await?;
    ///     println!("图片ID: {}", response.data.unwrap().image_id);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn upload_face_image(
        &self,
        request: FaceImageUploadRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FaceImageUploadResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::HUMAN_AUTHENTICATION_V1_FACE_IMAGES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 裁剪人脸图片
    ///
    /// 对上传的人脸图片进行裁剪处理，提取人脸区域以提高识别准确率。
    ///
    /// # Arguments
    ///
    /// * `request` - 图片裁剪请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回裁剪后的图片信息
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::human_authentication::*;
    /// use open_lark::service::human_authentication::models::CropParameters;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret")
    ///         .build();
    ///
    ///     let request = FaceImageCropRequest {
    ///         image_id: "image_123".to_string(),
    ///         crop_params: Some(CropParameters {
    ///             x: 100,
    ///             y: 100,
    ///             width: 200,
    ///             height: 200,
    ///         }),
    ///     };
    ///
    ///     let response = client.human_authentication.crop_face_image(request, None).await?;
    ///     println!("裁剪后图片ID: {}", response.data.unwrap().cropped_image_id);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn crop_face_image(
        &self,
        request: FaceImageCropRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FaceImageCropResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::HUMAN_AUTHENTICATION_V1_FACE_IMAGES_CROP.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询人脸认证结果
    ///
    /// 查询指定身份记录的人脸认证进度和结果。
    ///
    /// # Arguments
    ///
    /// * `identity_id` - 身份记录ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回认证结果信息
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::human_authentication::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret")
    ///         .build();
    ///
    ///     let response = client.human_authentication
    ///         .query_authentication_result("identity_123", None).await?;
    ///     
    ///     if let Some(data) = response.data {
    ///         println!("认证状态: {:?}", data.status);
    ///         println!("置信度: {:?}", data.confidence_score);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query_authentication_result(
        &self,
        identity_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AuthenticationResultResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::HUMAN_AUTHENTICATION_V1_IDENTITY_RESULT,
                "identity_id",
                identity_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 身份信息创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityCreateRequest {
    /// 真实姓名
    pub name: String,
    /// 身份证号码
    pub id_number: String,
    /// 证件类型，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<IdType>,
}

/// 身份信息创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityCreateResponse {
    /// 身份记录ID
    pub identity_id: String,
    /// 创建时间戳
    pub created_at: i64,
}

impl ApiResponseTrait for IdentityCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人脸图片上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceImageUploadRequest {
    /// 身份记录ID
    pub identity_id: String,
    /// 图片内容 (base64编码)
    pub image_content: String,
    /// 图片类型
    pub image_type: ImageType,
}

/// 人脸图片上传响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FaceImageUploadResponse {
    /// 图片ID
    pub image_id: String,
    /// 上传时间戳
    pub uploaded_at: i64,
}

impl ApiResponseTrait for FaceImageUploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人脸图片裁剪请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceImageCropRequest {
    /// 图片ID
    pub image_id: String,
    /// 裁剪参数，可选，不提供则自动检测人脸区域
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_params: Option<CropParameters>,
}

/// 人脸图片裁剪响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FaceImageCropResponse {
    /// 裁剪后的图片ID
    pub cropped_image_id: String,
    /// 裁剪时间戳
    pub cropped_at: i64,
    /// 检测到的人脸区域信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_face: Option<FaceRegion>,
}

impl ApiResponseTrait for FaceImageCropResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 认证结果查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationResultResponse {
    /// 身份记录ID
    pub identity_id: String,
    /// 认证状态
    pub status: AuthenticationStatus,
    /// 置信度分数 (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f64>,
    /// 认证完成时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ApiResponseTrait for AuthenticationResultResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_human_authentication_service_creation() {
        let config = Config::default();
        let service = HumanAuthenticationService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_human_authentication_service_with_custom_config() {
        let config = Config::builder()
            .app_id("human_auth_test_app")
            .app_secret("human_auth_test_secret")
            .req_timeout(Duration::from_secs(120))
            .build();

        let service = HumanAuthenticationService::new(config.clone());

        assert_eq!(service.config.app_id, "human_auth_test_app");
        assert_eq!(service.config.app_secret, "human_auth_test_secret");
        assert_eq!(service.config.req_timeout, Some(Duration::from_secs(120)));
    }

    #[test]
    fn test_human_authentication_service_config_independence() {
        let config1 = Config::builder().app_id("human_auth_app_1").build();

        let config2 = Config::builder().app_id("human_auth_app_2").build();

        let service1 = HumanAuthenticationService::new(config1);
        let service2 = HumanAuthenticationService::new(config2);

        assert_eq!(service1.config.app_id, "human_auth_app_1");
        assert_eq!(service2.config.app_id, "human_auth_app_2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_human_authentication_service_accessible() {
        let config = Config::default();
        let service = HumanAuthenticationService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
    }

    #[test]
    fn test_human_authentication_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = HumanAuthenticationService::new(config.clone());

        assert_eq!(service.config.app_id, "clone_test_app");
        assert_eq!(service.config.app_secret, "clone_test_secret");
    }

    #[test]
    fn test_human_authentication_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(150))
            .build();

        let service = HumanAuthenticationService::new(config);

        assert_eq!(service.config.req_timeout, Some(Duration::from_secs(150)));
    }

    #[test]
    fn test_human_authentication_service_multiple_instances() {
        let config = Config::default();

        let service1 = HumanAuthenticationService::new(config.clone());
        let service2 = HumanAuthenticationService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);
    }

    #[test]
    fn test_human_authentication_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = HumanAuthenticationService::new(config);

        assert_eq!(service.config.app_id, "consistency_test");
        assert_eq!(service.config.app_secret, "consistency_secret");
        assert_eq!(service.config.req_timeout, Some(Duration::from_secs(180)));
    }
}
