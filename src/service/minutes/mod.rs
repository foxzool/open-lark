//! 妙记（Minutes）服务
//!
//! 提供飞书妙记的完整功能集，支持会议记录、语音转写、智能摘要、
//! 内容管理等智能会议服务能力。是会议效率提升和知识管理的核心工具。
//!
//! # 核心功能
//!
//! ## 会议记录
//! - 📝 自动会议记录生成
//! - 🎤 实时语音转写
//! - 📊 会议内容结构化
//! - 🔍 会议记录搜索和检索
//! - 📋 会议纪要模板管理
//!
//! ## 智能转写
//! - 🗣️ 多语言语音识别
//! - 👥 说话人识别和分离
//! - 📝 文本智能校正
//! - ⏱️ 时间戳精确标记
//! - 🎯 关键词提取和标注
//!
//! ## 智能摘要
//! - 🤖 AI自动摘要生成
//! - 📊 重点内容提取
//! - 🎯 行动项识别
//! - 📅 待办事项整理
//! - 💡 智能建议和洞察
//!
//! ## 内容管理
//! - 📁 妙记文档分类管理
//! - 🔗 会议关联和引用
//! - 👥 协作编辑和评论
//! - 📊 访问权限控制
//! - 📈 使用统计和分析
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取妙记服务
//! let minutes = &client.minutes;
//!
//! // 获取妙记列表
//! // let list_request = ListMinutesRequest::builder()
//! //     .page_size(20)
//! //     .build();
//! // let minutes_list = minutes.v1.minute.list(list_request, None).await?;
//!
//! // 获取妙记详情
//! // let detail_request = GetMinuteRequest::builder()
//! //     .minute_token("minute_123")
//! //     .build();
//! // let minute_detail = minutes.v1.minute.get(detail_request, None).await?;
//!
//! // 获取转写内容
//! // let transcript_request = GetTranscriptRequest::builder()
//! //     .minute_token("minute_123")
//! //     .build();
//! // let transcript = minutes.v1.transcript.get(transcript_request, None).await?;
//!
//! // 获取统计信息
//! // let stats_request = GetStatisticsRequest::builder()
//! //     .minute_token("minute_123")
//! //     .build();
//! // let stats = minutes.v1.statistics.get(stats_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v1版本，提供基础的妙记功能：
//! - 妙记文档管理
//! - 转写内容获取
//! - 统计信息查询
//! - 权限控制和分享
//!
//! # 妙记特性
//!
//! - 🤖 AI驱动的智能转写
//! - 🎯 精准的内容识别
//! - 📊 丰富的数据分析
//! - 🔐 安全的权限控制
//! - 📱 多平台同步支持
//!
//! # 智能化能力
//!
//! - 🧠 深度学习语音识别
//! - 💡 智能内容理解
//! - 🎯 自动化信息提取
//! - 📈 数据洞察和分析
//! - 🔄 持续学习和优化

use crate::core::config::Config;

/// 数据模型定义
pub mod models;
/// 妙记服务 v1 版本
pub mod v1;

use v1::V1;

/// 妙记服务
///
/// 智能会议服务的统一入口，提供会议记录、语音转写、
/// 智能摘要、内容管理等完整的智能会议服务能力。
///
/// # 服务架构
///
/// - **v1**: 妙记API v1版本，提供基础功能集
/// - **models**: 数据模型和结构定义
///
/// # 核心特性
///
/// - 📝 智能的会议记录功能
/// - 🗣️ 精准的语音转写技术
/// - 🤖 AI驱动的内容摘要
/// - 📊 完善的内容管理系统
/// - 🔐 安全的权限控制机制
///
/// # 适用场景
///
/// - 企业会议记录管理
/// - 培训内容转写整理
/// - 重要讨论内容留存
/// - 会议效率分析优化
/// - 知识管理和沉淀
///
/// # 最佳实践
///
/// - 合理设置转写质量
/// - 定期整理会议内容
/// - 保护会议隐私安全
/// - 充分利用AI摘要
/// - 建立知识管理流程
pub struct MinutesService {
    /// v1版本API服务
    pub v1: V1,
}

impl MinutesService {
    /// 创建新的妙记服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的妙记服务实例
    pub fn new(config: Config) -> Self {
        Self {
            v1: V1::new(config),
        }
    }
}
