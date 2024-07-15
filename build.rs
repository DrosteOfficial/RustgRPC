use std::error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/calculator.proto")?;
    tonic_build::compile_protos("proto/pow.proto")?;
    Ok(())
}
