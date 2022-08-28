fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./protos/shared/language.proto")?;
    tonic_build::compile_protos("./protos/languages.proto")?;
    Ok(())
}
