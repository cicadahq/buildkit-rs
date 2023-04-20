use std::{io::Result, path::PathBuf};

const BUILDKIT_DIR: &str = "vendor/github.com/moby/buildkit";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let includes = ["vendor", "vendor/github.com/tonistiigi/fsutil/types"];

    let protos = [
        format!("{BUILDKIT_DIR}/frontend/gateway/pb/gateway.proto"),
        format!("{BUILDKIT_DIR}/api/services/control/control.proto"),
    ];

    tonic_build::configure()
        .build_server(false)
        .compile(&protos, &includes)?;

    for (session_type, pkg_name) in [
        ("auth", "moby.filesync.v1"),
        ("filesync", "moby.filesync.v1"),
        ("secrets", "moby.buildkit.secrets.v1"),
    ] {
        let protos = [format!(
            "{BUILDKIT_DIR}/session/{session_type}/{session_type}.proto"
        )];

        tonic_build::configure()
            .build_client(false)
            .compile(&protos, &includes)?;

        // Move the generated files to a new location
        let src = out_dir.join(format!("{pkg_name}.rs"));
        let dest = out_dir.join(format!("{session_type}.rs"));

        std::fs::rename(src, dest).unwrap();
    }

    Ok(())
}
