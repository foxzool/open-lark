use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 附件
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// 文件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// 文件名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RetrievalConfig {
    // TODO: Add fields
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

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_page_response_serialization() {
        let response = PageResponse {
            has_more: Some(true),
            page_token: Some("next_token".to_string()),
            items: Some(vec!["item1".to_string(), "item2".to_string()]),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("true"));
        assert!(json.contains("next_token"));
        assert!(json.contains("item1"));
    }

    #[test]
    fn test_page_response_empty() {
        let response: PageResponse<String> = PageResponse {
            has_more: Some(false),
            page_token: None,
            items: Some(vec![]),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("false"));
        assert!(!json.contains("page_token"));
    }

    #[test]
    fn test_session_create_request() {
        let mut metadata = HashMap::new();
        metadata.insert("user_id".to_string(), serde_json::json!("user123"));

        let request = SessionCreateRequest {
            app_id: "app456".to_string(),
            metadata: Some(metadata),
            tool_set: Some(ToolSet {
                tools: Some(vec![Tool {
                    tool_type: Some("function".to_string()),
                    config: Some(serde_json::json!({
                        "name": "get_weather",
                        "description": "Get weather information"
                    })),
                }]),
            }),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("app456"));
        assert!(json.contains("user123"));
        assert!(json.contains("get_weather"));
    }

    #[test]
    fn test_session_update_request() {
        let request = SessionUpdateRequest {
            app_id: "app789".to_string(),
            session_id: "session123".to_string(),
            metadata: Some(HashMap::new()),
            tool_set: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("app789"));
        assert!(json.contains("session123"));
        assert!(!json.contains("tool_set"));
    }

    #[test]
    fn test_session_get_request() {
        let request = SessionGetRequest {
            app_id: "app001".to_string(),
            session_id: "session456".to_string(),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("app001"));
        assert!(json.contains("session456"));
    }

    #[test]
    fn test_session_complete() {
        let session = Session {
            session_id: Some("sess789".to_string()),
            app_id: Some("app123".to_string()),
            created_at: Some("2024-01-01T00:00:00Z".to_string()),
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("context".to_string(), serde_json::json!("conversation"));
                map
            }),
            tool_set: Some(ToolSet {
                tools: Some(vec![]),
            }),
        };
        let json = serde_json::to_string(&session).unwrap();
        assert!(json.contains("sess789"));
        assert!(json.contains("app123"));
        assert!(json.contains("conversation"));
    }

    #[test]
    fn test_tool_set_with_function() {
        let tool_set = ToolSet {
            tools: Some(vec![Tool {
                tool_type: Some("function".to_string()),
                config: Some(serde_json::json!({
                    "name": "calculate",
                    "description": "Perform calculations",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "expression": {"type": "string"}
                        }
                    }
                })),
            }]),
        };
        let json = serde_json::to_string(&tool_set).unwrap();
        assert!(json.contains("function"));
        assert!(json.contains("calculate"));
        assert!(json.contains("parameters"));
    }

    #[test]
    fn test_message_create_request() {
        let request = MessageCreateRequest {
            app_id: "app555".to_string(),
            session_id: "session888".to_string(),
            content: "Hello, assistant!".to_string(),
            message_type: Some("text".to_string()),
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("priority".to_string(), serde_json::json!("high"));
                map
            }),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("app555"));
        assert!(json.contains("session888"));
        assert!(json.contains("Hello, assistant!"));
        assert!(json.contains("high"));
        assert!(json.contains("text"));
    }

    #[test]
    fn test_message_list_request() {
        let request = MessageListRequest {
            app_id: "app777".to_string(),
            session_id: "session999".to_string(),
            page_size: Some(50),
            page_token: Some("page_token_123".to_string()),
            order: Some("desc".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("app777"));
        assert!(json.contains("session999"));
        assert!(json.contains("50"));
        assert!(json.contains("desc"));
        assert!(json.contains("page_token_123"));
    }

    #[test]
    fn test_message_with_assistant_role() {
        let message = Message {
            message_id: Some("msg456".to_string()),
            session_id: Some("session123".to_string()),
            role: Some("assistant".to_string()),
            content: Some("I can help you with that task.".to_string()),
            message_type: Some("text".to_string()),
            created_at: Some("2024-01-01T10:00:00Z".to_string()),
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("confidence".to_string(), serde_json::json!(0.95));
                map
            }),
        };
        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("msg456"));
        assert!(json.contains("assistant"));
        assert!(json.contains("I can help you"));
        assert!(json.contains("0.95"));
    }

    #[test]
    fn test_run_create_request() {
        let request = RunCreateRequest {
            app_id: "app666".to_string(),
            session_id: "session444".to_string(),
            instructions: Some("Please analyze the uploaded document".to_string()),
            model: Some("gpt-4".to_string()),
            additional_messages: Some(vec![]),
            tool_set: Some(ToolSet {
                tools: Some(vec![Tool {
                    tool_type: Some("code_interpreter".to_string()),
                    config: None,
                }]),
            }),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("app666"));
        assert!(json.contains("session444"));
        assert!(json.contains("analyze the uploaded"));
        assert!(json.contains("code_interpreter"));
    }

    #[test]
    fn test_run_list_request() {
        let request = RunListRequest {
            app_id: "app888".to_string(),
            session_id: "session777".to_string(),
            page_size: Some(20),
            page_token: Some("run123".to_string()),
            order: Some("asc".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("app888"));
        assert!(json.contains("session777"));
        assert!(json.contains("20"));
        assert!(json.contains("asc"));
        assert!(json.contains("run123"));
    }

    #[test]
    fn test_run_in_progress() {
        let run = Run {
            run_id: Some("run789".to_string()),
            session_id: Some("session456".to_string()),
            status: Some("in_progress".to_string()),
            created_at: Some("2024-01-01T14:00:00Z".to_string()),
            started_at: Some("2024-01-01T14:00:00Z".to_string()),
            completed_at: None,
            failed_at: None,
            cancelled_at: None,
            instructions: Some("Generate a summary report".to_string()),
            model: Some("gpt-4".to_string()),
            tool_set: Some(ToolSet {
                tools: Some(vec![Tool {
                    tool_type: Some("code_interpreter".to_string()),
                    config: None,
                }]),
            }),
            last_error: None,
        };
        let json = serde_json::to_string(&run).unwrap();
        assert!(json.contains("run789"));
        assert!(json.contains("in_progress"));
        assert!(json.contains("Generate a summary"));
        assert!(!json.contains("completed_at"));
    }

    #[test]
    fn test_run_failed() {
        let run = Run {
            run_id: Some("run999".to_string()),
            session_id: Some("session111".to_string()),
            status: Some("failed".to_string()),
            created_at: Some("2024-01-01T15:00:00Z".to_string()),
            started_at: Some("2024-01-01T15:00:00Z".to_string()),
            completed_at: None,
            failed_at: Some("2024-01-01T15:05:00Z".to_string()),
            cancelled_at: None,
            instructions: Some("Process large dataset".to_string()),
            model: Some("gpt-4".to_string()),
            tool_set: None,
            last_error: Some(RunError {
                code: Some("TIMEOUT".to_string()),
                message: Some("Processing timeout".to_string()),
            }),
        };
        let json = serde_json::to_string(&run).unwrap();
        assert!(json.contains("run999"));
        assert!(json.contains("failed"));
        assert!(json.contains("TIMEOUT"));
        assert!(json.contains("Processing timeout"));
        assert!(json.contains("failed_at"));
    }

    #[test]
    fn test_attachment_complete() {
        let attachment = Attachment {
            file_id: Some("file789".to_string()),
            file_name: Some("presentation.pptx".to_string()),
            file_url: Some("https://files.example.com/file789".to_string()),
            file_size: Some(2048000),
            file_type: Some("application/vnd.ms-powerpoint".to_string()),
            uploaded_at: Some("2024-01-01T16:30:00Z".to_string()),
        };
        let json = serde_json::to_string(&attachment).unwrap();
        assert!(json.contains("file789"));
        assert!(json.contains("presentation.pptx"));
        assert!(json.contains("files.example.com"));
        assert!(json.contains("2048000"));
        assert!(json.contains("vnd.ms-powerpoint"));
    }

    #[test]
    fn test_attachment_minimal() {
        let attachment = Attachment {
            file_id: Some("file456".to_string()),
            file_name: Some("note.txt".to_string()),
            file_url: None,
            file_size: None,
            file_type: Some("text/plain".to_string()),
            uploaded_at: None,
        };
        let json = serde_json::to_string(&attachment).unwrap();
        assert!(json.contains("file456"));
        assert!(json.contains("note.txt"));
        assert!(json.contains("text/plain"));
        assert!(!json.contains("file_url"));
        assert!(!json.contains("uploaded_at"));
    }
}
