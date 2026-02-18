/// 知识库公共数据模型
///
/// 包含baike和lingo服务共用的数据结构定义
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 词条实体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entity {
    /// 词条ID
    pub entity_id: String,
    /// 词条名称
    pub name: String,
    /// 词条别名列表
    pub aliases: Vec<String>,
    /// 词条分类
    pub classifications: Vec<Classification>,
    /// 词条释义
    pub definition: String,
    /// 词条状态
    pub status: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 创建者
    pub creator: String,
    /// 词典ID
    pub repo_id: String,
    /// 词条封面图片
    pub cover: Option<EntityCover>,
    /// 词条扩展属性
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// 词条封面图片
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityCover {
    /// 图片token
    pub file_token: String,
    /// 图片URL
    pub url: String,
}

/// 分类信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Classification {
    /// 分类ID
    pub classification_id: String,
    /// 分类名称
    pub name: String,
    /// 父分类ID
    pub parent_id: Option<String>,
    /// 分类层级
    pub level: i32,
}

/// 词典信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Repository {
    /// 词典ID
    pub repo_id: String,
    /// 词典名称
    pub name: String,
    /// 词典描述
    pub description: Option<String>,
    /// 词条数量
    pub entity_count: i32,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
}

/// 草稿信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Draft {
    /// 草稿ID
    pub draft_id: String,
    /// 草稿标题
    pub title: String,
    /// 草稿内容
    pub content: String,
    /// 草稿状态
    pub status: String,
    /// 操作类型：create/update
    pub operation_type: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 创建者
    pub creator: String,
    /// 审批状态
    pub approval_status: String,
}

/// 词条搜索结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntitySearchResult {
    /// 词条ID
    pub entity_id: String,
    /// 词条名称
    pub name: String,
    /// 词条别名
    pub aliases: Vec<String>,
    /// 词条释义
    pub definition: String,
    /// 匹配分数
    pub score: f64,
    /// 高亮信息
    pub highlights: Option<Vec<HighlightInfo>>,
}

/// 高亮信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HighlightInfo {
    /// 字段名
    pub field: String,
    /// 高亮片段
    pub fragments: Vec<String>,
    /// 开始位置
    pub start_position: i32,
    /// 结束位置
    pub end_position: i32,
}

/// 词条匹配结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityMatchResult {
    /// 词条ID
    pub entity_id: String,
    /// 词条名称
    pub name: String,
    /// 匹配类型：name/alias
    pub match_type: String,
}

/// 词条提取结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityExtractResult {
    /// 提取的词条
    pub entity: String,
    /// 建议的别名
    pub suggested_aliases: Vec<String>,
    /// 置信度
    pub confidence: f64,
    /// 开始位置
    pub start_position: i32,
    /// 结束位置
    pub end_position: i32,
}

/// 文件上传请求
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FileUploadRequest {
    /// 文件名
    pub file_name: String,
    /// 文件内容（base64编码）
    pub file_content: String,
    /// 文件大小
    pub file_size: i64,
    /// 文件类型
    pub content_type: String,
}

/// 文件上传结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileUploadResult {
    /// 文件token
    pub file_token: String,
    /// 文件URL
    pub url: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小
    pub file_size: i64,
    /// 文件类型
    pub content_type: String,
}

/// 分页响应结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageResponse<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 页码
    pub page: i32,
    /// 每页大小
    pub page_size: i32,
    /// 总数
    pub total: i32,
    /// 是否有下一页
    pub has_next: bool,
}

/// API响应包装结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse<T> {
    /// 响应数据
    pub data: Option<T>,
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 请求ID
    pub request_id: Option<String>,
}

/// 词条类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    /// 普通词条
    Normal,
    /// 专业词条
    Professional,
    /// 自定义词条
    Custom,
}

/// 草稿状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DraftStatus {
    /// 编辑中
    Editing,
    /// 待审批
    Pending,
    /// 已批准
    Approved,
    /// 已拒绝
    Rejected,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(
        original: &T,
    ) {
        let json = serde_json::to_string(original).expect("序列化失败");
        let deserialized: T = serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(original, &deserialized, "roundtrip 后数据不一致");
    }

    #[test]
    fn test_entity_serialization() {
        let entity = Entity {
            entity_id: "entity123".to_string(),
            name: "词条名称".to_string(),
            aliases: vec!["别名1".to_string(), "别名2".to_string()],
            classifications: vec![Classification {
                classification_id: "class1".to_string(),
                name: "分类1".to_string(),
                parent_id: None,
                level: 1,
            }],
            definition: "词条定义".to_string(),
            status: "normal".to_string(),
            create_time: "2024-01-01T00:00:00Z".to_string(),
            update_time: "2024-01-02T00:00:00Z".to_string(),
            creator: "user123".to_string(),
            repo_id: "repo123".to_string(),
            cover: None,
            extra: None,
        };
        test_roundtrip(&entity);
    }

    #[test]
    fn test_entity_cover_serialization() {
        let cover = EntityCover {
            file_token: "token123".to_string(),
            url: "https://example.com/cover.jpg".to_string(),
        };
        test_roundtrip(&cover);
    }

    #[test]
    fn test_classification_serialization() {
        let class = Classification {
            classification_id: "class123".to_string(),
            name: "技术类".to_string(),
            parent_id: Some("parent123".to_string()),
            level: 2,
        };
        test_roundtrip(&class);
    }

    #[test]
    fn test_repository_serialization() {
        let repo = Repository {
            repo_id: "repo123".to_string(),
            name: "技术词典".to_string(),
            description: Some("技术相关词条".to_string()),
            entity_count: 100,
            create_time: "2024-01-01T00:00:00Z".to_string(),
            update_time: "2024-01-02T00:00:00Z".to_string(),
        };
        test_roundtrip(&repo);
    }

    #[test]
    fn test_draft_serialization() {
        let draft = Draft {
            draft_id: "draft123".to_string(),
            title: "草稿标题".to_string(),
            content: "草稿内容".to_string(),
            status: "editing".to_string(),
            operation_type: "create".to_string(),
            create_time: "2024-01-01T00:00:00Z".to_string(),
            update_time: "2024-01-02T00:00:00Z".to_string(),
            creator: "user123".to_string(),
            approval_status: "pending".to_string(),
        };
        test_roundtrip(&draft);
    }

    #[test]
    fn test_entity_search_result_serialization() {
        let result = EntitySearchResult {
            entity_id: "entity123".to_string(),
            name: "搜索结果".to_string(),
            aliases: vec!["别名".to_string()],
            definition: "定义".to_string(),
            score: 0.95,
            highlights: None,
        };
        test_roundtrip(&result);
    }

    #[test]
    fn test_highlight_info_serialization() {
        let highlight = HighlightInfo {
            field: "name".to_string(),
            fragments: vec!["<em>高亮</em>".to_string()],
            start_position: 0,
            end_position: 10,
        };
        test_roundtrip(&highlight);
    }

    #[test]
    fn test_entity_match_result_serialization() {
        let result = EntityMatchResult {
            entity_id: "entity123".to_string(),
            name: "匹配结果".to_string(),
            match_type: "name".to_string(),
        };
        test_roundtrip(&result);
    }

    #[test]
    fn test_entity_extract_result_serialization() {
        let result = EntityExtractResult {
            entity: "提取词条".to_string(),
            suggested_aliases: vec!["建议别名".to_string()],
            confidence: 0.85,
            start_position: 0,
            end_position: 4,
        };
        test_roundtrip(&result);
    }

    #[test]
    fn test_file_upload_request_serialization() {
        let req = FileUploadRequest {
            file_name: "test.pdf".to_string(),
            file_content: "base64content".to_string(),
            file_size: 1024,
            content_type: "application/pdf".to_string(),
        };
        test_roundtrip(&req);
    }

    #[test]
    fn test_file_upload_result_serialization() {
        let result = FileUploadResult {
            file_token: "token123".to_string(),
            url: "https://example.com/file".to_string(),
            file_name: "test.pdf".to_string(),
            file_size: 1024,
            content_type: "application/pdf".to_string(),
        };
        test_roundtrip(&result);
    }

    #[test]
    fn test_page_response_serialization() {
        let response: PageResponse<String> = PageResponse {
            items: vec!["item1".to_string(), "item2".to_string()],
            page: 1,
            page_size: 20,
            total: 100,
            has_next: true,
        };
        test_roundtrip(&response);
    }

    #[test]
    fn test_api_response_serialization() {
        let response: ApiResponse<String> = ApiResponse {
            data: Some("test data".to_string()),
            code: 0,
            msg: "success".to_string(),
            request_id: Some("req123".to_string()),
        };
        test_roundtrip(&response);
    }

    #[test]
    fn test_entity_type_serialization() {
        test_roundtrip(&EntityType::Normal);
        test_roundtrip(&EntityType::Professional);
        test_roundtrip(&EntityType::Custom);
    }

    #[test]
    fn test_draft_status_serialization() {
        test_roundtrip(&DraftStatus::Editing);
        test_roundtrip(&DraftStatus::Pending);
        test_roundtrip(&DraftStatus::Approved);
        test_roundtrip(&DraftStatus::Rejected);
    }
}
