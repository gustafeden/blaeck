# Contributing to Blaeck

Thanks for your interest in contributing to Blaeck!

## Development Setup

```bash
# Clone
git clone https://github.com/gustafeden/quill
cd quill

# Build
cargo build --all-features

# Test
cargo test --all

# Run an example
cargo run --example hello
```

## Before Submitting a PR

```bash
# Format
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features

# Test
cargo test --all
```

## Adding a New Component

1. Create the component file in `quill/src/components/`
2. Add the module to `quill/src/components/mod.rs`
3. Export types from `quill/src/lib.rs` and the prelude
4. If the component returns a Fragment for vertical rendering, add it to the renderer's component lists in `quill/src/renderer.rs`
5. Add tests
6. Create an example in `quill/examples/`

## Component Guidelines

- Props struct with builder pattern methods
- Default impl for props
- `Component` trait implementation
- Helper functions for common use cases
- Tests for props, rendering, and edge cases

## Code Style

- Use `#[must_use]` on builder methods
- Document public APIs
- Keep examples minimal but complete
- Prefer composition over inheritance

## Issues

- Bug reports: include minimal reproduction
- Feature requests: describe the use case
- Questions: check examples first

## License

Contributions are dual-licensed under MIT OR Apache-2.0.
