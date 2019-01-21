teranex-ctl
===========

Command line remote control tool for [Black Magic Teranex video converter](https://www.blackmagicdesign.com/products/teranex).

# Build
- Install Rust: `curl https://sh.rustup.rs -sSf | sh`
- `cargo build --release`
- Binary is in `./target/release/teranex-ctl`
- You can also run from sources using `cargo run`

## Cross-Compile for Zynq UltraScale+
- `rustup target add aarch64-unknown-linux-gnu`
- Create a `~/.cargo/config` file with this content:
```
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```
- `cargo build --target=aarch64-unknown-linux-gnu --release`
- Copy binary `target/aarch64-unknown-linux-gnu/release/teranex-ctl` to the board

# Usage
See `teranex-ctl --help` and `teranex-ctl set-video-mode --help`.

You can define `RUST_LOG=teranex_ctl=debug` or `RUST_LOG=teranex_ctl=trace` to enable debug or trace output.

On Linux you can use `avahi-browse _bmd-teranex._tcp --resolve` to try discovering the Teranex IP.
