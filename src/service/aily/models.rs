use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 分页响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记，下次请求的起点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 数据项目列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<T>>,
}

// ============ 会话相关结构 ============

/// 会话创建请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionCreateRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 会话元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
    /// 工具配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_set: Option<ToolSet>,
}

/// 会话更新请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUpdateRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 会话ID
    pub session_id: String,
    /// 会话元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
    /// 工具配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_set: Option<ToolSet>,
}

/// 会话查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionGetRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 会话ID
    pub session_id: String,
}

/// 会话删除请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionDeleteRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 会话ID
    pub session_id: String,
}

/// 会话信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    /// 会话ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// 智能伙伴ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 会话创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 会话元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
    /// 工具配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_set: Option<ToolSet>,
}

/// 工具配置
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolSet {
    /// 工具列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
}

/// 工具信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    /// 工具类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_type: Option<String>,
    /// 工具配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
}

// ============ 消息相关结构 ============

/// 消息发送请求
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCreateRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 会话ID
    pub session_id: String,
    /// 消息内容
    pub content: String,
    /// 消息类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// 消息元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
}

/// 消息查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageGetRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 会话ID
    pub session_id: String,
    /// 消息ID
    pub message_id: String,
}

/// 消息列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageListRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 会话ID
    pub session_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 排序方式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// 消息信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// 消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 会话ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// 消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 消息类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// 消息角色（user/assistant）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 消息创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 消息元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
}

// ============ 运行相关结构 ============

/// 运行创建请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RunCreateRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 会话ID
    pub session_id: String,
    /// 运行参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// 模型参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// 附加消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_messages: Option<Vec<Message>>,
    /// 工具配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_set: Option<ToolSet>,
}

/// 运行查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RunGetRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 会话ID
    pub session_id: String,
    /// 运行ID
    pub run_id: String,
}

/// 运行列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RunListRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 会话ID
    pub session_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 排序方式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// 运行取消请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RunCancelRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 会话ID
    pub session_id: String,
    /// 运行ID
    pub run_id: String,
}

/// 运行信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Run {
    /// 运行ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// 会话ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// 运行状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 运行创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 运行开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// 运行完成时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// 运行失败时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<String>,
    /// 运行取消时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
    /// 运行指令
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// 使用的模型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// 工具配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_set: Option<ToolSet>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<RunError>,
}

/// 运行错误信息
#[derive(Debug, Serialize, Deserialize)]
pub struct RunError {
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// ============ 技能相关结构 ============

/// 技能调用请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillStartRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 技能ID
    pub skill_id: String,
    /// 输入参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<HashMap<String, Value>>,
    /// 会话ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// 技能信息查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillGetRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 技能ID
    pub skill_id: String,
}

/// 技能列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillListRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 技能信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    /// 技能ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<String>,
    /// 技能名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 技能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 技能类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_type: Option<String>,
    /// 技能状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 输入参数配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_config: Option<Value>,
    /// 输出参数配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<Value>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// 技能执行结果
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillExecution {
    /// 执行ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// 技能ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<String>,
    /// 执行状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 输入参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<HashMap<String, Value>>,
    /// 输出结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<HashMap<String, Value>>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// 完成时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

// ============ 知识问答相关结构 ============

/// 数据知识问答请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeAskRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 问题
    pub question: String,
    /// 知识库ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_ids: Option<Vec<String>>,
    /// 对话历史
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_history: Option<Vec<ChatMessage>>,
    /// 检索配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_config: Option<RetrievalConfig>,
}

/// 对话消息
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    /// 消息角色
    pub role: String,
    /// 消息内容
    pub content: String,
}

/// 检索配置
#[derive(Debug, Serialize, Deserialize)]
pub struct RetrievalConfig {
    /// 检索文档数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    /// 相似度阈值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_threshold: Option<f64>,
}

/// 数据知识问答结果
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeAnswer {
    /// 回答内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    /// 引用的文档
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<KnowledgeReference>>,
    /// 相关性评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f64>,
}

/// 知识引用
#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeReference {
    /// 文档ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// 文档标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 引用片段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 相关性评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevance_score: Option<f64>,
}

// ============ 数据知识管理相关结构 ============

/// 文件上传请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeFileUploadRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 文件内容（base64编码或文件token）
    pub file: String,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

/// 数据知识创建请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeCreateRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 数据知识标题
    pub title: String,
    /// 数据知识内容
    pub content: String,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
}

/// 数据知识查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeGetRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 数据知识ID
    pub knowledge_id: String,
}

/// 数据知识删除请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeDeleteRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 数据知识ID
    pub knowledge_id: String,
}

/// 数据知识列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeListRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 搜索关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

/// 数据知识分类列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeCategoryListRequest {
    /// 智能伙伴ID
    pub app_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 数据知识信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledge {
    /// 数据知识ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 分类名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
}

/// 数据知识分类
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeCategory {
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 分类名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 分类描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 父分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 排序权重
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// 文件上传结果
#[derive(Debug, Serialize, Deserialize)]
pub struct FileUploadResult {
    /// 文件token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 上传时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<String>,
}
