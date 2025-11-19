//! Drive Explorer API 数据模型
//!
//! 提供资源浏览器相关的数据结构，支持文件夹和文件的基本操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 获取我的空间（根文件夹）元数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetRootFolderMetaResponse {
    /// 文件夹信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<FolderMeta>,
}

impl ApiResponseTrait for GetRootFolderMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件夹元数据请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetFolderMetaRequest {
    /// 文件夹token
    pub folder_token: String,
}

impl GetFolderMetaRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.folder_token.trim().is_empty() {
            return Err("文件夹token不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取文件夹元数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetFolderMetaResponse {
    /// 文件夹信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<FolderMeta>,
}

impl ApiResponseTrait for GetFolderMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文件夹元数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FolderMeta {
    /// 文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 文件夹名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 文件夹类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_type: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 父文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    /// 是否被删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

/// 新建文件请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFileRequest {
    /// 父文件夹token
    pub folder_token: String,
    /// 文件类型：doc/sheet/bitable/file
    pub file_type: String,
    /// 文件名称
    pub file_name: String,
    /// 文件内容（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl CreateFileRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.folder_token.trim().is_empty() {
            return Err("父文件夹token不能为空".to_string());
        }
        if !["doc", "sheet", "bitable", "file"].contains(&self.file_type.as_str()) {
            return Err("文件类型必须是doc、sheet、bitable或file".to_string());
        }
        if self.file_name.trim().is_empty() {
            return Err("文件名不能为空".to_string());
        }
        Ok(())
    }
}

/// 新建文件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateFileResponse {
    /// 文件信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileMeta>,
}

impl ApiResponseTrait for CreateFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新建文件夹请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFolderRequest {
    /// 父文件夹token
    pub folder_token: String,
    /// 文件夹名称
    pub folder_name: String,
}

impl CreateFolderRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.folder_token.trim().is_empty() {
            return Err("父文件夹token不能为空".to_string());
        }
        if self.folder_name.trim().is_empty() {
            return Err("文件夹名称不能为空".to_string());
        }
        Ok(())
    }
}

/// 新建文件夹响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateFolderResponse {
    /// 文件夹信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<FolderMeta>,
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件夹下的文档清单请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetFolderChildrenRequest {
    /// 文件夹token
    pub folder_token: String,
    /// 页面大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 文件类型过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 排序字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// 排序方向
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

impl GetFolderChildrenRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.folder_token.trim().is_empty() {
            return Err("文件夹token不能为空".to_string());
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 || page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }
        Ok(())
    }
}

/// 获取文件夹下的文档清单响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetFolderChildrenResponse {
    /// 文件/文件夹列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<FileOrFolderItem>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for GetFolderChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文件或文件夹项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileOrFolderItem {
    /// 文件token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 类型：folder/doc/sheet/bitable/file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// 文件元数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileMeta {
    /// 文件token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    /// 文件名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// 复制文档请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyFileRequest {
    /// 源文件token
    pub file_token: String,
    /// 目标文件夹token
    pub dest_folder_token: String,
    /// 新文件名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 复制类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CopyFileRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("源文件token不能为空".to_string());
        }
        if self.dest_folder_token.trim().is_empty() {
            return Err("目标文件夹token不能为空".to_string());
        }
        Ok(())
    }
}

/// 复制文档响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CopyFileResponse {
    /// 新文件信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileMeta>,
}

impl ApiResponseTrait for CopyFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除文档请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteFileRequest {
    /// 文件token
    pub file_token: String,
    /// 文件类型
    pub file_type: String,
}

impl DeleteFileRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("文件token不能为空".to_string());
        }
        if !["doc", "sheet"].contains(&self.file_type.as_str()) {
            return Err("文件类型必须是doc或sheet".to_string());
        }
        Ok(())
    }
}

/// 删除文档响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteFileResponse {
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
