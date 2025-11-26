//! 设备记录管理 API
//!
//! 提供设备的增删改查功能，支持设备记录的生命周期管理。

use std::sync::Arc;

/// 设备记录管理服务
#[derive(Debug)]
pub struct DeviceRecordsService {
    config: Arc<crate::models::SecurityConfig>,
}

impl DeviceRecordsService {
    /// 创建新的设备记录管理服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 获取我的设备认证信息
    pub fn mine(&self) -> GetMyDeviceRecordsBuilder {
        GetMyDeviceRecordsBuilder {
            config: self.config.clone(),
            page_size: Some(20),
            page_token: None,
            status: None,
        }
    }

    /// 新增设备记录
    pub fn create(&self) -> CreateDeviceRecordBuilder {
        CreateDeviceRecordBuilder {
            config: self.config.clone(),
            device_name: String::new(),
            device_type: String::new(),
            device_brand: None,
            device_model: None,
            os_type: None,
            os_version: None,
            serial_number: None,
            mac_address: None,
            personal_device: None,
            extension: None,
        }
    }

    /// 查询设备信息
    pub fn list(&self) -> ListDeviceRecordsBuilder {
        ListDeviceRecordsBuilder {
            config: self.config.clone(),
            page_size: Some(20),
            page_token: None,
            user_id: None,
            device_type: None,
            status: None,
            personal_device: None,
            compliance_status: None,
        }
    }

    /// 获取设备信息
    pub fn get(&self) -> GetDeviceRecordBuilder {
        GetDeviceRecordBuilder {
            config: self.config.clone(),
            device_record_id: String::new(),
        }
    }

    /// 更新设备信息
    pub fn update(&self) -> UpdateDeviceRecordBuilder {
        UpdateDeviceRecordBuilder {
            config: self.config.clone(),
            device_record_id: String::new(),
            device_name: None,
            device_brand: None,
            device_model: None,
            os_type: None,
            os_version: None,
            serial_number: None,
            mac_address: None,
            compliance_status: None,
            extension: None,
        }
    }

    /// 删除设备
    pub fn delete(&self) -> DeleteDeviceRecordBuilder {
        DeleteDeviceRecordBuilder {
            config: self.config.clone(),
            device_record_id: String::new(),
        }
    }
}

/// 获取我的设备认证信息构建器
#[derive(Debug)]
pub struct GetMyDeviceRecordsBuilder {
    config: Arc<crate::models::SecurityConfig>,
    page_size: Option<i32>,
    page_token: Option<String>,
    status: Option<crate::models::security_and_compliance::DeviceRecordStatus>,
}

impl GetMyDeviceRecordsBuilder {
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

    /// 设置状态过滤
    pub fn status(
        mut self,
        status: crate::models::security_and_compliance::DeviceRecordStatus,
    ) -> Self {
        self.status = Some(status);
        self
    }

    /// 发送请求获取我的设备认证信息
    pub async fn send(
        self,
    ) -> crate::SecurityResult<
        crate::models::PageResponse<crate::models::security_and_compliance::DeviceRecord>,
    > {
        let url = format!(
            "{}/open-apis/security_and_compliance/v2/device_records/mine",
            self.config.base_url
        );

        let mut query_params = Vec::new();

        if let Some(page_size) = self.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &self.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(status) = &self.status {
            let status_str = match status {
                crate::models::security_and_compliance::DeviceRecordStatus::Pending => "pending",
                crate::models::security_and_compliance::DeviceRecordStatus::Approved => "approved",
                crate::models::security_and_compliance::DeviceRecordStatus::Rejected => "rejected",
                crate::models::security_and_compliance::DeviceRecordStatus::Expired => "expired",
                crate::models::security_and_compliance::DeviceRecordStatus::Revoked => "revoked",
                crate::models::security_and_compliance::DeviceRecordStatus::NonCompliant => {
                    "non_compliant"
                }
            };
            query_params.push(format!("status={}", status_str));
        }

        let url_with_params = if !query_params.is_empty() {
            format!("{}?{}", url, query_params.join("&"))
        } else {
            url
        };

        let response = reqwest::Client::new()
            .get(&url_with_params)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<
                crate::models::PageResponse<crate::models::security_and_compliance::DeviceRecord>,
            > = response.json().await?;
            match api_response.data {
                Some(records) => Ok(records),
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

/// 新增设备记录构建器
#[derive(Debug)]
pub struct CreateDeviceRecordBuilder {
    config: Arc<crate::models::SecurityConfig>,
    device_name: String,
    device_type: String,
    device_brand: Option<String>,
    device_model: Option<String>,
    os_type: Option<String>,
    os_version: Option<String>,
    serial_number: Option<String>,
    mac_address: Option<String>,
    personal_device: Option<bool>,
    extension: Option<crate::models::ExtensionMap>,
}

impl CreateDeviceRecordBuilder {
    /// 设置设备名称
    pub fn device_name(mut self, device_name: impl Into<String>) -> Self {
        self.device_name = device_name.into();
        self
    }

    /// 设置设备类型
    pub fn device_type(mut self, device_type: impl Into<String>) -> Self {
        self.device_type = device_type.into();
        self
    }

    /// 设置设备品牌
    pub fn device_brand(mut self, device_brand: impl Into<String>) -> Self {
        self.device_brand = Some(device_brand.into());
        self
    }

    /// 设置设备型号
    pub fn device_model(mut self, device_model: impl Into<String>) -> Self {
        self.device_model = Some(device_model.into());
        self
    }

    /// 设置操作系统
    pub fn os_type(mut self, os_type: impl Into<String>) -> Self {
        self.os_type = Some(os_type.into());
        self
    }

    /// 设置操作系统版本
    pub fn os_version(mut self, os_version: impl Into<String>) -> Self {
        self.os_version = Some(os_version.into());
        self
    }

    /// 设置设备序列号
    pub fn serial_number(mut self, serial_number: impl Into<String>) -> Self {
        self.serial_number = Some(serial_number.into());
        self
    }

    /// 设置MAC地址
    pub fn mac_address(mut self, mac_address: impl Into<String>) -> Self {
        self.mac_address = Some(mac_address.into());
        self
    }

    /// 设置是否为个人设备
    pub fn personal_device(mut self, personal_device: bool) -> Self {
        self.personal_device = Some(personal_device);
        self
    }

    /// 设置扩展信息
    pub fn extension(mut self, extension: crate::models::ExtensionMap) -> Self {
        self.extension = Some(extension);
        self
    }

    /// 发送请求新增设备记录
    pub async fn send(
        self,
    ) -> crate::SecurityResult<crate::models::security_and_compliance::DeviceRecord> {
        let url = format!(
            "{}/open-apis/security_and_compliance/v2/device_records",
            self.config.base_url
        );

        let request_body = crate::models::security_and_compliance::DeviceRecordRequest {
            device_name: self.device_name,
            device_type: self.device_type,
            device_brand: self.device_brand,
            device_model: self.device_model,
            os_type: self.os_type,
            os_version: self.os_version,
            serial_number: self.serial_number,
            mac_address: self.mac_address,
            personal_device: self.personal_device,
            extension: self.extension,
        };

        let response = reqwest::Client::new()
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<
                crate::models::security_and_compliance::DeviceRecord,
            > = response.json().await?;
            match api_response.data {
                Some(record) => Ok(record),
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

/// 查询设备信息构建器
#[derive(Debug)]
pub struct ListDeviceRecordsBuilder {
    config: Arc<crate::models::SecurityConfig>,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id: Option<String>,
    device_type: Option<String>,
    status: Option<crate::models::security_and_compliance::DeviceRecordStatus>,
    personal_device: Option<bool>,
    compliance_status: Option<crate::models::security_and_compliance::ComplianceStatus>,
}

impl ListDeviceRecordsBuilder {
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

    /// 设置用户ID过滤
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 设置设备类型过滤
    pub fn device_type(mut self, device_type: impl Into<String>) -> Self {
        self.device_type = Some(device_type.into());
        self
    }

    /// 设置状态过滤
    pub fn status(
        mut self,
        status: crate::models::security_and_compliance::DeviceRecordStatus,
    ) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置是否为个人设备过滤
    pub fn personal_device(mut self, personal_device: bool) -> Self {
        self.personal_device = Some(personal_device);
        self
    }

    /// 设置合规状态过滤
    pub fn compliance_status(
        mut self,
        compliance_status: crate::models::security_and_compliance::ComplianceStatus,
    ) -> Self {
        self.compliance_status = Some(compliance_status);
        self
    }

    /// 发送请求查询设备信息
    pub async fn send(
        self,
    ) -> crate::SecurityResult<
        crate::models::PageResponse<crate::models::security_and_compliance::DeviceRecord>,
    > {
        let url = format!(
            "{}/open-apis/security_and_compliance/v2/device_records",
            self.config.base_url
        );

        let mut query_params = Vec::new();

        if let Some(page_size) = self.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &self.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(user_id) = &self.user_id {
            query_params.push(format!("user_id={}", user_id));
        }
        if let Some(device_type) = &self.device_type {
            query_params.push(format!("device_type={}", device_type));
        }
        if let Some(status) = &self.status {
            let status_str = match status {
                crate::models::security_and_compliance::DeviceRecordStatus::Pending => "pending",
                crate::models::security_and_compliance::DeviceRecordStatus::Approved => "approved",
                crate::models::security_and_compliance::DeviceRecordStatus::Rejected => "rejected",
                crate::models::security_and_compliance::DeviceRecordStatus::Expired => "expired",
                crate::models::security_and_compliance::DeviceRecordStatus::Revoked => "revoked",
                crate::models::security_and_compliance::DeviceRecordStatus::NonCompliant => {
                    "non_compliant"
                }
            };
            query_params.push(format!("status={}", status_str));
        }
        if let Some(personal_device) = self.personal_device {
            query_params.push(format!(
                "personal_device={}",
                if personal_device { "true" } else { "false" }
            ));
        }
        if let Some(compliance_status) = &self.compliance_status {
            let status_str = match compliance_status {
                crate::models::security_and_compliance::ComplianceStatus::Compliant => "compliant",
                crate::models::security_and_compliance::ComplianceStatus::NonCompliant => {
                    "non_compliant"
                }
                crate::models::security_and_compliance::ComplianceStatus::Pending => "pending",
                crate::models::security_and_compliance::ComplianceStatus::Unknown => "unknown",
            };
            query_params.push(format!("compliance_status={}", status_str));
        }

        let url_with_params = if !query_params.is_empty() {
            format!("{}?{}", url, query_params.join("&"))
        } else {
            url
        };

        let response = reqwest::Client::new()
            .get(&url_with_params)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<
                crate::models::PageResponse<crate::models::security_and_compliance::DeviceRecord>,
            > = response.json().await?;
            match api_response.data {
                Some(records) => Ok(records),
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

/// 获取设备信息构建器
#[derive(Debug)]
pub struct GetDeviceRecordBuilder {
    config: Arc<crate::models::SecurityConfig>,
    device_record_id: String,
}

impl GetDeviceRecordBuilder {
    /// 设置设备记录ID
    pub fn device_record_id(mut self, device_record_id: impl Into<String>) -> Self {
        self.device_record_id = device_record_id.into();
        self
    }

    /// 发送请求获取设备信息
    pub async fn send(
        self,
    ) -> crate::SecurityResult<crate::models::security_and_compliance::DeviceRecord> {
        let url = format!(
            "{}/open-apis/security_and_compliance/v2/device_records/{}",
            self.config.base_url, self.device_record_id
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
            let api_response: crate::models::ApiResponse<
                crate::models::security_and_compliance::DeviceRecord,
            > = response.json().await?;
            match api_response.data {
                Some(record) => Ok(record),
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

/// 更新设备信息构建器
#[derive(Debug)]
pub struct UpdateDeviceRecordBuilder {
    config: Arc<crate::models::SecurityConfig>,
    device_record_id: String,
    device_name: Option<String>,
    device_brand: Option<String>,
    device_model: Option<String>,
    os_type: Option<String>,
    os_version: Option<String>,
    serial_number: Option<String>,
    mac_address: Option<String>,
    compliance_status: Option<crate::models::security_and_compliance::ComplianceStatus>,
    extension: Option<crate::models::ExtensionMap>,
}

impl UpdateDeviceRecordBuilder {
    /// 设置设备记录ID
    pub fn device_record_id(mut self, device_record_id: impl Into<String>) -> Self {
        self.device_record_id = device_record_id.into();
        self
    }

    /// 设置设备名称
    pub fn device_name(mut self, device_name: impl Into<String>) -> Self {
        self.device_name = Some(device_name.into());
        self
    }

    /// 设置设备品牌
    pub fn device_brand(mut self, device_brand: impl Into<String>) -> Self {
        self.device_brand = Some(device_brand.into());
        self
    }

    /// 设置设备型号
    pub fn device_model(mut self, device_model: impl Into<String>) -> Self {
        self.device_model = Some(device_model.into());
        self
    }

    /// 设置操作系统
    pub fn os_type(mut self, os_type: impl Into<String>) -> Self {
        self.os_type = Some(os_type.into());
        self
    }

    /// 设置操作系统版本
    pub fn os_version(mut self, os_version: impl Into<String>) -> Self {
        self.os_version = Some(os_version.into());
        self
    }

    /// 设置设备序列号
    pub fn serial_number(mut self, serial_number: impl Into<String>) -> Self {
        self.serial_number = Some(serial_number.into());
        self
    }

    /// 设置MAC地址
    pub fn mac_address(mut self, mac_address: impl Into<String>) -> Self {
        self.mac_address = Some(mac_address.into());
        self
    }

    /// 设置合规状态
    pub fn compliance_status(
        mut self,
        compliance_status: crate::models::security_and_compliance::ComplianceStatus,
    ) -> Self {
        self.compliance_status = Some(compliance_status);
        self
    }

    /// 设置扩展信息
    pub fn extension(mut self, extension: crate::models::ExtensionMap) -> Self {
        self.extension = Some(extension);
        self
    }

    /// 发送请求更新设备信息
    pub async fn send(
        self,
    ) -> crate::SecurityResult<crate::models::security_and_compliance::DeviceRecord> {
        let url = format!(
            "{}/open-apis/security_and_compliance/v2/device_records/{}",
            self.config.base_url, self.device_record_id
        );

        let request_body = crate::models::security_and_compliance::DeviceRecordUpdateRequest {
            device_name: self.device_name,
            device_brand: self.device_brand,
            device_model: self.device_model,
            os_type: self.os_type,
            os_version: self.os_version,
            serial_number: self.serial_number,
            mac_address: self.mac_address,
            compliance_status: self.compliance_status,
            extension: self.extension,
        };

        let response = reqwest::Client::new()
            .put(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<
                crate::models::security_and_compliance::DeviceRecord,
            > = response.json().await?;
            match api_response.data {
                Some(record) => Ok(record),
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

/// 删除设备构建器
#[derive(Debug)]
pub struct DeleteDeviceRecordBuilder {
    config: Arc<crate::models::SecurityConfig>,
    device_record_id: String,
}

impl DeleteDeviceRecordBuilder {
    /// 设置设备记录ID
    pub fn device_record_id(mut self, device_record_id: impl Into<String>) -> Self {
        self.device_record_id = device_record_id.into();
        self
    }

    /// 发送请求删除设备
    pub async fn send(self) -> crate::SecurityResult<crate::models::OperationResponse> {
        let url = format!(
            "{}/open-apis/security_and_compliance/v2/device_records/{}",
            self.config.base_url, self.device_record_id
        );

        let response = reqwest::Client::new()
            .delete(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::OperationResponse> =
                response.json().await?;
            match api_response.data {
                Some(result) => Ok(result),
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
