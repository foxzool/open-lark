/// 创建文件快捷方式
///
/// 创建文件快捷方式，用于访问云空间的文件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create_shortcut

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 创建文件快捷方式请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileShortcutParams {
    /// 快捷方式名称
    pub name: String,
    /// 快捷方式类型
    pub shortcut_type: String,
    /// 目标文件token
    pub file_token: String,
    /// 父文件夹token
    pub parent_folder_token: String,
}

/// 创建文件快捷方式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileShortcutResponse {
    /// 快捷方式信息
    pub data: Option<FileShortcutData>,
}

/// 文件快捷方式数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileShortcutData {
    /// 文件token
    pub file_token: String,
    /// 快捷方式名称
    pub name: String,
    /// 快捷方式类型
    pub shortcut_type: String,
    /// 创建时间
    pub create_time: String,
}

impl ApiResponseTrait for CreateFileShortcutResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建文件快捷方式请求
pub struct CreateFileShortcutRequest {
    config: Config,
}

impl CreateFileShortcutRequest {
    /// 创建新的请求实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行创建文件快捷方式操作
    pub async fn execute(self, params: CreateFileShortcutParams) -> SDKResult<CreateFileShortcutResponse> {
        // 验证必填字段
        validate_required_field("快捷方式名称", Some(&params.name), "快捷方式名称不能为空")?;
        validate_required_field("快捷方式类型", Some(&params.shortcut_type), "快捷方式类型不能为空")?;
        validate_required_field("目标文件token", Some(&params.file_token), "目标文件token不能为空")?;
        validate_required_field("父文件夹token", Some(&params.parent_folder_token), "父文件夹token不能为空")?;

        let api_endpoint = DriveApi::CreateShortcut;
        let api_request: ApiRequest<CreateFileShortcutResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建文件快捷方式")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建文件快捷方式")
    }
}

// ==================== 构建器模式 ====================

/// 创建文件快捷方式构建器
pub struct CreateFileShortcutBuilder {
    request: CreateFileShortcutParams,
}

impl CreateFileShortcutBuilder {
    /// 创建新的构建器
    pub fn new(name: impl Into<String>, shortcut_type: impl Into<String>, file_token: impl Into<String>, parent_folder_token: impl Into<String>) -> Self {
        Self {
            request: CreateFileShortcutParams {
                name: name.into(),
                shortcut_type: shortcut_type.into(),
                file_token: file_token.into(),
                parent_folder_token: parent_folder_token.into(),
            },
        }
    }

    /// 执行创建文件快捷方式操作
    pub async fn execute(self, service: &FileService) -> SDKResult<CreateFileShortcutResponse> {
        service.create_file_shortcut(self.request).await
    }
}

/// 文件服务
#[derive(Clone, Debug)]
pub struct FileService {
    config: Config,
}

impl FileService {
    /// 创建文件服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 创建文件快捷方式
    /// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create_shortcut
    pub async fn create_file_shortcut(&self, params: CreateFileShortcutParams) -> SDKResult<CreateFileShortcutResponse> {
        CreateFileShortcutRequest::new(self.config.clone()).execute(params).await
    }

    /// 创建创建文件快捷方式构建器
    pub fn create_file_shortcut_builder(&self, name: impl Into<String>, shortcut_type: impl Into<String>, file_token: impl Into<String>, parent_folder_token: impl Into<String>) -> CreateFileShortcutBuilder {
        CreateFileShortcutBuilder::new(name, shortcut_type, file_token, parent_folder_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = FileService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_create_file_shortcut_builder() {
        let builder = CreateFileShortcutBuilder::new(
            "快捷方式名称",
            "document",
            "file_token_123",
            "folder_token_456"
        );

        assert_eq!(builder.request.name, "快捷方式名称");
        assert_eq!(builder.request.shortcut_type, "document");
        assert_eq!(builder.request.file_token, "file_token_123");
        assert_eq!(builder.request.parent_folder_token, "folder_token_456");
    }

    #[test]
    fn test_create_file_shortcut_params() {
        let params = CreateFileShortcutParams {
            name: "测试快捷方式".to_string(),
            shortcut_type: "spreadsheet".to_string(),
            file_token: "token_123".to_string(),
            parent_folder_token: "folder_456".to_string(),
        };

        assert_eq!(params.name, "测试快捷方式");
        assert_eq!(params.shortcut_type, "spreadsheet");
        assert_eq!(params.file_token, "token_123");
        assert_eq!(params.parent_folder_token, "folder_456");
    }

    #[test]
    fn test_response_trait_implementation() {
        assert_eq!(CreateFileShortcutResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_serialization() {
        let params = CreateFileShortcutParams {
            name: "序列化测试".to_string(),
            shortcut_type: "doc".to_string(),
            file_token: "token_789".to_string(),
            parent_folder_token: "folder_012".to_string(),
        };

        let serialized = serde_json::to_string(&params).unwrap();
        let deserialized: CreateFileShortcutParams = serde_json::from_str(&serialized).unwrap();

        assert_eq!(params.name, deserialized.name);
        assert_eq!(params.shortcut_type, deserialized.shortcut_type);
        assert_eq!(params.file_token, deserialized.file_token);
        assert_eq!(params.parent_folder_token, deserialized.parent_folder_token);
    }

    #[test]
    fn test_file_shortcut_data() {
        let data = FileShortcutData {
            file_token: "shortcut_token_123".to_string(),
            name: "快捷方式测试".to_string(),
            shortcut_type: "bitable".to_string(),
            create_time: "2023-12-01T00:00:00Z".to_string(),
        };

        assert_eq!(data.file_token, "shortcut_token_123");
        assert_eq!(data.name, "快捷方式测试");
        assert_eq!(data.shortcut_type, "bitable");
        assert_eq!(data.create_time, "2023-12-01T00:00:00Z");
    }
}