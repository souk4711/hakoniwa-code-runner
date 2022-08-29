fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("protos/languages.proto")?;
    tonic_build::compile_protos("protos/runs.proto")?;
    Ok(())
}
