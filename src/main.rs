#![forbid(unsafe_code)]

use ores_otel_sidecar::{runtime, SidecarConfig, SidecarIdentity};

fn main() {
    let cfg = SidecarConfig::from_env(SidecarIdentity::new(
        "hhm-sidecar",
        "HHM_SIDECAR_BIND",
    ));
    runtime::run(&cfg);
}
