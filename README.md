# Rust Backend with Axum

This is a minimal Rust backend project using the [Axum](https://github.com/tokio-rs/axum) web framework. It demonstrates how to set up a simple HTTP server with a single endpoint.

## Features
- Built with [Axum](https://github.com/tokio-rs/axum)
- Async server powered by [Tokio](https://tokio.rs/)
- JSON response endpoint

## Endpoint

| Method | Path     | Description           |
|--------|----------|-----------------------|
| GET    | /hello   | Returns a hello world JSON message |

**Example response:**
```json
{
  "msg": "hello, world!"
}
```

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later)

### Running the Server

1. Clone the repository or copy the project files.
2. Install dependencies and run the server:

```bash
cargo run
```

3. The server will start on [http://127.0.0.1:3000](http://127.0.0.1:3000)

4. Test the endpoint:

```bash
curl http://127.0.0.1:3000/hello
```

## Project Structure

- `src/main.rs` — Main application code, router, and handler
- `Cargo.toml` — Project dependencies and metadata

## Dependencies
- [axum](https://crates.io/crates/axum)
- [serde](https://crates.io/crates/serde)
- [tokio](https://crates.io/crates/tokio)

## License

MIT
