use std::{fs};

// Build script runs before package is built.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local_proto_dir = "./proto";
    let local_proto_path = "./proto/scraper.proto";
    
    
    let src_proto_path = "../../proto/scraper.proto";
    fs::create_dir_all(local_proto_dir)?;
    fs::copy(src_proto_path, local_proto_path)?;

    /*
     * Build protobufs
     */
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // Enable optional proto fields.
        .compile(&[local_proto_path], &[local_proto_dir])?;

    Ok(())
}
