fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/generated")
        .compile_protos(
            &["../proto/service_registry.proto"],
            &["../proto"],
        )?;
    Ok(())
}