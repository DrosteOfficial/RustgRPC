fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/generated") // Specify the output directory here
        .compile(
            &[
                "proto/calculator.proto",
                "proto/pow.proto",
                "proto/user.proto",
                "proto/messages.proto",
                "userV2.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
