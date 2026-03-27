//! 构建脚本：为 `openlark-protocol` 生成 protobuf Rust 代码。

use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["protos/pbbp2.proto"], &["protos/"])?;

    Ok(())
}
