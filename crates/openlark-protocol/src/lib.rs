//! OpenLark WebSocket 协议的 protobuf 类型定义。
//!
//! 该 crate 主要暴露由构建脚本生成的 `pbbp2` 模块，供 WebSocket 客户端复用。

/// `pbbp2` 为构建期生成的 protobuf 模块。
///
/// 由于内容完全来自生成代码，当前仅在模块边界保留一个
/// `#[allow(missing_docs)]` 例外，避免为生成产物重复补文档。
#[allow(missing_docs)]
pub mod pbbp2 {
    include!(concat!(env!("OUT_DIR"), "/pbbp2.rs"));
}
