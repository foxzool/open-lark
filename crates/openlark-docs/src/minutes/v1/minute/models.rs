//! Minutes API 数据模型

use openlark_core::api::ApiResponseTrait;
use openlark_core::api::ResponseFormat;
use serde::{Deserialize, Serialize};

/// 妙记基础信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteInfo {
    /// 妙记标题
    pub title: Option<String>,
    /// 封面图片URL
    pub cover_url: Option<String>,
    /// 音频时长（秒）
    pub duration: Option<i64>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 拥有者ID
    pub owner_id: Option<String>,
    /// 妙记URL
    pub url: Option<String>,
}

impl ApiResponseTrait for MinuteInfo {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 妙记音视频文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteMediaInfo {
    /// 音频文件下载URL
    pub audio_url: Option<String>,
    /// 视频文件下载URL
    pub video_url: Option<String>,
    /// 文件大小（字节）
    pub file_size: Option<i64>,
    /// 文件格式
    pub format: Option<String>,
}

impl ApiResponseTrait for MinuteMediaInfo {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 妙记文字记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteTranscript {
    /// 完整文字记录
    pub content: Option<String>,
    /// 语言类型
    pub language: Option<String>,
    /// 文字记录格式
    pub format: Option<String>,
    /// 段落列表
    pub paragraphs: Option<Vec<TranscriptParagraph>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptParagraph {
    /// 段落内容
    pub content: String,
    /// 开始时间（毫秒）
    pub start_time: i64,
    /// 结束时间（毫秒）
    pub end_time: i64,
    /// 说话人
    pub speaker: Option<String>,
}

impl ApiResponseTrait for MinuteTranscript {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 妙记统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteStatistics {
    /// 访问次数（PV）
    pub pv: Option<i64>,
    /// 访问用户数（UV）
    pub uv: Option<i64>,
    /// 访问过的用户ID列表
    pub user_ids: Option<Vec<String>>,
    /// 访问时间戳列表
    pub user_timestamps: Option<Vec<i64>>,
}

impl ApiResponseTrait for MinuteStatistics {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
