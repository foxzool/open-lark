//! 妙记服务
//!
//! 提供妙记信息、音视频、文字记录和统计数据的统一管理接口。

use super::{
    GetMinuteMediaRequest, GetMinuteRequest, GetMinuteStatisticsRequest, GetMinuteTranscriptRequest,
};
use openlark_core::config::Config;

/// 妙记服务
pub struct MinutesService {
    config: Config,
}

impl MinutesService {
    /// 创建新的妙记服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取妙记信息请求构建器
    pub fn get_builder(&self, minute_token: impl Into<String>) -> GetMinuteRequest {
        GetMinuteRequest::new(self.config.clone()).minute_token(minute_token)
    }

    /// 获取妙记信息
    pub fn get(&self, minute_token: impl Into<String>) -> GetMinuteRequest {
        self.get_builder(minute_token)
    }

    /// 下载妙记音视频文件请求构建器
    pub fn media_builder(&self, minute_token: impl Into<String>) -> GetMinuteMediaRequest {
        GetMinuteMediaRequest::new(self.config.clone()).minute_token(minute_token)
    }

    /// 下载妙记音视频文件
    pub fn media(&self, minute_token: impl Into<String>) -> GetMinuteMediaRequest {
        self.media_builder(minute_token)
    }

    /// 导出妙记文字记录请求构建器
    pub fn transcript_builder(
        &self,
        minute_token: impl Into<String>,
    ) -> GetMinuteTranscriptRequest {
        GetMinuteTranscriptRequest::new(self.config.clone()).minute_token(minute_token)
    }

    /// 导出妙记文字记录
    pub fn transcript(&self, minute_token: impl Into<String>) -> GetMinuteTranscriptRequest {
        self.transcript_builder(minute_token)
    }

    /// 获取妙记统计数据请求构建器
    pub fn statistics_builder(
        &self,
        minute_token: impl Into<String>,
    ) -> GetMinuteStatisticsRequest {
        GetMinuteStatisticsRequest::new(self.config.clone()).minute_token(minute_token)
    }

    /// 获取妙记统计数据
    pub fn statistics(&self, minute_token: impl Into<String>) -> GetMinuteStatisticsRequest {
        self.statistics_builder(minute_token)
    }
}
