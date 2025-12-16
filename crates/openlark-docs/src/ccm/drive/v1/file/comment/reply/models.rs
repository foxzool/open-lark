use serde::{Deserialize, Serialize};

/// 评论回复信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyInfo {
    /// 回复ID
    pub reply_id: String,
    /// 回复内容
    pub content: ReplyContent,
    /// 回复者信息
    pub user_id: String,
    /// 创建时间
    pub create_time: i64,
    /// 更新时间
    pub update_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyContent {
    /// 回复文本内容
    pub elements: Vec<ReplyElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyElement {
    #[serde(rename = "type")]
    pub r#type: String,
    pub text_run: Option<TextRun>,
    pub docs_link: Option<DocsLink>,
    pub person: Option<Person>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsLink {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub user_id: String,
}
