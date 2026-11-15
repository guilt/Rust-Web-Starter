# Rust Web Starter

A minimal web server starter built on [Rocket](https://rocket.rs), tuned for
**fast local builds** and modern Rust.

```text
GET /              -> Hello, world! Hit /hello/Potato
GET /hello/<name>  -> Hello <name>!
```

## Requirements

- **Rust stable** (edition 2024, so 1.85+). `rust-toolchain.toml` installs the
  right toolchain and components automatically via rustup.
- **mold** on Linux, for fast linking. Install it with your package manager:

  ```bash
  sudo apt install mold      # Debian/Ubuntu
  sudo dnf install mold      # Fedora
  sudo pacman -S mold        # Arch
  brew install mold          # macOS (see the commented block in .cargo/config.toml)
  ```

  No mold? Delete the matching `[target.*]` section in `.cargo/config.toml` and
  the system linker is used instead. Windows is unaffected.

- **sccache**, the shared compilation cache enabled by default in
  `.cargo/config.toml`:

  ```bash
  cargo install sccache
  # or download a prebuilt binary:
  # https://github.com/mozilla/sccache/releases
  ```

  No sccache? Delete the `[build]` block in `.cargo/config.toml`; builds fall
  back to plain rustc.

## Quick start

```bash
cargo run --release
```

Then open <http://127.0.0.1:8000/> or:

```bash
curl http://127.0.0.1:8000/hello/Potato
```

Useful environment variables:

| Variable            | Default     | Description                     |
| ------------------- | ----------- | ------------------------------- |
| `ROCKET_PORT`       | `8000`      | Port to listen on               |
| `ROCKET_ADDRESS`    | `127.0.0.1` | Bind address                    |
| `ROCKET_LOG_LEVEL`  | `normal`    | `off`, `critical`, `normal`, `debug` |

Rocket installs its own logger, so no extra logging crate is needed.

## Build speed

Measured with a clean `target/` on an AMD Ryzen Threadripper 3970X (32C/64T),
Rust 1.100, mold:

| Build                    | Before | No sccache | sccache warm |
| ------------------------ | -----: | ---------: | -----------: |
| Clean debug              |  ~16s  |      ~11s  |         ~4s  |
| Clean release            |  ~17s  |      ~15s  |         ~7s  |
| Fresh copy of template   |      — |      ~11s  |         ~4s  |
| Incremental edit+rebuild |      — |     ~0.5s  |        ~0.5s |

The first sccache build is about 2s slower (cache writes). After that, warm
clean builds are 2–3× faster, and the cache (~215 MB for both profiles) is
shared across every project on the machine.

What makes it fast:

- **mold** for linking (config in `.cargo/config.toml`).
- **sccache** for compilation caching, enabled in `.cargo/config.toml`. This is
  what makes fresh clones, new projects from this template, and clean rebuilds
  fast.
- **`debug = 1`** in the dev profile: line tables only, so debug info is much
  smaller and linking is cheaper. Backtraces still show file/line numbers.
- **Incremental compilation** and 256 codegen units for dev builds.
- **No HTTP/2 by default.** Rocket's default `http2` feature pulls in `h2` and
  friends; enable it only when you need it (see below).
- **Thin LTO** for release builds: most of the runtime benefit of fat LTO, but
  much less build time.
- **`Cargo.lock` is committed**, so dependency resolution is reproducible and
  cached builds are not invalidated by surprise upgrades.
- **CI caching** via `Swatinem/rust-cache` + `mozilla-actions/sccache-action`
  (GitHub Actions cache backend), plus mold, in `.github/workflows/ci.yml`.

## Releases

Push a version tag to build and publish binaries:

```bash
git tag v0.1.0
git push origin v0.1.0
```

`.github/workflows/release.yml` cross-compiles the server and attaches the
binaries plus `checksums.txt` to the GitHub release. Binaries are named
`rust-web-starter-<os>-<arch>` (with `.exe` on Windows).

Core targets (always built):

| OS      | Architectures              |
| ------- | -------------------------- |
| Linux   | amd64, arm64 (static musl) |
| macOS   | amd64, arm64               |
| Windows | amd64 (MSVC)               |

Experimental targets (a failure is skipped, so it never blocks a release):

- Linux: armv7, 386, riscv64, ppc64le, s390x, loongarch64
- BSD: FreeBSD amd64/arm64/386, NetBSD amd64
- Solaris family: Solaris amd64, illumos amd64
- Windows: 386, arm64

DragonFly and OpenBSD are not included because Rust does not ship a prebuilt
`std` for them on stable.

`workflow_dispatch` runs the whole matrix without publishing, which is useful
for testing changes to the pipeline. Verify a downloaded release with:

```bash
sha256sum -c checksums.txt
```

## Project layout

```text
.cargo/config.toml       Fast linker and sccache configuration
.github/workflows/ci.yml      Format, clippy, build, test (with caching)
.github/workflows/release.yml Cross-platform binaries, checksums, releases
rust-toolchain.toml      Pinned stable toolchain + components
src/main.rs              Rocket launch and route mounting
src/route.rs             Handlers
Cargo.toml               Dependencies and build profiles
```

## Customizing

Enable more Rocket features when you need them:

```toml
[dependencies]
rocket = { version = "0.5.2", default-features = false, features = ["json", "http2", "tls"] }
```

sccache is on by default. To turn it off, delete the `[build] rustc-wrapper`
block in `.cargo/config.toml`. Its cache lives in `~/.cache/sccache` by default
(`SCCACHE_DIR` overrides it); clear it with:

```bash
sccache --stop-server && rm -rf ~/.cache/sccache
```

Want full debug info instead of line tables? Set `debug = 2` in
`[profile.dev]`.

## Checks

```bash
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
```
