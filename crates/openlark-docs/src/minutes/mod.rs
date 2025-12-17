// 重新导出子模块
pub mod minutes;
pub use minutes::*;

use openlark_core::config::Config;
use crate::minutes::v1::minute::get::GetMinuteBuilder;
use crate::minutes::v1::minute::media::get::GetMinuteMediaBuilder;
use crate::minutes::v1::minute::statistics::get::GetMinuteStatisticsBuilder;
use crate::minutes::v1::minute::transcript::get::GetMinuteTranscriptBuilder;

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
    /// doc: https://open.feishu.cn/document/server-docs/minutes-v1/minute/get
    pub fn get_minute(&self, minute_token: impl ToString) -> GetMinuteBuilder {
        GetMinuteBuilder::new(minute_token)
    }

    /// 下载妙记音视频文件
    ///
    /// doc: https://open.feishu.cn/document/minutes-v1/minute-media/get
    pub fn get_minute_media(&self, minute_token: impl ToString) -> GetMinuteMediaBuilder {
        GetMinuteMediaBuilder::new(minute_token)
    }

    /// 获取妙记统计数据
    ///
    /// doc: https://open.feishu.cn/document/server-docs/minutes-v1/minute-statistics/get
    pub fn get_minute_statistics(&self, minute_token: impl ToString) -> GetMinuteStatisticsBuilder {
        GetMinuteStatisticsBuilder::new(minute_token)
    }

    /// 导出妙记文字记录
    ///
    /// doc: https://open.feishu.cn/document/minutes-v1/minute-transcript/get
    pub fn get_minute_transcript(&self, minute_token: impl ToString) -> GetMinuteTranscriptBuilder {
        GetMinuteTranscriptBuilder::new(minute_token)
    }
}
