// Build script runs before package is built.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
     * Build protobufs
     */
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // Enable optional proto fields. 
        .compile(
            &["../../proto/scraper.proto"],
            &["../../proto"],
        )?;
    Ok(())
}
