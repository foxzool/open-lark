/// CCM Drive Explorer 请求类型定义
use serde::{Deserialize, Serialize};

/// 获取根目录元数据请求
#[derive(Debug, Clone)]
pub struct RootFolderMetaRequest {
    /// 文件夹类型
    pub folder_type: Option<String>,
}

impl RootFolderMetaRequest {
    /// 创建新的获取根目录元数据请求
    pub fn new() -> Self {
        Self {
            folder_type: None,
        }
    }

    /// 设置文件夹类型
    pub fn folder_type(mut self, folder_type: &str) -> Self {
        self.folder_type = Some(folder_type.to_string());
        self
    }
}

/// 获取文件夹元数据请求
#[derive(Debug, Clone)]
pub struct FolderMetaRequest {
    /// 文件夹token
    pub folder_token: String,
}

impl FolderMetaRequest {
    /// 创建新的获取文件夹元数据请求
    pub fn new(folder_token: &str) -> Self {
        Self {
            folder_token: folder_token.to_string(),
        }
    }
}

/// 获取文件元数据请求
#[derive(Debug, Clone)]
pub struct FileRequest {
    /// 文件token
    pub file_token: String,
}

impl FileRequest {
    /// 创建新的获取文件元数据请求
    pub fn new(file_token: &str) -> Self {
        Self {
            file_token: file_token.to_string(),
        }
    }
}

/// 复制文件请求
#[derive(Debug, Clone)]
pub struct FileCopyRequest {
    /// 源文件token
    pub file_token: String,
    /// 目标文件夹token
    pub folder_token: String,
    /// 新文件名
    pub name: Option<String>,
}

impl FileCopyRequest {
    /// 创建新的复制文件请求
    pub fn new(file_token: &str, folder_token: &str) -> Self {
        Self {
            file_token: file_token.to_string(),
            folder_token: folder_token.to_string(),
            name: None,
        }
    }

    /// 设置新文件名
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

/// 获取文档文件信息请求
#[derive(Debug, Clone)]
pub struct FileDocsRequest {
    /// 文件token
    pub file_token: String,
}

impl FileDocsRequest {
    /// 创建新的获取文档文件信息请求
    pub fn new(file_token: &str) -> Self {
        Self {
            file_token: file_token.to_string(),
        }
    }
}

/// 获取表格文件信息请求
#[derive(Debug, Clone)]
pub struct FileSpreadsheetsRequest {
    /// 文件token
    pub file_token: String,
}

impl FileSpreadsheetsRequest {
    /// 创建新的获取表格文件信息请求
    pub fn new(file_token: &str) -> Self {
        Self {
            file_token: file_token.to_string(),
        }
    }
}

/// 获取文件夹子内容请求
#[derive(Debug, Clone)]
pub struct FolderChildrenRequest {
    /// 文件夹token
    pub folder_token: String,
    /// 每页数量
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
    /// 排序字段
    pub order_by: Option<String>,
    /// 排序方向
    pub direction: Option<String>,
}

impl FolderChildrenRequest {
    /// 创建新的获取文件夹子内容请求
    pub fn new(folder_token: &str) -> Self {
        Self {
            folder_token: folder_token.to_string(),
            page_size: None,
            page_token: None,
            order_by: None,
            direction: None,
        }
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置页面标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_string());
        self
    }

    /// 设置排序字段
    pub fn order_by(mut self, order_by: &str) -> Self {
        self.order_by = Some(order_by.to_string());
        self
    }

    /// 设置排序方向
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = Some(direction.to_string());
        self
    }
}

/// 创建文件夹请求
#[derive(Debug, Clone)]
pub struct FolderRequest {
    /// 父文件夹token
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
}

impl FolderRequest {
    /// 创建新的创建文件夹请求
    pub fn new(folder_token: &str, name: &str) -> Self {
        Self {
            folder_token: folder_token.to_string(),
            name: name.to_string(),
        }
    }
}

// 权限相关请求

/// 成员权限请求
#[derive(Debug, Clone)]
pub struct MemberPermittedRequest {
    /// 文件/文件夹token
    pub obj_token: String,
    /// 类型
    pub obj_type: Option<String>,
}

impl MemberPermittedRequest {
    /// 创建新的成员权限请求
    pub fn new(obj_token: &str) -> Self {
        Self {
            obj_token: obj_token.to_string(),
            obj_type: None,
        }
    }

    /// 设置类型
    pub fn obj_type(mut self, obj_type: &str) -> Self {
        self.obj_type = Some(obj_type.to_string());
        self
    }
}

/// 成员转移请求
#[derive(Debug, Clone)]
pub struct MemberTransferRequest {
    /// 文件/文件夹token
    pub obj_token: String,
    /// 目标用户ID
    pub target_id: String,
    /// 目标类型
    pub target_type: Option<String>,
}

impl MemberTransferRequest {
    /// 创建新的成员转移请求
    pub fn new(obj_token: &str, target_id: &str) -> Self {
        Self {
            obj_token: obj_token.to_string(),
            target_id: target_id.to_string(),
            target_type: None,
        }
    }

    /// 设置目标类型
    pub fn target_type(mut self, target_type: &str) -> Self {
        self.target_type = Some(target_type.to_string());
        self
    }
}

/// 公开链接请求
#[derive(Debug, Clone)]
pub struct PublicRequest {
    /// 文件/文件夹token
    pub obj_token: String,
    /// 链接类型
    pub link_type: Option<String>,
}

impl PublicRequest {
    /// 创建新的公开链接请求
    pub fn new(obj_token: &str) -> Self {
        Self {
            obj_token: obj_token.to_string(),
            link_type: None,
        }
    }

    /// 设置链接类型
    pub fn link_type(mut self, link_type: &str) -> Self {
        self.link_type = Some(link_type.to_string());
        self
    }
}