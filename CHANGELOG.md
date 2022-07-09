# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased] - 2022-07-09

### Changed
- [`Hugo4OS-Bootloader`] Is no longer a submodule, but part of the repo
- [`Hugo4OS-Bootloader`] Is now its own crate, instead of a fork of [`rust-osdev/bootloader`](github.com/rust-osdev/bootloader)
  > It's still using the `rust-osdev/bootloader` codebase, but without its build system. BIOS and UEFI will probably also be split into seperate crates.
- Revamped entire build system for more control
  > Instead of building using `rust-osdev/bootloader`'s multi-step build-system, `Hugo4OS` now uses [`just`](https://github.com/casey/just) for running commands directly. This change significantly decreases build times as there `build.rs`, `builder.rs` and the `boot` crate now no longer need to be compiled.
- Hugo4OS images now export to `img/` instead of `target/x86_64-bootloader/release/`

[Unreleased]: https://github.com/Hugo4IT/Hugo4OS
[`Hugo4OS-Bootloader`]: https://github.com/Hugo4IT/Hugo4OS/tree/master/crates/bootloader_bios