/// Sheets v2 增强图片写入服务
///
/// 提供飞书电子表格v2版本的高级图片写入功能，包括：
/// - 向电子表格中插入多种格式的图片（PNG、JPEG、GIF等）
/// - 精确的图片位置控制（像素级定位和Excel风格定位）
/// - 灵活的图片尺寸设置（原始尺寸、自定义尺寸、比例缩放）
/// - 批量图片插入和管理功能
/// - 图片压缩和优化选项
/// - 企业级错误处理和数据验证
/// - 构建器模式支持，提供流畅API设计
use serde_json::Value;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
