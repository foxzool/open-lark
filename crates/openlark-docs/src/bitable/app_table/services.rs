//! Bitable App Table API 服务实现
//!
//! 提供多维表格数据表管理相关的API服务，包括：
//! - 数据表的创建、查询、更新、删除
//! - 批量操作支持
//! - 完整的错误处理和参数验证

use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 多维表格数据表管理服务
#[derive(Debug, Clone)]
pub struct AppTableService {
    config: Config,
}

impl AppTableService {
    /// 创建新的数据表管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 新增一个数据表
    ///
    /// 通过该接口，可以新增一个仅包含索引列的空数据表，也可以指定一部分初始字段
    ///
    /// # 参数
    /// * `request` - 新增数据表请求
    ///
    /// # 返回
    /// 返回新创建的数据表信息
    pub async fn create_table(
        &self,
        request: &CreateTableRequest,
    ) -> SDKResult<CreateTableResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "新增数据表: app_token={}, name={}",
            request.app_token,
            request.name
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("name", Value::String(request.name.clone()));

        if let Some(ref description) = request.description {
            body.insert("description", Value::String(description.clone()));
        }
        if let Some(is_advanced) = request.is_advanced {
            body.insert("is_advanced", Value::Bool(is_advanced));
        }
        if let Some(ref fields) = request.fields {
            body.insert("fields", serde_json::to_value(fields)?);
        }

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/tables", request.app_token))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<CreateTableResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "新增数据表完成: app_token={}, name={}",
            request.app_token,
            request.name
        );

        Ok(response)
    }

    /// 批量新增数据表
    ///
    /// 新增多个数据表
    ///
    /// # 参数
    /// * `request` - 批量新增数据表请求
    ///
    /// # 返回
    /// 返回创建的数据表信息列表
    pub async fn batch_create_table(
        &self,
        request: &BatchCreateTableRequest,
    ) -> SDKResult<BatchCreateTableResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "批量新增数据表: app_token={}, count={}",
            request.app_token,
            request.tables.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("tables", serde_json::to_value(&request.tables)?);

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/tables/batch_create",
                request.app_token
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<BatchCreateTableResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("批量新增数据表完成: app_token={}", request.app_token);

        Ok(response)
    }

    /// 更新数据表
    ///
    /// 该接口用于更新数据表的基本信息，包括数据表的名称等
    ///
    /// # 参数
    /// * `request` - 更新数据表请求
    ///
    /// # 返回
    /// 返回更新后的数据表信息
    pub async fn update_table(
        &self,
        request: &UpdateTableRequest,
    ) -> SDKResult<UpdateTableResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新数据表: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        // 构建请求体
        let mut body = HashMap::new();

        if let Some(ref name) = request.name {
            body.insert("name", Value::String(name.clone()));
        }
        if let Some(ref description) = request.description {
            body.insert("description", Value::String(description.clone()));
        }
        if let Some(is_advanced) = request.is_advanced {
            body.insert("is_advanced", Value::Bool(is_advanced));
        }

        // 构建API请求
        let api_req = ApiRequest::put(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}",
                request.app_token, request.table_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<UpdateTableResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新数据表完成: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        Ok(response)
    }

    /// 列出数据表
    ///
    /// 根据 app_token，获取多维表格下的所有数据表
    ///
    /// # 参数
    /// * `request` - 列出数据表请求
    ///
    /// # 返回
    /// 返回数据表列表
    pub async fn list_tables(&self, request: &ListTablesRequest) -> SDKResult<ListTablesResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("列出数据表: app_token={}", request.app_token);

        // 构建查询参数
        let mut query_params = HashMap::new();

        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = request.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建API请求
        let api_req = ApiRequest::get(format!(
                "/open-apis/bitable/v1/apps/{}/tables", request.app_token))
            .query(query_params);

        // 发送请求
        let resp = Transport::<ListTablesResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "列出数据表完成: app_token={}, count={}",
            request.app_token,
            response.tables.as_ref().map(|t| t.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 删除一个数据表
    ///
    /// 删除一个数据表，最后一张数据表不允许被删除
    ///
    /// # 参数
    /// * `request` - 删除数据表请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    pub async fn delete_table(
        &self,
        request: &DeleteTableRequest,
    ) -> SDKResult<DeleteTableResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "删除数据表: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        // 构建API请求
        let api_req = ApiRequest::delete(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}",
                request.app_token, request.table_id
            ))
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<DeleteTableResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "删除数据表完成: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        Ok(response)
    }

    /// 批量删除数据表
    ///
    /// 删除多个数据表
    ///
    /// # 参数
    /// * `request` - 批量删除数据表请求
    ///
    /// # 返回
    /// 返回批量删除的结果
    pub async fn batch_delete_table(
        &self,
        request: &BatchDeleteTableRequest,
    ) -> SDKResult<BatchDeleteTableResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "批量删除数据表: app_token={}, count={}",
            request.app_token,
            request.table_ids.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("table_ids", serde_json::to_value(&request.table_ids)?);

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/tables/batch_delete",
                request.app_token
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<BatchDeleteTableResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("批量删除数据表完成: app_token={}", request.app_token);

        Ok(response)
    }
}

// 构建器模式实现
pub struct CreateTableRequestBuilder {
    request: CreateTableRequest,
}

impl CreateTableRequestBuilder {
    pub fn new(app_token: impl Into<String>) -> Self {
        Self {
            request: CreateTableRequest {
                app_token: app_token.into(),
                name: String::new(),
                description: None,
                is_advanced: None,
                fields: None,
            },
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn is_advanced(mut self, is_advanced: bool) -> Self {
        self.request.is_advanced = Some(is_advanced);
        self
    }

    pub fn fields(mut self, fields: Vec<FieldInfo>) -> Self {
        self.request.fields = Some(fields);
        self
    }

    pub async fn execute(self, service: &AppTableService) -> SDKResult<CreateTableResponse> {
        service.create_table(&self.request).await
    }
}

pub struct UpdateTableRequestBuilder {
    request: UpdateTableRequest,
}

impl UpdateTableRequestBuilder {
    pub fn new(app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            request: UpdateTableRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                name: None,
                description: None,
                is_advanced: None,
            },
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn is_advanced(mut self, is_advanced: bool) -> Self {
        self.request.is_advanced = Some(is_advanced);
        self
    }

    pub async fn execute(self, service: &AppTableService) -> SDKResult<UpdateTableResponse> {
        service.update_table(&self.request).await
    }
}
