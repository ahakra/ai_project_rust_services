fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/generated")
        .compile_protos(
            &["../shared_proto/service_registry.proto"],
            &["../shared_proto"],
        )?;
    Ok(())
}
