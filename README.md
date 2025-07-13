# Rust Backend with Axum

This is a modular Rust backend project using the [Axum](https://github.com/tokio-rs/axum) web framework. It demonstrates a professional folder structure with separate modules for user, video, and music endpoints, each returning fake sample data.

## Features
- Built with [Axum](https://github.com/tokio-rs/axum)
- Async server powered by [Tokio](https://tokio.rs/)
- Modular structure: user, video, music endpoints
- JSON response endpoints with sample data

## Endpoints

| Method | Path                  | Description                        |
|--------|-----------------------|------------------------------------|
| GET    | /user/:username       | Get a user by username             |
| GET    | /video/:uid           | Get a video by uid                 |
| GET    | /music/:uid           | Get a music by uid                 |

### Example responses

**GET /user/johndoe**
```json
{
  "username": "johndoe",
  "name": "John Doe",
  "email": "johndoe@example.com"
}
```

**GET /video/abc123**
```json
{
  "uid": "abc123",
  "title": "Sample Video",
  "duration": 3600
}
```

**GET /music/xyz789**
```json
{
  "uid": "xyz789",
  "title": "Sample Song",
  "artist": "Sample Artist"
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

4. Test the endpoints:

```bash
curl http://127.0.0.1:3000/user/johndoe
curl http://127.0.0.1:3000/video/abc123
curl http://127.0.0.1:3000/music/xyz789
```

### Automatic Recompilation with cargo-watch

For a smoother development experience, you can use [`cargo-watch`](https://crates.io/crates/cargo-watch) to automatically recompile and restart your backend whenever you change files.

#### Install cargo-watch

```bash
cargo install cargo-watch
```

#### Run the server with cargo-watch

```bash
cargo watch -x run
```

This will watch your project files and automatically restart the server when you make changes.

## Project Structure

- `src/main.rs`