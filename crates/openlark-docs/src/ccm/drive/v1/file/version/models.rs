use serde::{Deserialize, Serialize};
use openlark_core::api::ApiResponseTrait;

/// 版本文档信息（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileVersionInfo {
    /// 版本文档的标题
    pub name: String,
    /// 版本文档的版本标识
    pub version: String,
    /// 当前版本对应的源文档的 token
    pub parent_token: String,
    /// 版本文档的所有者的 ID
    pub owner_id: String,
    /// 版本文档的创建者的 ID
    pub creator_id: String,
    /// 版本文档的创建时间（Unix 时间戳，单位秒）
    pub create_time: String,
    /// 版本文档的更新时间（创建版本时可能不返回）
    pub update_time: Option<String>,
    /// 版本文档的状态
    pub status: String,
    /// 版本文档的类型
    pub obj_type: String,
    /// 源文档的类型
    pub parent_type: String,
}

/// 获取文件版本列表响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFileVersionsData {
    /// 版本文档列表
    pub items: Vec<FileVersionInfo>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
}

impl ApiResponseTrait for FileVersionInfo {}
impl ApiResponseTrait for ListFileVersionsData {}
