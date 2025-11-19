use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["protos/pbbp2.proto"], &["protos/"])?;

    Ok(())
}
