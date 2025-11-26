//! 人脸识别管理 API
//!
//! 提供用户人脸图片的上传和下载功能。

use std::sync::Arc;

/// 人脸识别管理服务
#[derive(Debug)]
pub struct UserFacesService {
    config: Arc<crate::models::SecurityConfig>,
}

impl UserFacesService {
    /// 创建新的人脸识别管理服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 下载人脸图片
    pub fn get(&self) -> GetUserFaceBuilder {
        GetUserFaceBuilder {
            config: self.config.clone(),
            user_id: String::new(),
        }
    }

    /// 上传人脸图片
    pub fn update(&self) -> UpdateUserFaceBuilder {
        UpdateUserFaceBuilder {
            config: self.config.clone(),
            user_id: String::new(),
            face_image: Vec::new(),
            image_format: "jpeg".to_string(),
        }
    }
}

/// 获取用户人脸图片构建器
#[derive(Debug)]
pub struct GetUserFaceBuilder {
    config: Arc<crate::models::SecurityConfig>,
    user_id: String,
}

impl GetUserFaceBuilder {
    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 发送请求下载人脸图片
    pub async fn send(self) -> crate::SecurityResult<crate::models::acs::FaceImageInfo> {
        let url = format!(
            "{}/open-apis/acs/v1/users/{}/face",
            self.config.base_url, self.user_id
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::acs::FaceImageInfo> =
                response.json().await?;
            match api_response.data {
                Some(face_info) => Ok(face_info),
                None => Err(crate::SecurityError::APIError {
                    code: api_response.code,
                    message: api_response.msg,
                }),
            }
        } else {
            Err(crate::SecurityError::APIError {
                code: response.status().as_u16() as i32,
                message: format!(
                    "HTTP {}: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            })
        }
    }
}

/// 更新用户人脸图片构建器
#[derive(Debug)]
pub struct UpdateUserFaceBuilder {
    config: Arc<crate::models::SecurityConfig>,
    user_id: String,
    face_image: Vec<u8>,
    image_format: String,
}

impl UpdateUserFaceBuilder {
    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置人脸图片数据
    pub fn face_image(mut self, face_image: Vec<u8>) -> Self {
        self.face_image = face_image;
        self
    }

    /// 设置图片格式
    pub fn image_format(mut self, image_format: impl Into<String>) -> Self {
        self.image_format = image_format.into();
        self
    }

    /// 从文件路径加载人脸图片
    pub async fn face_image_from_file(
        mut self,
        file_path: impl AsRef<std::path::Path>,
    ) -> crate::SecurityResult<Self> {
        use std::fs;
        let image_data = fs::read(file_path).map_err(|e| {
            crate::SecurityError::DeviceError(format!("Failed to read image file: {}", e))
        })?;
        self.face_image = image_data;
        Ok(self)
    }

    /// 发送请求上传人脸图片
    pub async fn send(self) -> crate::SecurityResult<crate::models::acs::FaceImageInfo> {
        let url = format!(
            "{}/open-apis/acs/v1/users/{}/face",
            self.config.base_url, self.user_id
        );

        // 创建 multipart 表单数据
        let form = reqwest::multipart::Form::new().part(
            "image",
            reqwest::multipart::Part::bytes(self.face_image)
                .file_name(format!("face_image.{}", self.image_format))
                .mime_str(&format!("image/{}", self.image_format))
                .map_err(|e| {
                    crate::SecurityError::DeviceError(format!("Invalid mime type: {}", e))
                })?,
        );

        let response = reqwest::Client::new()
            .put(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::acs::FaceImageInfo> =
                response.json().await?;
            match api_response.data {
                Some(face_info) => Ok(face_info),
                None => Err(crate::SecurityError::APIError {
                    code: api_response.code,
                    message: api_response.msg,
                }),
            }
        } else {
            Err(crate::SecurityError::APIError {
                code: response.status().as_u16() as i32,
                message: format!(
                    "HTTP {}: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            })
        }
    }
}

/// 获取应用访问令牌的辅助函数
async fn get_app_token(_config: &crate::models::SecurityConfig) -> crate::SecurityResult<String> {
    // 这里应该调用认证服务获取应用访问令牌
    // 为了简化实现，暂时返回占位符
    // TODO: 集成 openlark-auth 服务
    Ok("placeholder_app_token".to_string())
}
