# Load Balancer

- pingora: [quickstart](https://github.com/cloudflare/pingora/blob/main/docs/quick_start.md)
- code: [load_balancer.rs](https://github.com/cloudflare/pingora/blob/main/pingora-proxy/examples/load_balancer.rs)

## Init

```bash
git clone https://github.com/cloudflare/pingora.git
cargo new load_balancer
```

### load_balancer/Cargo.toml

```toml
[dependencies]
async-trait = "0.1.80"
pingora = { path = "../pingora/pingora" }
pingora-core = { path = "../pingora/pingora-core" }
pingora-proxy = { path = "../pingora/pingora-proxy" }
pingora-load-balancing = { path = "../pingora/pingora-load-balancing" }
```

## Run

### Run the server

```bash
cargo run
```

### Run the client

```bash
curl 127.0.0.1:6188 -svo /dev/null
```

