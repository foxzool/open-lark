//! Cloud Docs Comments服务模块
//!
//! 文档评论系统，提供文档评论、回复、点赞、管理等功能，
//! 支持富文本评论、@提及、附件等高级功能。

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

use openlark_core::config::Config;

/// 评论服务
///
/// 提供完整的文档评论功能，包括：
/// - 创建评论和回复
/// - 评论点赞和取消点赞
/// - 评论删除和管理
/// - 评论搜索和过滤
/// - 评论通知和提醒
#[derive(Clone, Debug)]
pub struct CommentsService {
    pub config: Config,
}

impl CommentsService {
    /// 创建Comments服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
