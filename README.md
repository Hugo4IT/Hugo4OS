# Hugo4OS

Me just experimenting with OS development in rust

## Usage

> Minimum supported rustc: `1.61.0-nightly` (I'm using `0677edc86 2022-03-31`).
>
> `llvm-tools-preview` must also be installed and available in PATH: `rustup component add llvm-tools-preview`
>
> To enable verbose output in the terminal, append `--features verbose` to any of these commands:

- Building and running with the resulting image in [Qemu](https://www.qemu.org/): `just run`

Big thanks to [Philipp Oppermann](https://os.phil-opp.com/)

## Showcase

A screenshot (taken in QEMU) of current rendering capabilities in Hugo4OS:

![wowie](./.github/showcase/screenshot-2022-04-03_19-03-52.png)

A video of Hugo4OS running on real hardware (turn the sound on!):

https://user-images.githubusercontent.com/48245742/161477623-4e3c07cb-3731-45eb-ac30-6e9032fa7e2b.mp4
