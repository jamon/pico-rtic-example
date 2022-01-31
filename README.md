# `pico-rtic-example`

> Example project for Raspberry Pi PICO that uses [rp-rs/rp2040-hal/rp-pico](https://github.com/rp-rs/rp-hal) [rtic](https://github.com/rtic-rs/cortex-m-rtic) and [rp2040-monotonic](https://github.com/korken89/rp2040-monotonic) and [defmt](https://github.com/knurling-rs/defmt)

Based on [rp2040-project-template](https://github.com/rp-rs/rp2040-project-template), [cortex-m-quickstart](https://docs.rs/cortex-m-quickstart/0.3.1/cortex_m_quickstart/), and [rp-pico rtic example](https://github.com/rp-rs/rp-hal/blob/main/boards/rp-pico/examples/pico_rtic.rs).  See also [defmt-app-template](https://github.com/rtic-rs/defmt-app-template)

Other examples of pico + rtic, by kalkyl [here](https://github.com/kalkyl/rp-rtic)
Generic (but somewhat out of date as of now) RTIC examples [here](https://github.com/rtic-rs/rtic-examples)
Teensy has some good RTIC Examples [here](https://github.com/mciantyre/teensy4-rs/tree/master/examples)

## Hardware Setup
  - Program a Raspberry Pi PICO to be a CMSIS-DAP probe using [DapperMime](https://github.com/majbthrd/DapperMime)
    - Download from [DapperMime Releases](https://github.com/majbthrd/DapperMime/releases)
    - Install onto the PICO you'll use as the probe
  - Connect the probe PICO to the PICO you'll be programming
    - [digikey guide](https://www.digikey.com/en/maker/projects/raspberry-pi-pico-and-rp2040-cc-part-2-debugging-with-vs-code/470abc7efb07432b82c95f6f67f184c0) covers this well, though you don't need to connect the UART between the devices.
    - in short:

| Probe PICO | PICO to be programmed |
| ---------- | --------------------- |
| VSYS       | VSYS                  |
| GND        | GND                   |
| GP2        | SWCLK                 |
| GP3        | SWDIO                 |

## Software Setup

- Rust
  - Install [rustup](https://rustup.rs/)
  - `rustup self update` - update rustup
  - `rustup update stable` - update rust
  - `rustup target add thumbv6m-none-eabi` - add cortex m0 compilation target for cross-compiling

- Rust Tools
  - `cargo install flip-link` - linker to enable [flip-link](https://github.com/knurling-rs/flip-link)
  - `cargo install probe-run` - run using probe (default, configured runner in .cargo/config.toml)
  - `cargo install elf2uf2-rs --locked` - build uf2 files (configure runner in .cargo/config.toml)
  - probe-rs (not currently working; don't use)
    - `cargo install --force --git https://github.com/probe-rs/probe-rs probe-rs-debugger`


- VS Code
  - Install [VS Code](https://code.visualstudio.com/)
  - Extensions:
    - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
    - [PIO ASM Syntax Highlighting](https://marketplace.visualstudio.com/items?itemName=chris-hock.pioasm)

- Rust helper commands (only used to follow cortex_m_quickstart, used to build this example; not needed)
  - `cargo install cargo-edit`
  - `cargo install cargo-generate`

## Running

`cargo run`

That's about it!

## Helpful Documentation

High-level documentation:
  - [rust book](https://doc.rust-lang.org/book/)
  - [rust embedded book](https://rust-embedded.github.io/book)
  - [rtic by example book](https://rtic.rs/1.0/book/en/preface.html)

Setup Guide for rp-rs (covers getting rust running on the pi pico or similar boards, including probe-run, elf2uf2-rs, etc)
  - [rp-rs README.md](https://github.com/rp-rs/rp-hal)

Other useful information:
  - [Writing embedded drivers in rust](https://hboeving.dev/blog/rust-2c-driver-p1/)
  - [Embedonomicon](https://docs.rust-embedded.org/embedonomicon/preface.html)
  - [Embedonomicon - Concurrency](https://japaric.github.io/embedonomicon/concurrency.html)
  - [embedded-hal Documentation](https://docs.rs/embedded-hal/latest/embedded_hal/)
  - [nb (non-blocking abstraction used by embedded-hal)](https://docs.rs/nb/latest/nb/)

# TODO

- Update .vscode/* for rp-pico
- get breakpoints/debugging working

# License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
