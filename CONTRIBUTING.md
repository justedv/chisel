# Contributing to Solder

Thanks for your interest in contributing to Solder.

## Getting Started

1. Fork the repo
2. Clone your fork
3. Create a feature branch
4. Make your changes
5. Run tests: `cargo test --all`
6. Run clippy: `cargo clippy --all -- -D warnings`
7. Submit a PR

## Guidelines

- Keep PRs focused and small
- Add tests for new features
- Follow existing code style
- Update docs if behavior changes
- No unsafe code without justification

## Architecture

- `solder-derive` — proc-macro crate, handles `#[derive(Solder)]` expansion
- `solder-core` — runtime traits, zero-copy helpers, error types
- `solder` (root) — re-exports both crates

## Testing

```bash
cargo test --all                    # all tests
cargo test -p solder-derive         # macro tests only
cargo test -p solder-core           # core tests only
```

## License

By contributing, you agree your contributions will be licensed under MIT.
