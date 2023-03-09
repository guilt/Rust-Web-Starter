# Rust Web Starter

Basic Rust Web Server template based on `rocket-rs`. 

# Dependencies

We need `rust 1.67+` or above *stable* installed. They all have the
improvements we need for building things fast and known compatibility
for Rocket v0.5rc3+.

Rust Configuration in your `.bashrc` and `.zshrc`

```bash
export CARGO_HOME=$HOME/.cargo
PATH=$PATH:$CARGO_HOME/bin
```

To speed up builds, it is suggested you use the **mold** linker, currently
supported on *Linux* and *Darwin*. You can set this up in your Cargo
Configuration in `~/.cargo/config.toml`.

You could set up **sccache** as well but there are some crates where
it does not make a real difference.

```toml
[build]
# rustc-wrapper = "sccache"

[target.x86_64-apple-darwin]
#linker = "/usr/bin/clang"
rustflags = ["-C", "link-arg=--ld-path=/path/to/ld64.mold"]

[target.aarch64-apple-darwin]
#linker = "/usr/bin/clang"
rustflags = ["-C", "link-arg=--ld-path=/path/to/ld64.mold"]

[target.i686-pc-windows-msvc]
#rustflags = []

[target.x86_64-pc-windows-msvc]
#rustflags = []

[target.i686-unknown-linux-gnu]
#linker = "/usr/bin/gcc"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[target.x86_64-unknown-linux-gnu]
#linker = "/usr/bin/gcc"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[target.aarch64-unknown-linux-gnu]
#linker = "/usr/bin/gcc"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```
# Notes on Compilation

## Building for Local
```bash
cargo build
```

## Building for Release

```bash
cargo build --release
```

# Notes on Running

```bash
RUST_LOG=info ROCKET_CLI_COLORS=true cargo run --release
```
