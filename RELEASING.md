# Releasing

## Version Structure

This is a Cargo workspace with two crates that share the same version:

- `blaeck` — the main library
- `blaeck-macros` — proc-macro companion

The version is set once in the root `Cargo.toml` under `[workspace.package]`:

```toml
[workspace.package]
version = "0.3.0"
```

Both crates inherit this via `version.workspace = true`.

**Important:** `blaeck` has a path + version dependency on `blaeck-macros`. When bumping the workspace version, you must also update this dependency in `blaeck/Cargo.toml`:

```toml
blaeck-macros = { path = "../blaeck-macros", version = "0.3.0" }
```

## Release Checklist

1. **Update version** in `Cargo.toml` (workspace-level):

   ```toml
   [workspace.package]
   version = "X.Y.Z"
   ```

2. **Update `blaeck-macros` dependency version** in `blaeck/Cargo.toml` to match:

   ```toml
   blaeck-macros = { path = "../blaeck-macros", version = "X.Y.Z" }
   ```

3. **Update the install snippet** in `README.md`:

   ```toml
   blaeck = "X.Y"
   ```

4. **Update `CHANGELOG.md`** with the new version and release notes.

5. **Run CI checks locally**:

   ```bash
   cargo fmt --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace --all-features
   ```

6. **Commit and tag**:

   ```bash
   git add -A
   git commit -m "Bump version to X.Y.Z"
   git tag vX.Y.Z
   git push origin main --tags
   ```

7. **Publish to crates.io** (macros first, since `blaeck` depends on it):

   ```bash
   cargo publish -p blaeck-macros
   cargo publish -p blaeck
   ```

## What Happens Automatically

Pushing a `v*` tag triggers the [Release workflow](.github/workflows/release.yml), which:

1. Builds `example_viewer` binaries for three targets:
   - `x86_64-unknown-linux-gnu`
   - `x86_64-apple-darwin`
   - `aarch64-apple-darwin`
2. Creates a GitHub Release with those binaries attached and auto-generated release notes.

The release workflow can also be triggered manually via `workflow_dispatch`.

## CI

Every push to `main` and every PR runs the [CI workflow](.github/workflows/ci.yml):

- `cargo test --workspace --all-features`
- `cargo clippy` with `-D warnings`
- `cargo fmt --check`
