# RustOS

> [!WARNING]
> Project meant as a learning experience, therefore WIP and use at own risk!

Building on the guide from [Philipp Oppermann](https://os.phil-opp.com/) I write my own little
kernel for experimental purposes.

## Requirements

- Cargo nightly toolchain: `rustup override set nightly` and some components
  - `rustup component add rust-src`
  - `rustup component add llvm-tools-preview`
- `QEMU`, to run it virtually.
