fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &[
                "protocol/core/Tron.proto",
                "protocol/core/Discover.proto",
                "protocol/core/TronInventoryItems.proto",
            ],
            &["protocol"],
        )?;
    Ok(())
}
