use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let descriptor_path = out_dir.join("hakoniwa-code-runner.bin");

    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    tonic_build::configure()
        .file_descriptor_set_path(descriptor_path)
        .compile_with_config(
            config,
            &["protos/languages.proto", "protos/runs.proto"],
            &["protos/"],
        )?;
    Ok(())
}
