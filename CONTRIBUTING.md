# Contributing to MRT

## Setup

Install Git and Rust. The repository's `rust-toolchain.toml` installs the
supported compiler, formatter, and linter automatically.

```console
git clone https://github.com/chaliy/mrt.git
cd mrt
cargo build --locked
```

## Development workflow

Create a branch from current `main` and keep each change focused. Add tests for
new behavior and update the README when user-facing behavior changes.

Before opening a pull request, run:

```console
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
cargo doc --locked --no-deps --all-features
cargo deny check
cargo audit
```

Commit and pull-request titles use Conventional Commits, for example:

```text
feat(cli): filter packages by path
fix(runner): preserve command output newlines
docs: clarify manifest discovery
```

Pull requests should explain the outcome, motivation, risk, and validation.
CI must pass before a change is merged.

## Releases

1. Update the version in `Cargo.toml` and refresh `Cargo.lock`.
2. Move the relevant entries from `Unreleased` into a versioned changelog
   section such as `## [1.1.0] - 2026-07-16`.
3. Merge the release preparation pull request.
4. Create and push an annotated `vMAJOR.MINOR.PATCH` tag from `main`.

The release workflow verifies that the tag matches `Cargo.toml`, builds
precompiled binaries, generates checksums, and publishes a GitHub Release.
The `mrt` package name is already owned by an unrelated crate on crates.io, so
MRT is distributed through GitHub rather than crates.io.

## License

By contributing, you agree that your contributions are licensed under the MIT
License.
