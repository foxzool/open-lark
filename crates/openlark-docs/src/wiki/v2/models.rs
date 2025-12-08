//! Wiki V2 API 数据模型

use serde::{Deserialize, Serialize};
use openlark_core::api::{ApiResponseTrait, ResponseFormat};

/// 知识空间基础信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSpace {
    /// 知识空间ID
    pub space_id: String,
    /// 知识空间名称
    pub name: String,
    /// 知识空间描述
    pub description: Option<String>,
    /// 知识空间图标
    pub icon: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 创建者ID
    pub creator_id: String,
    /// 空间类型
    pub space_type: String,
    /// 是否为公开空间
    pub is_public: bool,
    /// 空间域名
    pub domain: Option<String>,
    /// 空间状态
    pub status: String,
}

impl ApiResponseTrait for WikiSpace {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 知识空间节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSpaceNode {
    /// 节点Token
    pub node_token: String,
    /// 节点标题
    pub title: String,
    /// 节点类型
    pub node_type: String,
    /// 父节点Token
    pub parent_node_token: Option<String>,
    /// 节点URL
    pub url: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 创建者ID
    pub creator_id: String,
    /// 节点深度
    pub depth: i32,
    /// 是否有子节点
    pub has_child: bool,
    /// 子节点数量
    pub child_count: i32,
}

impl ApiResponseTrait for WikiSpaceNode {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 知识空间成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSpaceMember {
    /// 成员ID
    pub member_id: String,
    /// 成员类型
    pub member_type: String,
    /// 成员名称
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
    /// 成员角色
    pub role: String,
    /// 加入时间
    pub join_time: String,
    /// 操作者ID
    pub operate_id: String,
    /// 操作时间
    pub operate_time: String,
}

impl ApiResponseTrait for WikiSpaceMember {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 知识空间设置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSpaceSetting {
    /// 知识空间ID
    pub space_id: String,
    /// 空间名称
    pub name: String,
    /// 空间描述
    pub description: Option<String>,
    /// 空间图标
    pub icon: Option<String>,
    /// 是否为公开空间
    pub is_public: bool,
    /// 空间域名
    pub domain: Option<String>,
    /// 谁可以创建节点
    pub who_can_create_node: String,
    /// 谁可以查看节点
    pub who_can_view_node: String,
    /// 谁可以编辑节点
    pub who_can_edit_node: String,
}

impl ApiResponseTrait for WikiSpaceSetting {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiTask {
    /// 任务ID
    pub task_id: String,
    /// 任务类型
    pub task_type: String,
    /// 任务状态
    pub status: String,
    /// 任务进度
    pub progress: i32,
    /// 创建时间
    pub create_time: String,
    /// 完成时间
    pub complete_time: Option<String>,
    /// 错误信息
    pub error_message: Option<String>,
    /// 任务结果
    pub result: Option<serde_json::Value>,
}

impl ApiResponseTrait for WikiTask {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSearchResult {
    /// 节点Token
    pub node_token: String,
    /// 节点标题
    pub title: String,
    /// 节点类型
    pub node_type: String,
    /// 父节点Token
    pub parent_node_token: Option<String>,
    /// 空间ID
    pub space_id: String,
    /// 空间名称
    pub space_name: String,
    /// 节点URL
    pub url: String,
    /// 内容摘要
    pub snippet: Option<String>,
    /// 最后更新时间
    pub last_update_time: String,
}

impl ApiResponseTrait for WikiSearchResult {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 节点移动请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveNodeRequest {
    /// 目标父节点Token
    pub parent_node_token: String,
    /// 插入位置
    pub position: Option<String>,
}

/// 节点复制请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyNodeRequest {
    /// 目标父节点Token
    pub parent_node_token: String,
    /// 新节点标题
    pub title: Option<String>,
    /// 插入位置
    pub position: Option<String>,
}

/// 节点标题更新请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNodeTitleRequest {
    /// 新标题
    pub title: String,
}

/// 搜索请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchWikiRequest {
    /// 搜索关键词
    pub query: String,
    /// 空间ID列表
    pub space_ids: Option<Vec<String>>,
    /// 节点类型过滤
    pub node_type: Option<String>,
    /// 每页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 获取知识空间成员列表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpaceMembersParams {
    /// 每页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 添加知识空间成员请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceMemberParams {
    /// 成员ID列表
    pub member_ids: Vec<String>,
    /// 成员类型
    pub member_type: String,
    /// 成员角色
    pub role: String,
}

/// 获取知识空间节点列表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpaceNodesParams {
    /// 每页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 父节点Token
    pub parent_node_token: Option<String>,
    /// 节点类型过滤
    pub node_type: Option<String>,
}

/// 移动云空间文档至知识空间请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDocsToWikiRequest {
    /// 源文档Token列表
    pub obj_tokens: Vec<String>,
    /// 目标父节点Token
    pub parent_node_token: String,
    /// 插入位置
    pub position: Option<String>,
}