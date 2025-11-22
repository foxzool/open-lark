//! Bitable App Table Record API 服务实现
//!
//! 提供多维表格数据记录管理相关的API服务，包括：
//! - 记录的创建、查询、更新、删除
//! - 批量记录操作支持
//! - 记录搜索和过滤功能
//! - 完整的错误处理和参数验证
use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 多维表格数据记录管理服务
#[derive(Debug, Clone)]
pub struct AppTableRecordService {
    config: Config,
}

impl AppTableRecordService {
    /// 创建新的数据记录管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 新增记录
    ///
    /// 在指定的数据表中新增一条记录
    ///
    /// # 参数
    /// * `request` - 新增记录请求
    ///
    /// # 返回
    /// 返回新创建的记录信息
    pub async fn create_record(
        &self,
        request: &CreateRecordRequest,
    ) -> SDKResult<CreateRecordResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "新增记录: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("fields", serde_json::to_value(&request.fields)?);

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/records",
                request.app_token, request.table_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<CreateRecordResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "新增记录完成: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        Ok(response)
    }

    /// 批量新增记录
    ///
    /// 在指定的数据表中批量新增多条记录
    ///
    /// # 参数
    /// * `request` - 批量新增记录请求
    ///
    /// # 返回
    /// 返回新创建的记录信息列表
    pub async fn batch_create_record(
        &self,
        request: &BatchCreateRecordRequest,
    ) -> SDKResult<BatchCreateRecordResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "批量新增记录: app_token={}, table_id={}, count={}",
            request.app_token,
            request.table_id,
            request.records.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("records", serde_json::to_value(&request.records)?);

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_create",
                request.app_token, request.table_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<BatchCreateRecordResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "批量新增记录完成: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        Ok(response)
    }

    /// 更新记录
    ///
    /// 更新指定数据表中的记录
    ///
    /// # 参数
    /// * `request` - 更新记录请求
    ///
    /// # 返回
    /// 返回更新后的记录信息
    pub async fn update_record(
        &self,
        request: &UpdateRecordRequest,
    ) -> SDKResult<UpdateRecordResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新记录: app_token={}, table_id={}, record_id={}",
            request.app_token,
            request.table_id,
            request.record_id
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("fields", serde_json::to_value(&request.fields)?);

        // 构建API请求
        let api_req = ApiRequest::put(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
                request.app_token, request.table_id, request.record_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<UpdateRecordResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新记录完成: app_token={}, table_id={}, record_id={}",
            request.app_token,
            request.table_id,
            request.record_id
        );

        Ok(response)
    }

    /// 在指定的数据表中批量更新多条记录
    ///
    /// # 参数
    /// * `request` - 更新请求
    pub async fn batch_update_record(
        &self,
        request: &BatchUpdateRecordRequest,
    ) -> SDKResult<BatchUpdateRecordResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "// 构建API请求
        let api_req = ApiRequest::put(format!(: app_token={}, table_id={}, count={}",
            request.app_token,
            request.table_id,
            request.records.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("records", serde_json::to_value(&request.records)?);

        // 构建API请求
        let api_req = ApiRequest::get(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update",
                request.app_token, request.table_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<BatchUpdateRecordResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "// 构建API请求
        let api_req = ApiRequest::put(format!(完成: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        Ok(response)
    }

    /// 删除记录
    ///
    /// 删除指定数据表中的记录
    ///
    /// # 参数
    /// * `request` - 删除记录请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    pub async fn delete_record(
        &self,
        request: &DeleteRecordRequest,
    ) -> SDKResult<DeleteRecordResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "删除记录: app_token={}, table_id={}, record_id={}",
            request.app_token,
            request.table_id,
            request.record_id
        );

        // 构建API请求
        let api_req = ApiRequest::delete(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
                request.app_token, request.table_id, request.record_id
            ))
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<DeleteRecordResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "删除记录完成: app_token={}, table_id={}, record_id={}",
            request.app_token,
            request.table_id,
            request.record_id
        );

        Ok(response)
    }

    /// 批量删除记录
    ///
    /// 在指定的数据表中批量删除多条记录
    ///
    /// # 参数
    /// * `request` - 批量删除记录请求
    ///
    /// # 返回
    /// 返回批量删除的结果
    pub async fn batch_delete_record(
        &self,
        request: &BatchDeleteRecordRequest,
    ) -> SDKResult<BatchDeleteRecordResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "批量删除记录: app_token={}, table_id={}, count={}",
            request.app_token,
            request.table_id,
            request.record_ids.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("record_ids", serde_json::to_value(&request.record_ids)?);

        // 构建API请求
        let api_req = ApiRequest::get(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_delete",
                request.app_token, request.table_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<BatchDeleteRecordResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "批量删除记录完成: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        Ok(response)
    }

    /// 获取记录
    ///
    /// 获取指定数据表中的单条记录
    ///
    /// # 参数
    /// * `request` - 获取记录请求
    ///
    /// # 返回
    /// 返回记录信息
    pub async fn get_record(&self, request: &GetRecordRequest) -> SDKResult<GetRecordResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "获取记录: app_token={}, table_id={}, record_id={}",
            request.app_token,
            request.table_id,
            request.record_id
        );

        // 构建查询参数
        let mut query_params = HashMap::new();

        if let Some(ref field_type) = request.field_type {
            query_params.insert("field_type", field_type.clone());
        }
        if let Some(need_field_name) = request.need_field_name {
            query_params.insert("need_field_name", need_field_name.to_string());
        }

        // 构建API请求
        let api_req = ApiRequest::get(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
                request.app_token, request.table_id, request.record_id
            ))
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<GetRecordResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "获取记录完成: app_token={}, table_id={}, record_id={}",
            request.app_token,
            request.table_id,
            request.record_id
        );

        Ok(response)
    }

    /// 列出记录
    ///
    /// 获取指定数据表中的记录列表
    ///
    /// # 参数
    /// * `request` - 列出记录请求
    ///
    /// # 返回
    /// 返回记录列表
    pub async fn list_records(
        &self,
        request: &ListRecordsRequest,
    ) -> SDKResult<ListRecordsResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "列出记录: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        // 构建查询参数
        let mut query_params = HashMap::new();

        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = request.page_token {
            query_params.insert("page_token", page_token.clone());
        }
        if let Some(ref field_type) = request.field_type {
            query_params.insert("field_type", field_type.clone());
        }
        if let Some(need_field_name) = request.need_field_name {
            query_params.insert("need_field_name", need_field_name.to_string());
        }
        if let Some(ref sort) = request.sort {
            query_params.insert("sort", serde_json::to_string(sort)?);
        }
        if let Some(ref filter) = request.filter {
            query_params.insert("filter", serde_json::to_string(filter)?);
        }

        // 构建API请求
        let api_req = ApiRequest::get(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/records",
                request.app_token, request.table_id
            ))
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<ListRecordsResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "列出记录完成: app_token={}, table_id={}, count={}",
            request.app_token,
            request.table_id,
            response.records.as_ref().map(|r| r.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 批量获取记录
    ///
    /// 根据记录ID列表批量获取记录
    ///
    /// # 参数
    /// * `request` - 批量获取记录请求
    ///
    /// # 返回
    /// 返回记录信息列表
    pub async fn batch_get_record(
        &self,
        request: &BatchGetRecordRequest,
    ) -> SDKResult<BatchGetRecordResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "批量获取记录: app_token={}, table_id={}, count={}",
            request.app_token,
            request.table_id,
            request.record_ids.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("record_ids", serde_json::to_value(&request.record_ids)?);

        if let Some(ref field_type) = request.field_type {
            body.insert("field_type", serde_json::to_value(field_type)?);
        }
        if let Some(need_field_name) = request.need_field_name {
            body.insert("need_field_name", serde_json::to_value(need_field_name)?);
        }

        // 构建API请求
        let api_req = ApiRequest::get(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_get",
                request.app_token, request.table_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<BatchGetRecordResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "批量获取记录完成: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        Ok(response)
    }

    /// 搜索记录
    ///
    /// 在指定的数据表中搜索记录
    ///
    /// # 参数
    /// * `request` - 搜索记录请求
    ///
    /// # 返回
    /// 返回搜索结果
    pub async fn search_records(
        &self,
        request: &SearchRecordsRequest,
    ) -> SDKResult<SearchRecordsResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "搜索记录: app_token={}, table_id={}, search_string={}",
            request.app_token,
            request.table_id,
            request.search_string
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert(
            "search_string",
            serde_json::to_value(&request.search_string)?,
        );

        if let Some(page_size) = request.page_size {
            body.insert("page_size", serde_json::to_value(page_size)?);
        }
        if let Some(ref page_token) = request.page_token {
            body.insert("page_token", serde_json::to_value(page_token)?);
        }
        if let Some(ref field_type) = request.field_type {
            body.insert("field_type", serde_json::to_value(field_type)?);
        }
        if let Some(need_field_name) = request.need_field_name {
            body.insert("need_field_name", serde_json::to_value(need_field_name)?);
        }
        if let Some(ref sort) = request.sort {
            body.insert("sort", serde_json::to_value(sort)?);
        }

        // 构建API请求
        let api_req = ApiRequest::get(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/records/search",
                request.app_token, request.table_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<SearchRecordsResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "搜索记录完成: app_token={}, table_id={}, count={}",
            request.app_token,
            request.table_id,
            response.records.as_ref().map(|r| r.len()).unwrap_or(0)
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct CreateRecordRequestBuilder {
    request: CreateRecordRequest,
}

impl CreateRecordRequestBuilder {
    pub fn new(app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            request: CreateRecordRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                fields: Value::Object(serde_json::Map::new()),
            },
        }
    }

    pub fn fields(mut self, fields: Value) -> Self {
        self.request.fields = fields;
        self
    }

    pub async fn execute(self, service: &AppTableRecordService) -> SDKResult<CreateRecordResponse> {
        service.create_record(&self.request).await
    }
}

pub struct UpdateRecordRequestBuilder {
    request: UpdateRecordRequest,
}

impl UpdateRecordRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        record_id: impl Into<String>,
    ) -> Self {
        Self {
            request: UpdateRecordRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                record_id: record_id.into(),
                fields: Value::Object(serde_json::Map::new()),
            },
        }
    }

    pub fn fields(mut self, fields: Value) -> Self {
        self.request.fields = fields;
        self
    }

    pub async fn execute(self, service: &AppTableRecordService) -> SDKResult<UpdateRecordResponse> {
        service.update_record(&self.request).await
    }
}
