fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fds = protox::compile(["proto/bevoid.proto"], ["proto"])?;
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_fds(fds)?;
    Ok(())
}
