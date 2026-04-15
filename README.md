# DairyShop

A simple Rust "Hello, World!" application serving as a project scaffolding.

## Building

Requires the Rust toolchain (edition 2024). To build the binary:

```bash
cargo build --release
```

## Running

Run without arguments for the default greeting:

```bash
cargo run
```

Output:

```
Hello, World!
```

Or pass a name as an argument:

```bash
cargo run -- Alice
```

Output:

```
Hello, Alice!
```

## Project Structure

```
src/
  main.rs    - CLI entry point
  lib.rs     - Library root with a convenience `hello()` function
  models.rs  - `Greeting` struct and message formatting
  parser.rs  - Input parsing utilities
tests/
  models.rs  - Unit tests for the Greeting model
  parser.rs  - Unit and CLI integration tests for parsing
```

## Testing

Run unit and integration tests with:

```bash
cargo test
```

GitHub Actions will also run the tests on every pull request.

## License

MIT
