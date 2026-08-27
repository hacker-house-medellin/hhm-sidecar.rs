# hhm-sidecar.rs

k8s sidecar for Hacker House Medellin (health, future NATS bridge).

Inherits [`ores-otel-sidecar`](https://github.com/ores-otel/ores-otel-sidecar.rs).
Bind with `HHM_SIDECAR_BIND` (default `127.0.0.1:9090`).

```sh
cargo run --bin hhm-sidecar
```
