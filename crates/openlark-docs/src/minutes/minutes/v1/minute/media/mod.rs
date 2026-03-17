/// 妙记音视频（minute.media）相关接口
pub mod get;

// 使用通配符导出所有子模块
// get 模块显式导出
pub use get::{
    GetMinuteMediaRequest,
    GetMinuteMediaResponse,
};
