# kx

`kx` is a utility to switch interactively between kubernetes contexts without any external dependencies and bash witchcraft. Written in Rust :crab:

![Build](https://github.com/onatm/kx/workflows/Build/badge.svg)

## Installation

### From binary

You can directly [download the kx executable](https://github.com/onatm/kx/releases).

### Install from crates.io

```sh
cargo install kx
```

### Build Manually

Clone the repo and run:

```sh
cargo install --path .
```

Alternatively, run:

```sh
cargo build --release
```

then put the resulting `target/release/kx` executable on your PATH.

## Usage

```md
  kx                   : list the contexts
  kx <NAME>            : switch to context <NAME>
  kx -c, --current     : show the current context name
```

## Todo

- [ ] Add tests
- [ ] `bash`/`zsh`/`fish` completions
- [ ] Use [crossterm](https://github.com/crossterm-rs/crossterm) based solution instead of [skim](https://github.com/lotabout/skim)
- [ ] Windows support
