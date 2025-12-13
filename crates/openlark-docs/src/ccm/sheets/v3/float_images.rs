/// 浮动图片管理服务
///
/// 提供电子表格浮动图片的完整管理功能，包括：
/// - 创建和插入浮动图片
/// - 更新图片位置和大小
/// - 查询和获取图片信息
/// - 删除浮动图片
/// - 图片格式和尺寸管理
use serde::{Deserialize, Serialize};

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
