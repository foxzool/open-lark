use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 草稿状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftStatus {
    /// 草稿中
    Draft,
    /// 已发布
    Published,
    /// 已拒绝
    Rejected,
}

/// 草稿信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Draft {
    /// 草稿ID
    pub draft_id: String,
    /// 词条ID（如果是更新现有词条的草稿）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// 主名称
    pub main_keys: Vec<String>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// 词条释义富文本
    pub description: String,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_id: Option<String>,
    /// 外链（用于跳转到释义页面）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    /// 相关词条ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    /// 统计信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
    /// 草稿状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DraftStatus>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 词条信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// 词条ID
    pub id: String,
    /// 主名称
    pub main_keys: Vec<String>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// 词条释义富文本
    pub description: String,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_id: Option<String>,
    /// 外链（用于跳转到释义页面）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    /// 相关词条ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    /// 统计信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
    /// 词库ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 外链信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OuterInfo {
    /// 链接提供方
    pub provider: String,
    /// 外部链接
    pub outer_url: String,
}

/// 相关词条信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedMeta {
    /// 相关用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// 相关群组列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chats: Option<Vec<String>>,
    /// 相关文档列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<Vec<String>>,
    /// 相关链接列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oncalls: Option<Vec<String>>,
    /// 相关链接列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// 相关词条列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviations: Option<Vec<String>>,
    /// 分类列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifications: Option<Vec<String>>,
    /// 图片列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
}

/// 统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    /// 点赞数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i32>,
    /// 不喜欢数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dislike_count: Option<i32>,
}

/// 分类信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classification {
    /// 分类ID
    pub id: String,
    /// 分类名称
    pub name: String,
    /// 父分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub father_id: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 词库信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    /// 词库ID
    pub id: String,
    /// 词库名称
    pub name: String,
    /// 词库描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 词库类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_type: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 词条搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySearchResult {
    /// 词条信息
    pub entity: Entity,
    /// 匹配的关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_keys: Option<Vec<String>>,
    /// 匹配分数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

/// 词条匹配结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMatchResult {
    /// 词条信息
    pub entity: Entity,
    /// 匹配的词
    pub matched_word: String,
    /// 匹配类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
}

/// 高亮范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightRange {
    /// 开始位置
    pub start: i32,
    /// 结束位置
    pub end: i32,
    /// 词条ID
    pub entity_id: String,
}

/// 高亮结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightResult {
    /// 原始文本
    pub text: String,
    /// 高亮范围列表
    pub ranges: Vec<HighlightRange>,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件ID
    pub file_token: String,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 上传时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<i64>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_page_response_serialization() {
        let page_response = PageResponse {
            items: vec!["item1".to_string(), "item2".to_string()],
            page_token: Some("next_page_token".to_string()),
            has_more: Some(true),
        };
        let serialized = serde_json::to_string(&page_response).unwrap();
        let deserialized: PageResponse<String> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(page_response.items, deserialized.items);
        assert_eq!(page_response.page_token, deserialized.page_token);
        assert_eq!(page_response.has_more, deserialized.has_more);
    }

    #[test]
    fn test_page_response_with_none_values() {
        let page_response: PageResponse<String> = PageResponse {
            items: vec![],
            page_token: None,
            has_more: None,
        };
        let serialized = serde_json::to_string(&page_response).unwrap();
        assert!(!serialized.contains("page_token"));
        assert!(!serialized.contains("has_more"));
        let deserialized: PageResponse<String> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(page_response.items, deserialized.items);
        assert_eq!(page_response.page_token, deserialized.page_token);
        assert_eq!(page_response.has_more, deserialized.has_more);
    }

    #[test]
    fn test_draft_status_serialization() {
        let draft_status = DraftStatus::Draft;
        let serialized = serde_json::to_string(&draft_status).unwrap();
        assert_eq!(serialized, "\"draft\"");
        let deserialized: DraftStatus = serde_json::from_str(&serialized).unwrap();
        assert!(matches!(deserialized, DraftStatus::Draft));

        let published_status = DraftStatus::Published;
        let serialized = serde_json::to_string(&published_status).unwrap();
        assert_eq!(serialized, "\"published\"");

        let rejected_status = DraftStatus::Rejected;
        let serialized = serde_json::to_string(&rejected_status).unwrap();
        assert_eq!(serialized, "\"rejected\"");
    }

    #[test]
    fn test_draft_serialization() {
        let draft = Draft {
            draft_id: "draft_123".to_string(),
            entity_id: Some("entity_456".to_string()),
            main_keys: vec!["主键1".to_string(), "主键2".to_string()],
            aliases: Some(vec!["别名1".to_string()]),
            description: "词条描述".to_string(),
            classification_id: Some("class_789".to_string()),
            outer_info: Some(OuterInfo {
                provider: "provider_name".to_string(),
                outer_url: "https://example.com".to_string(),
            }),
            related_meta: Some(RelatedMeta {
                users: Some(vec!["user1".to_string()]),
                chats: Some(vec!["chat1".to_string()]),
                docs: Some(vec!["doc1".to_string()]),
                oncalls: Some(vec!["oncall1".to_string()]),
                links: Some(vec!["link1".to_string()]),
                abbreviations: Some(vec!["abbr1".to_string()]),
                classifications: Some(vec!["class1".to_string()]),
                images: Some(vec!["image1".to_string()]),
            }),
            statistics: Some(Statistics {
                like_count: Some(10),
                dislike_count: Some(2),
            }),
            status: Some(DraftStatus::Draft),
            creator: Some("creator_123".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
        };
        let serialized = serde_json::to_string(&draft).unwrap();
        let deserialized: Draft = serde_json::from_str(&serialized).unwrap();
        assert_eq!(draft.draft_id, deserialized.draft_id);
        assert_eq!(draft.entity_id, deserialized.entity_id);
        assert_eq!(draft.main_keys, deserialized.main_keys);
        assert_eq!(draft.description, deserialized.description);
    }

    #[test]
    fn test_draft_with_none_values() {
        let draft = Draft {
            draft_id: "draft_123".to_string(),
            entity_id: None,
            main_keys: vec!["主键".to_string()],
            aliases: None,
            description: "简单描述".to_string(),
            classification_id: None,
            outer_info: None,
            related_meta: None,
            statistics: None,
            status: None,
            creator: None,
            created_at: None,
            updated_at: None,
        };
        let serialized = serde_json::to_string(&draft).unwrap();
        assert!(!serialized.contains("entity_id"));
        assert!(!serialized.contains("aliases"));
        assert!(!serialized.contains("classification_id"));
        let deserialized: Draft = serde_json::from_str(&serialized).unwrap();
        assert_eq!(draft.draft_id, deserialized.draft_id);
        assert_eq!(draft.entity_id, deserialized.entity_id);
    }

    #[test]
    fn test_entity_serialization() {
        let entity = Entity {
            id: "entity_123".to_string(),
            main_keys: vec!["主键1".to_string(), "主键2".to_string()],
            aliases: Some(vec!["别名1".to_string(), "别名2".to_string()]),
            description: "词条的详细描述".to_string(),
            classification_id: Some("class_456".to_string()),
            outer_info: Some(OuterInfo {
                provider: "wiki".to_string(),
                outer_url: "https://wiki.example.com/term".to_string(),
            }),
            related_meta: Some(RelatedMeta {
                users: Some(vec!["user1".to_string(), "user2".to_string()]),
                chats: None,
                docs: Some(vec!["doc1".to_string()]),
                oncalls: None,
                links: Some(vec!["https://link1.com".to_string()]),
                abbreviations: None,
                classifications: Some(vec!["tech".to_string()]),
                images: Some(vec!["image1.jpg".to_string()]),
            }),
            statistics: Some(Statistics {
                like_count: Some(25),
                dislike_count: Some(3),
            }),
            repo_id: Some("repo_789".to_string()),
            creator: Some("creator_456".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1641081600),
        };
        let serialized = serde_json::to_string(&entity).unwrap();
        let deserialized: Entity = serde_json::from_str(&serialized).unwrap();
        assert_eq!(entity.id, deserialized.id);
        assert_eq!(entity.main_keys, deserialized.main_keys);
        assert_eq!(entity.description, deserialized.description);
        assert_eq!(entity.repo_id, deserialized.repo_id);
    }

    #[test]
    fn test_outer_info_serialization() {
        let outer_info = OuterInfo {
            provider: "external_wiki".to_string(),
            outer_url: "https://external.wiki.com/term/123".to_string(),
        };
        let serialized = serde_json::to_string(&outer_info).unwrap();
        let deserialized: OuterInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(outer_info.provider, deserialized.provider);
        assert_eq!(outer_info.outer_url, deserialized.outer_url);
    }

    #[test]
    fn test_related_meta_serialization() {
        let related_meta = RelatedMeta {
            users: Some(vec!["user1".to_string(), "user2".to_string()]),
            chats: Some(vec!["chat1".to_string()]),
            docs: Some(vec!["doc1".to_string(), "doc2".to_string()]),
            oncalls: Some(vec!["oncall1".to_string()]),
            links: Some(vec!["https://example1.com".to_string()]),
            abbreviations: Some(vec!["API".to_string(), "SDK".to_string()]),
            classifications: Some(vec!["tech".to_string(), "business".to_string()]),
            images: Some(vec!["image1.png".to_string(), "image2.jpg".to_string()]),
        };
        let serialized = serde_json::to_string(&related_meta).unwrap();
        let deserialized: RelatedMeta = serde_json::from_str(&serialized).unwrap();
        assert_eq!(related_meta.users, deserialized.users);
        assert_eq!(related_meta.chats, deserialized.chats);
        assert_eq!(related_meta.docs, deserialized.docs);
        assert_eq!(related_meta.links, deserialized.links);
    }

    #[test]
    fn test_related_meta_with_none_values() {
        let related_meta = RelatedMeta {
            users: Some(vec!["user1".to_string()]),
            chats: None,
            docs: None,
            oncalls: None,
            links: None,
            abbreviations: None,
            classifications: None,
            images: None,
        };
        let serialized = serde_json::to_string(&related_meta).unwrap();
        assert!(!serialized.contains("chats"));
        assert!(!serialized.contains("docs"));
        assert!(!serialized.contains("oncalls"));
        let deserialized: RelatedMeta = serde_json::from_str(&serialized).unwrap();
        assert_eq!(related_meta.users, deserialized.users);
        assert_eq!(related_meta.chats, deserialized.chats);
    }

    #[test]
    fn test_statistics_serialization() {
        let statistics = Statistics {
            like_count: Some(42),
            dislike_count: Some(7),
        };
        let serialized = serde_json::to_string(&statistics).unwrap();
        let deserialized: Statistics = serde_json::from_str(&serialized).unwrap();
        assert_eq!(statistics.like_count, deserialized.like_count);
        assert_eq!(statistics.dislike_count, deserialized.dislike_count);
    }

    #[test]
    fn test_statistics_with_none_values() {
        let statistics = Statistics {
            like_count: None,
            dislike_count: None,
        };
        let serialized = serde_json::to_string(&statistics).unwrap();
        assert!(!serialized.contains("like_count"));
        assert!(!serialized.contains("dislike_count"));
        let deserialized: Statistics = serde_json::from_str(&serialized).unwrap();
        assert_eq!(statistics.like_count, deserialized.like_count);
        assert_eq!(statistics.dislike_count, deserialized.dislike_count);
    }

    #[test]
    fn test_classification_serialization() {
        let classification = Classification {
            id: "class_123".to_string(),
            name: "技术分类".to_string(),
            father_id: Some("parent_class_456".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1641081600),
        };
        let serialized = serde_json::to_string(&classification).unwrap();
        let deserialized: Classification = serde_json::from_str(&serialized).unwrap();
        assert_eq!(classification.id, deserialized.id);
        assert_eq!(classification.name, deserialized.name);
        assert_eq!(classification.father_id, deserialized.father_id);
        assert_eq!(classification.created_at, deserialized.created_at);
    }

    #[test]
    fn test_classification_root_category() {
        let classification = Classification {
            id: "root_class".to_string(),
            name: "根分类".to_string(),
            father_id: None,
            created_at: Some(1640995200),
            updated_at: None,
        };
        let serialized = serde_json::to_string(&classification).unwrap();
        assert!(!serialized.contains("father_id"));
        assert!(!serialized.contains("updated_at"));
        let deserialized: Classification = serde_json::from_str(&serialized).unwrap();
        assert_eq!(classification.father_id, deserialized.father_id);
        assert_eq!(classification.updated_at, deserialized.updated_at);
    }

    #[test]
    fn test_repo_serialization() {
        let repo = Repo {
            id: "repo_123".to_string(),
            name: "技术词库".to_string(),
            description: Some("包含技术术语的词库".to_string()),
            repo_type: Some("public".to_string()),
            creator: Some("creator_456".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1641081600),
        };
        let serialized = serde_json::to_string(&repo).unwrap();
        let deserialized: Repo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(repo.id, deserialized.id);
        assert_eq!(repo.name, deserialized.name);
        assert_eq!(repo.description, deserialized.description);
        assert_eq!(repo.repo_type, deserialized.repo_type);
    }

    #[test]
    fn test_repo_minimal_data() {
        let repo = Repo {
            id: "repo_minimal".to_string(),
            name: "简单词库".to_string(),
            description: None,
            repo_type: None,
            creator: None,
            created_at: None,
            updated_at: None,
        };
        let serialized = serde_json::to_string(&repo).unwrap();
        assert!(!serialized.contains("description"));
        assert!(!serialized.contains("repo_type"));
        assert!(!serialized.contains("creator"));
        let deserialized: Repo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(repo.id, deserialized.id);
        assert_eq!(repo.name, deserialized.name);
    }

    #[test]
    fn test_entity_search_result_serialization() {
        let entity = Entity {
            id: "entity_search_123".to_string(),
            main_keys: vec!["搜索词条".to_string()],
            aliases: Some(vec!["搜索别名".to_string()]),
            description: "搜索结果描述".to_string(),
            classification_id: None,
            outer_info: None,
            related_meta: None,
            statistics: None,
            repo_id: Some("search_repo".to_string()),
            creator: None,
            created_at: None,
            updated_at: None,
        };
        let search_result = EntitySearchResult {
            entity,
            matched_keys: Some(vec!["搜索词条".to_string(), "搜索".to_string()]),
            score: Some(0.95),
        };
        let serialized = serde_json::to_string(&search_result).unwrap();
        let deserialized: EntitySearchResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(search_result.entity.id, deserialized.entity.id);
        assert_eq!(search_result.matched_keys, deserialized.matched_keys);
        assert_eq!(search_result.score, deserialized.score);
    }

    #[test]
    fn test_entity_match_result_serialization() {
        let entity = Entity {
            id: "entity_match_456".to_string(),
            main_keys: vec!["匹配词条".to_string()],
            aliases: None,
            description: "匹配结果描述".to_string(),
            classification_id: None,
            outer_info: None,
            related_meta: None,
            statistics: None,
            repo_id: None,
            creator: None,
            created_at: None,
            updated_at: None,
        };
        let match_result = EntityMatchResult {
            entity,
            matched_word: "匹配".to_string(),
            match_type: Some("exact".to_string()),
        };
        let serialized = serde_json::to_string(&match_result).unwrap();
        let deserialized: EntityMatchResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(match_result.entity.id, deserialized.entity.id);
        assert_eq!(match_result.matched_word, deserialized.matched_word);
        assert_eq!(match_result.match_type, deserialized.match_type);
    }

    #[test]
    fn test_highlight_range_serialization() {
        let highlight_range = HighlightRange {
            start: 10,
            end: 20,
            entity_id: "highlight_entity_123".to_string(),
        };
        let serialized = serde_json::to_string(&highlight_range).unwrap();
        let deserialized: HighlightRange = serde_json::from_str(&serialized).unwrap();
        assert_eq!(highlight_range.start, deserialized.start);
        assert_eq!(highlight_range.end, deserialized.end);
        assert_eq!(highlight_range.entity_id, deserialized.entity_id);
    }

    #[test]
    fn test_highlight_result_serialization() {
        let highlight_result = HighlightResult {
            text: "这是一段包含高亮词条的文本内容".to_string(),
            ranges: vec![
                HighlightRange {
                    start: 6,
                    end: 8,
                    entity_id: "entity_1".to_string(),
                },
                HighlightRange {
                    start: 9,
                    end: 11,
                    entity_id: "entity_2".to_string(),
                },
            ],
        };
        let serialized = serde_json::to_string(&highlight_result).unwrap();
        let deserialized: HighlightResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(highlight_result.text, deserialized.text);
        assert_eq!(highlight_result.ranges.len(), deserialized.ranges.len());
        assert_eq!(
            highlight_result.ranges[0].start,
            deserialized.ranges[0].start
        );
        assert_eq!(
            highlight_result.ranges[1].entity_id,
            deserialized.ranges[1].entity_id
        );
    }

    #[test]
    fn test_file_info_serialization() {
        let file_info = FileInfo {
            file_token: "file_token_123".to_string(),
            file_name: Some("词条文档.pdf".to_string()),
            file_size: Some(1024000),
            file_type: Some("application/pdf".to_string()),
            uploaded_at: Some(1640995200),
        };
        let serialized = serde_json::to_string(&file_info).unwrap();
        let deserialized: FileInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(file_info.file_token, deserialized.file_token);
        assert_eq!(file_info.file_name, deserialized.file_name);
        assert_eq!(file_info.file_size, deserialized.file_size);
        assert_eq!(file_info.file_type, deserialized.file_type);
        assert_eq!(file_info.uploaded_at, deserialized.uploaded_at);
    }

    #[test]
    fn test_file_info_minimal_data() {
        let file_info = FileInfo {
            file_token: "minimal_file_token".to_string(),
            file_name: None,
            file_size: None,
            file_type: None,
            uploaded_at: None,
        };
        let serialized = serde_json::to_string(&file_info).unwrap();
        assert!(!serialized.contains("file_name"));
        assert!(!serialized.contains("file_size"));
        assert!(!serialized.contains("file_type"));
        assert!(!serialized.contains("uploaded_at"));
        let deserialized: FileInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(file_info.file_token, deserialized.file_token);
        assert_eq!(file_info.file_name, deserialized.file_name);
    }
}
