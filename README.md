# Rust Web Starter

Basic Rust Web Server template based on `rocket-rs`. 

# Dependencies

We need `rust 1.50+` or above *nightly* installed. They all have the
improvements we need for building things fast and known compatibility.

It can be frustrating sometimes to see builds take a long time. **LTO**
is the biggest impediment to getting a fast build time, irrespective of 
how fast the linkers and processors are. Currently, has been disabled
in `release` builds until we get a speedup for it.

Rust Configuration in your `.bashrc` and `.zshrc`

```bash
export CARGO_HOME=$HOME/.cargo
PATH=$PATH:$CARGO_HOME/bin
```

I highly recommend setting up *sccache*.

```bash
which sccache 2>&1 >/dev/null && export RUSTC_WRAPPER=sccache
```

This improves cold boot performance on different projects which
use the same libraries.

# Building for Local
```bash
cargo build
```

# Building for Release

```bash
cargo build --release
```

# Notes on Compilation

# Running

```bash
set RUST_LOG=info
set ROCKET_CLI_COLORS=on
cargo run
```
