//! 访问记录 API
//!
//! 提供门禁访问记录的查询和人脸识别照片下载功能。

use std::sync::Arc;

/// 访问记录服务
#[derive(Debug)]
pub struct AccessRecordsService {
    config: Arc<crate::models::SecurityConfig>,
}

impl AccessRecordsService {
    /// 创建新的访问记录服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 获取访问记录列表
    pub fn list(&self) -> ListAccessRecordsBuilder {
        ListAccessRecordsBuilder {
            config: self.config.clone(),
            user_id_filter: None,
            device_id_filter: None,
            start_time: None,
            end_time: None,
            access_result_filter: None,
            page_size: Some(20),
            page_token: None,
            sort_field: Some("access_time".to_string()),
            sort_direction: Some("desc".to_string()),
        }
    }

    /// 获取访问记录的人脸识别照片
    pub fn get_access_photo(&self) -> GetAccessPhotoBuilder {
        GetAccessPhotoBuilder {
            config: self.config.clone(),
            access_record_id: String::new(),
        }
    }
}

/// 获取访问记录列表构建器
#[derive(Debug)]
pub struct ListAccessRecordsBuilder {
    config: Arc<crate::models::SecurityConfig>,
    user_id_filter: Option<String>,
    device_id_filter: Option<String>,
    start_time: Option<crate::models::Timestamp>,
    end_time: Option<crate::models::Timestamp>,
    access_result_filter: Option<crate::models::acs::AccessResult>,
    page_size: Option<i32>,
    page_token: Option<String>,
    sort_field: Option<String>,
    sort_direction: Option<String>,
}

impl ListAccessRecordsBuilder {
    /// 设置用户ID过滤
    pub fn user_id_filter(mut self, user_id: impl Into<String>) -> Self {
        self.user_id_filter = Some(user_id.into());
        self
    }

    /// 设置设备ID过滤
    pub fn device_id_filter(mut self, device_id: impl Into<String>) -> Self {
        self.device_id_filter = Some(device_id.into());
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: crate::models::Timestamp) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: crate::models::Timestamp) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// 设置时间范围
    pub fn time_range(
        mut self,
        start_time: crate::models::Timestamp,
        end_time: crate::models::Timestamp,
    ) -> Self {
        self.start_time = Some(start_time);
        self.end_time = Some(end_time);
        self
    }

    /// 设置最近N天的记录
    pub fn last_days(mut self, days: i64) -> Self {
        use chrono::Utc;
        let now = Utc::now().timestamp();
        let start_time = now - days * 24 * 3600;
        self.start_time = Some(start_time);
        self.end_time = Some(now);
        self
    }

    /// 设置最近N小时的记录
    pub fn last_hours(mut self, hours: i64) -> Self {
        use chrono::Utc;
        let now = Utc::now().timestamp();
        let start_time = now - hours * 3600;
        self.start_time = Some(start_time);
        self.end_time = Some(now);
        self
    }

    /// 设置访问结果过滤
    pub fn access_result_filter(mut self, access_result: crate::models::acs::AccessResult) -> Self {
        self.access_result_filter = Some(access_result);
        self
    }

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

    /// 设置排序字段
    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.sort_field = Some(sort_field.into());
        self
    }

    /// 设置排序方向
    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.sort_direction = Some(sort_direction.into());
        self
    }

    /// 设置按时间升序排列
    pub fn sort_by_time_asc(mut self) -> Self {
        self.sort_field = Some("access_time".to_string());
        self.sort_direction = Some("asc".to_string());
        self
    }

    /// 设置按时间降序排列
    pub fn sort_by_time_desc(mut self) -> Self {
        self.sort_field = Some("access_time".to_string());
        self.sort_direction = Some("desc".to_string());
        self
    }

    /// 发送请求获取访问记录列表
    pub async fn send(
        self,
    ) -> crate::SecurityResult<crate::models::PageResponse<crate::models::acs::AccessRecord>> {
        let url = format!("{}/open-apis/acs/v1/access_records", self.config.base_url);

        let mut query_params = Vec::new();

        if let Some(user_id_filter) = &self.user_id_filter {
            query_params.push(format!("user_id_filter={}", user_id_filter));
        }
        if let Some(device_id_filter) = &self.device_id_filter {
            query_params.push(format!("device_id_filter={}", device_id_filter));
        }
        if let Some(start_time) = self.start_time {
            query_params.push(format!("start_time={}", start_time));
        }
        if let Some(end_time) = self.end_time {
            query_params.push(format!("end_time={}", end_time));
        }
        if let Some(access_result_filter) = &self.access_result_filter {
            let result_str = match access_result_filter {
                crate::models::acs::AccessResult::Success => "success",
                crate::models::acs::AccessResult::Failed => "failed",
                crate::models::acs::AccessResult::Timeout => "timeout",
            };
            query_params.push(format!("access_result_filter={}", result_str));
        }
        if let Some(page_size) = self.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &self.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(sort_field) = &self.sort_field {
            query_params.push(format!("sort_field={}", sort_field));
        }
        if let Some(sort_direction) = &self.sort_direction {
            query_params.push(format!("sort_direction={}", sort_direction));
        }

        if !query_params.is_empty() {
            let url_with_params = format!("{}?{}", url, query_params.join("&"));
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
                    crate::models::PageResponse<crate::models::acs::AccessRecord>,
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
        } else {
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
                    crate::models::PageResponse<crate::models::acs::AccessRecord>,
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
}

/// 获取访问记录人脸识别照片构建器
#[derive(Debug)]
pub struct GetAccessPhotoBuilder {
    config: Arc<crate::models::SecurityConfig>,
    access_record_id: String,
}

impl GetAccessPhotoBuilder {
    /// 设置访问记录ID
    pub fn access_record_id(mut self, access_record_id: impl Into<String>) -> Self {
        self.access_record_id = access_record_id.into();
        self
    }

    /// 发送请求获取访问记录的人脸识别照片
    pub async fn send(self) -> crate::SecurityResult<Vec<u8>> {
        let url = format!(
            "{}/open-apis/acs/v1/access_records/{}/access_photo",
            self.config.base_url, self.access_record_id
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .send()
            .await?;

        if response.status().is_success() {
            let photo_data = response.bytes().await?;
            Ok(photo_data.to_vec())
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
