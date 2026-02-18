//! Baike v1 数据模型
//!
//! 仅存放模型定义，不视为 API 文件。

use serde::{Deserialize, Serialize};

/// 用户 ID 类型（query: user_id_type）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    OpenId,
    UnionId,
    UserId,
}

impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::OpenId => "open_id",
            UserIdType::UnionId => "union_id",
            UserIdType::UserId => "user_id",
        }
    }
}

/// 名称展示范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisplayStatus {
    /// 对应名称是否在消息/云文档高亮
    pub allow_highlight: bool,
    /// 对应名称是否在搜索结果中展示
    pub allow_search: bool,
}

/// 名称（词条名/别名）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Term {
    /// 名称的值
    pub key: String,
    /// 名称展示范围
    pub display_status: DisplayStatus,
}

/// 相关信息条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Referer {
    /// 对应相关信息 ID（部分场景不返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 对应相关信息的描述，如相关联系人的描述、相关链接的标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 链接地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// 更多相关信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelatedMeta {
    /// 相关联系人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<Referer>>,
    /// 相关公开群
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chats: Option<Vec<Referer>>,
    /// 相关云文档
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<Vec<Referer>>,
    /// 相关值班号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oncalls: Option<Vec<Referer>>,
    /// 相关链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Referer>>,
    /// 相关简称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviations: Option<Vec<Referer>>,
    /// 相关分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifications: Option<Vec<ClassificationItem>>,
    /// 相关图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<BaikeImage>>,
}

/// 反馈统计
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityStatistics {
    /// 累计点赞
    pub like_count: i32,
    /// 当前词条版本收到的负反馈数量
    pub dislike_count: i32,
}

/// 外部系统关联数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OuterInfo {
    /// 外部系统（不能包含中横线 "-"）
    pub provider: String,
    /// 词条在外部系统中对应的唯一 ID（不能包含中横线 "-"）
    pub outer_id: String,
}

/// 词条信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entity {
    /// 词条 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 词条名
    pub main_keys: Vec<Term>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Term>>,
    /// 纯文本释义（与 rich_text 二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间（timestamp，字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 创建者（部分接口返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 更新时间（timestamp，字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 更新者（部分接口返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updater: Option<String>,
    /// 更多相关信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    /// 当前词条收到的反馈数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<EntityStatistics>,
    /// 外部系统关联数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    /// 富文本格式（当填写富文本内容时，description 字段将会失效可不填写）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
    /// 词条来源（部分接口返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
}

/// 分类条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassificationItem {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub father_id: Option<String>,
}

/// 图片信息（related_meta.images）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BaikeImage {
    /// 通过文件接口上传图片后，获得的图片 token
    pub token: String,
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
    fn test_user_id_type_serialization() {
        test_roundtrip(&UserIdType::OpenId);
        test_roundtrip(&UserIdType::UnionId);
        test_roundtrip(&UserIdType::UserId);
    }

    #[test]
    fn test_display_status_serialization() {
        let status = DisplayStatus {
            allow_highlight: true,
            allow_search: false,
        };
        test_roundtrip(&status);
    }

    #[test]
    fn test_term_serialization() {
        let term = Term {
            key: "词条名".to_string(),
            display_status: DisplayStatus {
                allow_highlight: true,
                allow_search: true,
            },
        };
        test_roundtrip(&term);
    }

    #[test]
    fn test_referer_serialization() {
        let referer = Referer {
            id: Some("ref123".to_string()),
            title: Some("标题".to_string()),
            url: Some("https://example.com".to_string()),
        };
        test_roundtrip(&referer);
    }

    #[test]
    fn test_related_meta_serialization() {
        let meta = RelatedMeta {
            users: Some(vec![Referer {
                id: Some("user1".to_string()),
                title: Some("用户".to_string()),
                url: None,
            }]),
            chats: None,
            docs: None,
            oncalls: None,
            links: None,
            abbreviations: None,
            classifications: None,
            images: None,
        };
        test_roundtrip(&meta);
    }

    #[test]
    fn test_entity_statistics_serialization() {
        let stats = EntityStatistics {
            like_count: 50,
            dislike_count: 2,
        };
        test_roundtrip(&stats);
    }

    #[test]
    fn test_outer_info_serialization() {
        let info = OuterInfo {
            provider: "provider1".to_string(),
            outer_id: "outer123".to_string(),
        };
        test_roundtrip(&info);
    }

    #[test]
    fn test_entity_serialization() {
        let entity = Entity {
            id: Some("entity123".to_string()),
            main_keys: vec![Term {
                key: "主词条".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            aliases: None,
            description: Some("词条描述".to_string()),
            create_time: Some("1234567890".to_string()),
            creator: Some("creator1".to_string()),
            update_time: None,
            updater: None,
            related_meta: None,
            statistics: None,
            outer_info: None,
            rich_text: None,
            source: Some(1),
        };
        test_roundtrip(&entity);
    }

    #[test]
    fn test_classification_item_serialization() {
        let item = ClassificationItem {
            id: "class123".to_string(),
            name: "分类名称".to_string(),
            father_id: Some("father123".to_string()),
        };
        test_roundtrip(&item);
    }

    #[test]
    fn test_baike_image_serialization() {
        let img = BaikeImage {
            token: "img_token_123".to_string(),
        };
        test_roundtrip(&img);
    }
}
