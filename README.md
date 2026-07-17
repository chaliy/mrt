# MRT — Monorepo Tool

MRT discovers packages in a polyglot monorepo and runs the same script across
them. It currently understands npm and Poetry package metadata and can execute
npm scripts or Make targets.

## Install

Install the latest development version with Cargo:

```console
cargo install --git https://github.com/chaliy/mrt.git --locked
```

Versioned releases also provide prebuilt binaries for Linux, macOS, and Windows
on the [GitHub Releases](https://github.com/chaliy/mrt/releases) page.

## Use

MRT looks for packages under `packages/*` and `apps/*`. Pass a manifest path to
set the monorepo root:

```console
mrt --manifest ./mrt.yml list
mrt --manifest ./mrt.yml list --all
mrt --manifest ./mrt.yml run build
mrt --manifest ./mrt.yml --output json list
```

Without `--manifest`, MRT uses the current directory as the project root.
`list --all` includes directories whose package metadata could not be read or
whose package type could not be detected.

### Shell completion

Generate completion scripts for Bash, Elvish, Fish, PowerShell, or Zsh:

```console
mrt completion zsh > _mrt
```

## Supported packages

| Package type | Detection | Script runner |
| --- | --- | --- |
| npm | `package.json` | `make <script>`, then `npm run <script>` |
| Poetry | `pyproject.toml` with `[tool.poetry]` | `make <script>` |

When both a Make target and a package-manager script exist, MRT uses the Make
target first.

## Develop

The repository pins its Rust toolchain. Run the same core checks as CI with:

```console
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
cargo deny check
cargo audit
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the complete workflow.

## Roadmap

- Read package globs and configuration from the manifest
- Add native Cargo package support
- Filter packages by name, path, type, or changed files
- Model package dependencies and execution order
- Coordinate versioning and releases across packages

## License

MRT is available under the [MIT License](LICENSE).
