//! Drive Explorer API 服务实现
//!
//! 提供资源浏览器相关的API服务，包括：
//! - 文件夹和文件的基本操作
//! - 文档复制和删除
//! - 文件夹内容浏览
use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// Drive Explorer API服务
#[derive(Debug, Clone)]
pub struct ExplorerService {
    config: Config,
}

impl ExplorerService {
    /// 创建新的Explorer服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取我的空间（根文件夹）元数据
    ///
    /// 获取"我的空间"的元信息
    ///
    /// # 返回
    /// 返回根文件夹的元数据信息
    ///
    /// # 示例
    /// ```rust
    /// let response = service.get_root_folder_meta().await?;
    /// if let Some(folder) = response.folder {
    ///     println!("根文件夹: {:?}", folder.name,
    /// }
    /// ```
    pub async fn get_root_folder_meta(&self) -> SDKResult<GetRootFolderMetaResponse> {
        log::info!("获取我的空间（根文件夹）元数据",

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Get,
            url: "/open-apis/drive/explorer/v2/root_folder/meta".to_string(),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None,
            query: HashMap::new(),
            
        };

        // 发送请求
        let resp =
            Transport::<GetRootFolderMetaResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default(,

        log::info!("获取我的空间元数据完成",

        Ok(response)
    }

    /// 获取文件夹元数据
    ///
    /// 根据folderToken获取该文件夹的元信息
    ///
    /// # 参数
    /// * `request` - 获取文件夹元数据请求
    ///
    /// # 返回
    /// 返回文件夹的元数据信息
    pub async fn get_folder_meta(
        &self,
        request: &GetFolderMetaRequest,
    ) -> SDKResult<GetFolderMetaResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("获取文件夹元数据: folder_token={}", request.folder_token,

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Get,
            url: format!(
                "/open-apis/drive/explorer/v2/folder/{}/meta",
                request.folder_token
            ),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None,
            query: HashMap::new(),
            
        };

        // 发送请求
        let resp = Transport::<GetFolderMetaResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default(,

        log::info!(
            "获取文件夹元数据完成: folder_token={}",
            request.folder_token
        ,

        Ok(response)
    }

    /// 新建文件
    ///
    /// 根据folderToken创建Doc、Sheet或Bitable
    ///
    /// # 参数
    /// * `request` - 新建文件请求
    ///
    /// # 返回
    /// 返回新创建的文件信息
    pub async fn create_file(&self, request: &CreateFileRequest) -> SDKResult<CreateFileResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "新建文件: folder_token={}, file_type={}, file_name={}",
            request.folder_token,
            request.file_type,
            request.file_name
        ,

        // 构建请求体
        let mut body = HashMap::new(,
        body.insert("file_type", Value::String(request.file_type.clone()),
        body.insert("file_name", Value::String(request.file_name.clone()),

        if let Some(ref content) = request.content {
            body.insert("content", Value::String(content.clone()),
        }

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Post,
            url: format!("/open-apis/drive/explorer/v2/file/{}", request.folder_token),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(&body)))?,
            query: HashMap::new(),
            
        };

        // 发送请求
        let resp = Transport::<CreateFileResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default(,

        log::info!(
            "新建文件完成: folder_token={}, file_name={}",
            request.folder_token,
            request.file_name
        ,

        Ok(response)
    }

    /// 新建文件夹
    ///
    /// 根据folderToken在该folder下创建文件夹
    ///
    /// # 参数
    /// * `request` - 新建文件夹请求
    ///
    /// # 返回
    /// 返回新创建的文件夹信息
    pub async fn create_folder(
        &self,
        request: &CreateFolderRequest,
    ) -> SDKResult<CreateFolderResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "新建文件夹: folder_token={}, folder_name={}",
            request.folder_token,
            request.folder_name
        ,

        // 构建请求体
        let mut body = HashMap::new(,
        body.insert("folder_name", Value::String(request.folder_name.clone()),

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Post,
            url: format!(
                "/open-apis/drive/explorer/v2/folder/{}",
                request.folder_token
            ),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(&body)))?,
            query: HashMap::new(),
            
        };

        // 发送请求
        let resp = Transport::<CreateFolderResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default(,

        log::info!(
            "新建文件夹完成: folder_token={}, folder_name={}",
            request.folder_token,
            request.folder_name
        ,

        Ok(response)
    }

    /// 获取文件夹下的文档清单
    ///
    /// 根据folderToken获取该文件夹的文档清单，如doc、sheet、file、bitable、folder
    ///
    /// # 参数
    /// * `request` - 获取文件夹文档清单请求
    ///
    /// # 返回
    /// 返回文件夹下的文档列表
    pub async fn get_folder_children(
        &self,
        request: &GetFolderChildrenRequest,
    ) -> SDKResult<GetFolderChildrenResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "获取文件夹下的文档清单: folder_token={}",
            request.folder_token
        ,

        // 构建查询参数
        // query_params 将通过链式调用处理

        if let Some(page_size) = request.page_size {
            .query(insert("page_size", page_size.to_string(),
        }
        if let Some(ref page_token) = request.page_token {
            .query(insert("page_token", page_token.clone()
        }
        if let Some(ref file_type) = request.file_type {
            .query(insert("file_type", file_type.clone()
        }
        if let Some(ref order_by) = request.order_by {
            .query(insert("order_by", order_by.clone()
        }
        if let Some(ref direction) = request.direction {
            .query(insert("direction", direction.clone()
        }

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Get,
            url: format!(
                "/open-apis/drive/explorer/v2/folder/{}/children",
                request.folder_token
            ),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None,
            query_params,
            
        };

        // 发送请求
        let resp =
            Transport::<GetFolderChildrenResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default(,

        log::info!(
            "获取文件夹下的文档清单完成: folder_token={}, item_count={}",
            request.folder_token,
            response.items.as_ref().map(|i| i.len()).unwrap_or(0)
        ,

        Ok(response)
    }

    /// 复制文档
    ///
    /// 根据文件token复制Doc或Sheet到目标文件夹中
    ///
    /// # 参数
    /// * `request` - 复制文档请求
    ///
    /// # 返回
    /// 返回新复制的文档信息
    pub async fn copy_file(&self, request: &CopyFileRequest) -> SDKResult<CopyFileResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "复制文档: file_token={}, dest_folder_token={}",
            request.file_token,
            request.dest_folder_token
        ,

        // 构建请求体
        let mut body = HashMap::new(,
        body.insert(
            "dest_folder_token",
            Value::String(request.dest_folder_token.clone()),
        ,

        if let Some(ref name) = request.name {
            body.insert("name", Value::String(name.clone()),
        }
        if let Some(ref r#type) = request.r#type {
            body.insert("type", Value::String(r#type.clone()),
        }

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Post,
            url: format!(
                "/open-apis/drive/explorer/v2/file/copy/files/{}",
                request.file_token
            ),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(&body)))?,
            query: HashMap::new(),
            
        };

        // 发送请求
        let resp = Transport::<CopyFileResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default(,

        log::info!("复制文档完成: file_token={}", request.file_token,

        Ok(response)
    }

    /// 删除Sheet
    ///
    /// 根据spreadsheetToken删除对应的sheet文档
    ///
    /// # 参数
    /// * `file_token` - Sheet文档token
    ///
    /// # 返回
    /// 返回删除操作的结果
    pub async fn delete_sheet(&self, file_token: &str) -> SDKResult<DeleteFileResponse> {
        if file_token.trim().is_empty() {
            return Err(LarkAPIError::illegal_param("文件token不能为空".to_string()),
        }

        log::info!("删除Sheet: file_token={}", file_token,

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Delete,
            url: format!(
                "/open-apis/drive/explorer/v2/file/spreadsheets/{}",
                file_token
            ),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None,
            query: HashMap::new(),
            
        };

        // 发送请求
        let resp = Transport::<DeleteFileResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default(,

        log::info!("删除Sheet完成: file_token={}", file_token,

        Ok(response)
    }

    /// 删除Doc
    ///
    /// 根据docToken删除对应的Docs文档
    ///
    /// # 参数
    /// * `file_token` - Doc文档token
    ///
    /// # 返回
    /// 返回删除操作的结果
    pub async fn delete_doc(&self, file_token: &str) -> SDKResult<DeleteFileResponse> {
        if file_token.trim().is_empty() {
            return Err(LarkAPIError::illegal_param("文件token不能为空".to_string()),
        }

        log::info!("删除Doc: file_token={}", file_token,

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Delete,
            url: format!("/open-apis/drive/explorer/v2/file/docs/{}", file_token),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None,
            query: HashMap::new(),
            
        };

        // 发送请求
        let resp = Transport::<DeleteFileResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default(,

        log::info!("删除Doc完成: file_token={}", file_token,

        Ok(response)
    }
}

// 构建器模式实现
pub struct GetFolderMetaRequestBuilder {
    request: GetFolderMetaRequest,
}

impl GetFolderMetaRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: GetFolderMetaRequest {
                folder_token: String::new(),
            },
        }
    }

    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = folder_token.into(,
        self
    }

    pub async fn execute(self, service: &ExplorerService) -> SDKResult<GetFolderMetaResponse> {
        service.get_folder_meta(&self.request).await
    }
}

pub struct CreateFileRequestBuilder {
    request: CreateFileRequest,
}

impl CreateFileRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: CreateFileRequest {
                folder_token: String::new(),
                file_type: String::new(),
                file_name: String::new(),
                content: None,
            },
        }
    }

    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = folder_token.into(,
        self
    }

    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.request.file_type = file_type.into(,
        self
    }

    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.request.file_name = file_name.into(,
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.request.content = Some(content.into(),
        self
    }

    pub async fn execute(self, service: &ExplorerService) -> SDKResult<CreateFileResponse> {
        service.create_file(&self.request).await
    }
}
