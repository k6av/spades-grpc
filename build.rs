fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .type_attribute(".", "#[derive(serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .compile_protos(
            &["proto/maps_rpc.proto", "proto/maps.proto", "proto/modes_rpc.proto", "proto/modes.proto", "proto/servers_rpc.proto", "proto/servers.proto"],
            &["proto"],
        )?;
    Ok(())
}
