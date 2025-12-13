/// CCM Drive Explorer 响应类型定义
use serde::{Deserialize, Serialize};

/// 根目录元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootFolderMetaData {
    /// 文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
    /// 文件夹类型
    #[serde(rename = "folder_type")]
    pub folder_type: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
}

/// 文件夹元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderMetaData {
    /// 文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
    /// 父文件夹token
    #[serde(rename = "parent_folder_token")]
    pub parent_folder_token: Option<String>,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
    /// 所有者信息
    pub owner: Option<UserInfo>,
}

/// 文件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileData {
    /// 文件token
    #[serde(rename = "file_token")]
    pub file_token: String,
    /// 文件名称
    pub name: String,
    /// 文件类型
    #[serde(rename = "file_type")]
    pub file_type: String,
    /// 父文件夹token
    #[serde(rename = "parent_folder_token")]
    pub parent_folder_token: Option<String>,
    /// 文件大小
    pub size: i64,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
    /// 所有者信息
    pub owner: Option<UserInfo>,
}

/// 文件复制数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCopyData {
    /// 复制后的文件token
    #[serde(rename = "file_token")]
    pub file_token: String,
    /// 复制后的文件名称
    pub name: String,
    /// 文件类型
    #[serde(rename = "file_type")]
    pub file_type: String,
}

/// 文档文件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDocsData {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
}

/// 表格文件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSpreadsheetsData {
    /// 表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 表格标题
    pub title: String,
    /// 工作表数量
    #[serde(rename = "sheet_count")]
    pub sheet_count: i32,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
}

/// 文件夹子内容数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderChildrenData {
    /// 项目列表
    pub items: Vec<FolderItem>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 页面标记
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

/// 文件夹项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderItem {
    /// 项目token
    #[serde(rename = "obj_token")]
    pub obj_token: String,
    /// 项目名称
    pub name: String,
    /// 项目类型
    #[serde(rename = "obj_type")]
    pub obj_type: String,
    /// 文件大小
    pub size: Option<i64>,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
}

/// 文件夹数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderData {
    /// 文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
    /// 文件夹类型
    #[serde(rename = "folder_type")]
    pub folder_type: String,
    /// 父文件夹token
    #[serde(rename = "parent_folder_token")]
    pub parent_folder_token: Option<String>,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
}

// 权限相关响应

/// 成员权限数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPermittedData {
    /// 成员列表
    pub members: Vec<MemberInfo>,
    /// 总数
    pub total: i32,
}

/// 成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberInfo {
    /// 成员ID
    #[serde(rename = "member_id")]
    pub member_id: String,
    /// 成员类型
    #[serde(rename = "member_type")]
    pub member_type: String,
    /// 权限类型
    #[serde(rename = "perm_type")]
    pub perm_type: String,
    /// 权限内容
    pub perm: Option<String>,
}

/// 成员转移数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberTransferData {
    /// 转移结果
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
}

/// 公开链接数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicData {
    /// 链接URL
    #[serde(rename = "url")]
    pub url: String,
    /// 链接token
    #[serde(rename = "link_token")]
    pub link_token: String,
    /// 是否需要密码
    #[serde(rename = "need_password")]
    pub need_password: bool,
    /// 过期时间
    #[serde(rename = "expire_time")]
    pub expire_time: Option<i64>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "open_id")]
    pub open_id: String,
    /// 用户名称
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
    /// 头像
    #[serde(rename = "avatar_url")]
    pub avatar_url: Option<String>,
}