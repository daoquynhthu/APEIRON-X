fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use vendored protoc to avoid system dependency
    let protoc = protoc_bin_vendored::protoc_bin_path()
        .expect("failed to fetch vendored protoc");
    std::env::set_var("PROTOC", protoc);

    tonic_build::configure()
        .compile(&["proto/runtime.proto"], &["proto"])?;
    Ok(())
}

