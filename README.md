# Hugo4OS

Me just experimenting with OS development in rust

## Build dependencies

Cargo should automatically detect all necessary components by checking `rust-toolchain.toml`, but you still need `just` to use the build system:

```bash
cargo install just
```

> If Cargo didn't correctly install all components automatically, you can install them manually:
> 
> - Minimum supported rustc: `1.61.0-nightly` (I use `nightly-2022-04-02` as configured in `rust-toolchain.toml`).
> - LLVM tools also need to be installed and available in PATH: `rustup component add llvm-tools-preview`
> - `rust-src` is also needed: `rustup component add rust-src`


## Usage

> _To enable verbose output in the terminal, append `--features verbose` to any of these commands._

Building and running with the resulting image in [Qemu](https://www.qemu.org/):

```bash
just run
```

## Showcase

A screenshot (taken in QEMU) of current rendering capabilities in Hugo4OS:

![wowie](./.github/showcase/screenshot-2022-04-03_19-03-52.png)

A video of Hugo4OS running on real hardware (turn the sound on!):

https://user-images.githubusercontent.com/48245742/161477623-4e3c07cb-3731-45eb-ac30-6e9032fa7e2b.mp4

---

> Big thanks to [Philipp Oppermann](https://os.phil-opp.com/), his blog is what inspired me, and got Hugo4OS up and running.