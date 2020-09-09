# Rust Web Starter

Basic Rust Web Server template based on `actix-web`. 

# Dependencies

We need `rust 1.45+` or above installed. They all have the
improvements we need for building things fast.

It can be frustrating sometimes to see builds take a long time. **LTO**
is the biggest impediment to getting a fast build time, irrespective of 
how fast the linker and the processor are. Currently, has been disabled
in `release` builds until we get a speedup for it.

I highly recommend setting up *sccache* and enable it in your .bashrc

```bash
export CARGO_HOME=$HOME/.cargo
PATH=$PATH:$CARGO_HOME/bin
which sccache 2>&1 >/dev/null && export RUSTC_WRAPPER=sccache
```

This seriously improves cold boot performance on different projects which
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
cargo run
```
