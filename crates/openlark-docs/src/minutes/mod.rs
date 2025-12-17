// 重新导出子模块
pub mod minutes;
pub use minutes::*;

use crate::minutes::v1::minute::get::GetMinuteRequest;
use crate::minutes::v1::minute::media::get::GetMinuteMediaRequest;
use crate::minutes::v1::minute::statistics::get::GetMinuteStatisticsRequest;
use crate::minutes::v1::minute::transcript::get::GetMinuteTranscriptRequest;
use openlark_core::config::Config;

#[derive(Debug, Clone)]
pub struct MinutesService<'a> {
    config: &'a Config,
}

impl<'a> MinutesService<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// 获取妙记信息
    ///
    /// docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute/get
    pub fn get_minute(&self, minute_token: impl Into<String>) -> GetMinuteRequest {
        GetMinuteRequest::new(self.config.clone()).minute_token(minute_token)
    }

    /// 下载妙记音视频文件
    ///
    /// docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-media/get
    pub fn get_minute_media(&self, minute_token: impl Into<String>) -> GetMinuteMediaRequest {
        GetMinuteMediaRequest::new(self.config.clone()).minute_token(minute_token)
    }

    /// 获取妙记统计数据
    ///
    /// docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-statistics/get
    pub fn get_minute_statistics(
        &self,
        minute_token: impl Into<String>,
    ) -> GetMinuteStatisticsRequest {
        GetMinuteStatisticsRequest::new(self.config.clone()).minute_token(minute_token)
    }

    /// 导出妙记文字记录
    ///
    /// docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-transcript/get
    pub fn get_minute_transcript(
        &self,
        minute_token: impl Into<String>,
    ) -> GetMinuteTranscriptRequest {
        GetMinuteTranscriptRequest::new(self.config.clone()).minute_token(minute_token)
    }
}
