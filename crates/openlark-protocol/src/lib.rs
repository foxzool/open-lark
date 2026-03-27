//! OpenLark WebSocket 协议的 protobuf 类型定义。
//!
//! 该 crate 主要暴露由构建脚本生成的 `pbbp2` 模块，供 WebSocket 客户端复用。

#[allow(missing_docs)]
pub mod pbbp2 {
    include!(concat!(env!("OUT_DIR"), "/pbbp2.rs"));
}
