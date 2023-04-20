use std::{io::Result, path::PathBuf};

const VENDOR_DIR: &str = "vendor";
const BUILDKIT_DIR: &str = "vendor/github.com/moby/buildkit";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let protos = [
        format!("{BUILDKIT_DIR}/frontend/gateway/pb/gateway.proto"),
        format!("{BUILDKIT_DIR}/api/services/control/control.proto"),
    ];
    let includes = [VENDOR_DIR.into(), format!("{BUILDKIT_DIR}/vendor")];

    tonic_build::configure()
        .build_server(false)
        .compile(&protos, &includes)?;

    let protos = [
        format!("{BUILDKIT_DIR}/session/secrets/secrets.proto"),
        format!("{BUILDKIT_DIR}/session/auth/auth.proto"),
    ];

    tonic_build::configure()
        .build_client(false)
        .compile(&protos, &includes)?;

    Ok(())
}
