fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/tonic/") // saves generated structures at this location
        .compile(
            &["src/tonic/proto/solvio.proto"], // proto entry point
            &["src/tonic/proto"], // specify the root location to search proto dependencies
        )
        .unwrap();

    Ok(())
}
