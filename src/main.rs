#![forbid(unsafe_code)]

#[path = "../generated/rust/env.rs"]
mod env;
#[path = "../generated/rust/runtime.rs"]
mod env_runtime;

use ores_otel_sidecar::{runtime, SidecarConfig, SidecarIdentity};

/// A configured but malformed listen address must fail startup rather than
/// silently falling back to another address or listener policy.
fn config_from_bind(bind: &str) -> std::io::Result<SidecarConfig> {
    SidecarConfig::from_bind(
        SidecarIdentity::new(env::SERVICE, env::BIND),
        bind,
        false,
    )
    .map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "HHM_SIDECAR_BIND must be a valid permitted socket address",
        )
    })
}

fn main() -> std::io::Result<()> {
    let values = env_runtime::load_from_os();
    let cfg = config_from_bind(&values.bind)?;
    runtime::run(&cfg);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_default_bind_is_valid() {
        assert!(config_from_bind(env::BIND_DEFAULT).is_ok());
    }

    #[test]
    fn malformed_bind_does_not_fall_back() {
        for bad in ["", "0.0.0.0:9090", "[::]:9090", "1.1.1.1:9090", "127.0.0.1"] {
            let error = config_from_bind(bad)
                .err()
                .expect("malformed bind must fail closed");
            assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
            if !bad.is_empty() {
                assert!(!error.to_string().contains(bad));
            }
        }
    }
}
