/// 妙记音视频（minute.media）相关接口
pub mod get;

// get 模块显式导出
pub use get::{
    GetMinuteMediaRequest,
    GetMinuteMediaResponse,
    execute,
    execute_with_options,
    minute_token,
    new,
};
