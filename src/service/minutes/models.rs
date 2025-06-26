use serde::{Deserialize, Serialize};

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserIdType {
    /// 用户ID
    #[serde(rename = "user_id")]
    UserId,
    /// union_id
    #[serde(rename = "union_id")]
    UnionId,
    /// open_id
    #[serde(rename = "open_id")]
    OpenId,
}

impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
            UserIdType::OpenId => "open_id",
        }
    }
}

/// 妙记信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minute {
    /// 妙记ID
    pub minute_id: String,
    /// 妙记标题
    pub title: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 创建者
    pub creator: Option<UserInfo>,
    /// 状态
    pub status: Option<String>,
    /// 会议链接
    pub meeting_url: Option<String>,
    /// 会议ID
    pub meeting_id: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub id: String,
    /// 用户名称
    pub name: Option<String>,
    /// 用户头像
    pub avatar_url: Option<String>,
}

/// 妙记音视频文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteMedia {
    /// 文件ID
    pub file_id: String,
    /// 文件名
    pub filename: Option<String>,
    /// 文件大小
    pub file_size: Option<i64>,
    /// 文件类型
    pub file_type: Option<String>,
    /// 下载URL
    pub download_url: Option<String>,
    /// 有效期
    pub expires_time: Option<String>,
}

/// 妙记文字记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteTranscript {
    /// 转录内容
    pub content: String,
    /// 语言
    pub language: Option<String>,
    /// 格式
    pub format: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
}

/// 妙记统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteStatistics {
    /// 会议时长（秒）
    pub duration: Option<i32>,
    /// 参会人数
    pub participant_count: Option<i32>,
    /// 发言次数
    pub speech_count: Option<i32>,
    /// 发言时长（秒）
    pub speech_duration: Option<i32>,
    /// 静音时长（秒）
    pub mute_duration: Option<i32>,
    /// 关键词统计
    pub keywords: Option<Vec<KeywordStatistic>>,
}

/// 关键词统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordStatistic {
    /// 关键词
    pub keyword: String,
    /// 出现次数
    pub count: i32,
}
