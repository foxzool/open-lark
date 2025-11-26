//! 设备管理 API
//!
//! 提供门禁设备列表查询功能。

use std::sync::Arc;

/// 设备管理服务
#[derive(Debug)]
pub struct DevicesService {
    config: Arc<crate::models::SecurityConfig>,
}

impl DevicesService {
    /// 创建新的设备管理服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 获取设备列表
    pub fn list(&self) -> ListDevicesBuilder {
        ListDevicesBuilder {
            config: self.config.clone(),
            page_size: Some(20),
            page_token: None,
            device_type: None,
            status: None,
            online: None,
        }
    }
}

/// 获取设备列表构建器
#[derive(Debug)]
pub struct ListDevicesBuilder {
    config: Arc<crate::models::SecurityConfig>,
    page_size: Option<i32>,
    page_token: Option<String>,
    device_type: Option<crate::models::acs::DeviceType>,
    status: Option<crate::models::acs::DeviceStatus>,
    online: Option<bool>,
}

impl ListDevicesBuilder {
    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置设备类型过滤
    pub fn device_type(mut self, device_type: crate::models::acs::DeviceType) -> Self {
        self.device_type = Some(device_type);
        self
    }

    /// 设置设备状态过滤
    pub fn status(mut self, status: crate::models::acs::DeviceStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置在线状态过滤
    pub fn online(mut self, online: bool) -> Self {
        self.online = Some(online);
        self
    }

    /// 发送请求获取设备列表
    pub async fn send(
        self,
    ) -> crate::SecurityResult<crate::models::PageResponse<crate::models::acs::DeviceInfo>> {
        let mut url = format!("{}/open-apis/acs/v1/devices", self.config.base_url);
        let mut query_params = Vec::new();

        if let Some(page_size) = self.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &self.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(device_type) = &self.device_type {
            let type_str = match device_type {
                crate::models::acs::DeviceType::CardReader => "card_reader",
                crate::models::acs::DeviceType::FaceRecognition => "face_recognition",
                crate::models::acs::DeviceType::FingerprintReader => "fingerprint_reader",
                crate::models::acs::DeviceType::DoorLock => "door_lock",
                crate::models::acs::DeviceType::Turnstile => "turnstile",
                crate::models::acs::DeviceType::Other(ref s) => s,
            };
            query_params.push(format!("device_type={}", type_str));
        }
        if let Some(status) = &self.status {
            let status_str = match status {
                crate::models::acs::DeviceStatus::Normal => "normal",
                crate::models::acs::DeviceStatus::Offline => "offline",
                crate::models::acs::DeviceStatus::Fault => "fault",
                crate::models::acs::DeviceStatus::Maintenance => "maintenance",
                crate::models::acs::DeviceStatus::Disabled => "disabled",
            };
            query_params.push(format!("status={}", status_str));
        }
        if let Some(online) = self.online {
            query_params.push(format!("online={}", if online { "true" } else { "false" }));
        }

        if !query_params.is_empty() {
            url.push_str(&format!("?{}", query_params.join("&")));
        }

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
            let api_response: crate::models::ApiResponse<
                crate::models::PageResponse<crate::models::acs::DeviceInfo>,
            > = response.json().await?;
            match api_response.data {
                Some(devices) => Ok(devices),
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
