fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/bin")
        .compile(&["proto/helloworld.proto"], &["../proto"])?;
    Ok(())
}
