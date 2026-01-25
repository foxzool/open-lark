/// 妙记转写文本导出（minute.transcript）相关接口
pub mod get;

// get 模块显式导出
pub use get::{
    GetMinuteTranscriptRequest,
    execute,
    execute_with_options,
    file_format,
    minute_token,
    need_speaker,
    need_timestamp,
    new,
};
