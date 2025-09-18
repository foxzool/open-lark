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

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_type_serialization() {
        let user_id = UserIdType::UserId;
        let serialized = serde_json::to_string(&user_id).unwrap();
        assert_eq!(serialized, "\"user_id\"");

        let union_id = UserIdType::UnionId;
        let serialized = serde_json::to_string(&union_id).unwrap();
        assert_eq!(serialized, "\"union_id\"");

        let open_id = UserIdType::OpenId;
        let serialized = serde_json::to_string(&open_id).unwrap();
        assert_eq!(serialized, "\"open_id\"");
    }

    #[test]
    fn test_user_id_type_deserialization() {
        let user_id: UserIdType = serde_json::from_str("\"user_id\"").unwrap();
        assert_eq!(user_id, UserIdType::UserId);

        let union_id: UserIdType = serde_json::from_str("\"union_id\"").unwrap();
        assert_eq!(union_id, UserIdType::UnionId);

        let open_id: UserIdType = serde_json::from_str("\"open_id\"").unwrap();
        assert_eq!(open_id, UserIdType::OpenId);
    }

    #[test]
    fn test_user_id_type_as_str() {
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
    }

    #[test]
    fn test_minute_serialization() {
        let minute = Minute {
            minute_id: "minute_123".to_string(),
            title: Some("项目讨论会议".to_string()),
            create_time: Some("2024-01-15T10:00:00Z".to_string()),
            start_time: Some("2024-01-15T10:00:00Z".to_string()),
            end_time: Some("2024-01-15T11:00:00Z".to_string()),
            creator: Some(UserInfo {
                id: "creator_001".to_string(),
                name: Some("张三".to_string()),
                avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            }),
            status: Some("completed".to_string()),
            meeting_url: Some("https://meeting.example.com/room/123".to_string()),
            meeting_id: Some("meeting_456".to_string()),
        };

        let serialized = serde_json::to_string(&minute).unwrap();
        let deserialized: Minute = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.minute_id, "minute_123");
        assert_eq!(deserialized.title, Some("项目讨论会议".to_string()));
        assert_eq!(
            deserialized.create_time,
            Some("2024-01-15T10:00:00Z".to_string())
        );
        assert!(deserialized.creator.is_some());
        assert_eq!(deserialized.creator.as_ref().unwrap().id, "creator_001");
        assert_eq!(deserialized.status, Some("completed".to_string()));
    }

    #[test]
    fn test_minute_with_none_values() {
        let minute = Minute {
            minute_id: "minute_minimal".to_string(),
            title: None,
            create_time: None,
            start_time: None,
            end_time: None,
            creator: None,
            status: None,
            meeting_url: None,
            meeting_id: None,
        };

        let serialized = serde_json::to_string(&minute).unwrap();
        let deserialized: Minute = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.minute_id, "minute_minimal");
        assert!(deserialized.title.is_none());
        assert!(deserialized.create_time.is_none());
        assert!(deserialized.creator.is_none());
        assert!(deserialized.status.is_none());
    }

    #[test]
    fn test_user_info_serialization() {
        let user = UserInfo {
            id: "user_456".to_string(),
            name: Some("李四".to_string()),
            avatar_url: Some("https://cdn.example.com/user456.png".to_string()),
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: UserInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.id, "user_456");
        assert_eq!(deserialized.name, Some("李四".to_string()));
        assert_eq!(
            deserialized.avatar_url,
            Some("https://cdn.example.com/user456.png".to_string())
        );
    }

    #[test]
    fn test_user_info_minimal() {
        let user = UserInfo {
            id: "user_minimal".to_string(),
            name: None,
            avatar_url: None,
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: UserInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.id, "user_minimal");
        assert!(deserialized.name.is_none());
        assert!(deserialized.avatar_url.is_none());
    }

    #[test]
    fn test_minute_media_serialization() {
        let media = MinuteMedia {
            file_id: "file_789".to_string(),
            filename: Some("meeting_recording.mp4".to_string()),
            file_size: Some(104857600), // 100MB
            file_type: Some("video/mp4".to_string()),
            download_url: Some("https://storage.example.com/file_789".to_string()),
            expires_time: Some("2024-01-22T10:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&media).unwrap();
        let deserialized: MinuteMedia = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.file_id, "file_789");
        assert_eq!(
            deserialized.filename,
            Some("meeting_recording.mp4".to_string())
        );
        assert_eq!(deserialized.file_size, Some(104857600));
        assert_eq!(deserialized.file_type, Some("video/mp4".to_string()));
        assert_eq!(
            deserialized.download_url,
            Some("https://storage.example.com/file_789".to_string())
        );
    }

    #[test]
    fn test_minute_media_audio_file() {
        let audio = MinuteMedia {
            file_id: "audio_001".to_string(),
            filename: Some("meeting_audio.wav".to_string()),
            file_size: Some(52428800), // 50MB
            file_type: Some("audio/wav".to_string()),
            download_url: Some("https://storage.example.com/audio_001".to_string()),
            expires_time: Some("2024-01-23T15:30:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&audio).unwrap();
        let deserialized: MinuteMedia = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.file_type, Some("audio/wav".to_string()));
        assert!(deserialized.file_size.unwrap() > 0);
    }

    #[test]
    fn test_minute_transcript_serialization() {
        let transcript = MinuteTranscript {
            content: "会议讨论了项目进度，决定下周进行代码审查。".to_string(),
            language: Some("zh-CN".to_string()),
            format: Some("text/plain".to_string()),
            create_time: Some("2024-01-15T11:05:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&transcript).unwrap();
        let deserialized: MinuteTranscript = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            deserialized.content,
            "会议讨论了项目进度，决定下周进行代码审查。"
        );
        assert_eq!(deserialized.language, Some("zh-CN".to_string()));
        assert_eq!(deserialized.format, Some("text/plain".to_string()));
    }

    #[test]
    fn test_minute_transcript_english() {
        let transcript = MinuteTranscript {
            content: "The meeting discussed project milestones and deadlines.".to_string(),
            language: Some("en-US".to_string()),
            format: Some("text/markdown".to_string()),
            create_time: Some("2024-01-15T11:10:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&transcript).unwrap();
        let deserialized: MinuteTranscript = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.language, Some("en-US".to_string()));
        assert!(deserialized.content.contains("project"));
    }

    #[test]
    fn test_keyword_statistic_serialization() {
        let keyword = KeywordStatistic {
            keyword: "项目进度".to_string(),
            count: 15,
        };

        let serialized = serde_json::to_string(&keyword).unwrap();
        let deserialized: KeywordStatistic = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.keyword, "项目进度");
        assert_eq!(deserialized.count, 15);
    }

    #[test]
    fn test_minute_statistics_complete() {
        let statistics = MinuteStatistics {
            duration: Some(3600), // 1 hour
            participant_count: Some(8),
            speech_count: Some(42),
            speech_duration: Some(2400), // 40 minutes
            mute_duration: Some(1200),   // 20 minutes
            keywords: Some(vec![
                KeywordStatistic {
                    keyword: "项目".to_string(),
                    count: 25,
                },
                KeywordStatistic {
                    keyword: "进度".to_string(),
                    count: 18,
                },
                KeywordStatistic {
                    keyword: "代码审查".to_string(),
                    count: 8,
                },
            ]),
        };

        let serialized = serde_json::to_string(&statistics).unwrap();
        let deserialized: MinuteStatistics = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.duration, Some(3600));
        assert_eq!(deserialized.participant_count, Some(8));
        assert_eq!(deserialized.speech_count, Some(42));

        let keywords = deserialized.keywords.unwrap();
        assert_eq!(keywords.len(), 3);
        assert_eq!(keywords[0].keyword, "项目");
        assert_eq!(keywords[0].count, 25);
        assert_eq!(keywords[2].keyword, "代码审查");
        assert_eq!(keywords[2].count, 8);
    }

    #[test]
    fn test_minute_statistics_minimal() {
        let statistics = MinuteStatistics {
            duration: None,
            participant_count: None,
            speech_count: None,
            speech_duration: None,
            mute_duration: None,
            keywords: None,
        };

        let serialized = serde_json::to_string(&statistics).unwrap();
        let deserialized: MinuteStatistics = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.duration.is_none());
        assert!(deserialized.participant_count.is_none());
        assert!(deserialized.keywords.is_none());
    }

    #[test]
    fn test_minute_statistics_empty_keywords() {
        let statistics = MinuteStatistics {
            duration: Some(1800), // 30 minutes
            participant_count: Some(3),
            speech_count: Some(12),
            speech_duration: Some(1200),
            mute_duration: Some(600),
            keywords: Some(vec![]), // Empty vector
        };

        let serialized = serde_json::to_string(&statistics).unwrap();
        let deserialized: MinuteStatistics = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.duration, Some(1800));
        let keywords = deserialized.keywords.unwrap();
        assert_eq!(keywords.len(), 0);
    }

    #[test]
    fn test_complex_minute_with_full_data() {
        let complex_minute = Minute {
            minute_id: "complex_minute_001".to_string(),
            title: Some("季度总结会议 - Q1 2024".to_string()),
            create_time: Some("2024-03-28T14:00:00Z".to_string()),
            start_time: Some("2024-03-28T14:00:00Z".to_string()),
            end_time: Some("2024-03-28T16:30:00Z".to_string()),
            creator: Some(UserInfo {
                id: "manager_001".to_string(),
                name: Some("王经理".to_string()),
                avatar_url: Some("https://company.com/avatars/manager.jpg".to_string()),
            }),
            status: Some("completed".to_string()),
            meeting_url: Some("https://company.com/meetings/q1-summary".to_string()),
            meeting_id: Some("q1_summary_2024".to_string()),
        };

        let serialized = serde_json::to_string(&complex_minute).unwrap();
        let deserialized: Minute = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.title.as_ref().unwrap().contains("季度总结"));
        assert!(deserialized.title.as_ref().unwrap().contains("Q1 2024"));
        assert_eq!(
            deserialized.creator.as_ref().unwrap().name,
            Some("王经理".to_string())
        );

        // Test time parsing would work
        let start = deserialized.start_time.unwrap();
        let end = deserialized.end_time.unwrap();
        assert!(start < end); // Start time should be before end time
    }
}
