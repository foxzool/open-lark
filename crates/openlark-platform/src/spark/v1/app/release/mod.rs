//! 妙搭应用发布记录。

/// 回写发布节点状态。
pub mod update;

pub use update::{
    SparkReleaseNodeAction, SparkReleaseNodeResult, UpdateSparkAppReleaseBody,
    UpdateSparkAppReleaseRequest, UpdateSparkAppReleaseResponse,
};
