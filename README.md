# evaporust
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](#)
[![Twitter: magnum\_dingus](https://img.shields.io/twitter/follow/magnum\_dingus.svg?style=social)](https://twitter.com/magnum\_dingus)

> CLI tool for cleaning up all those Rust projects you're totally going to work on at some point.

Evaporust scans for existing Rust projects (any directory containing a `Cargo.toml`) and runs `cargo clean` on them.

## Install

```sh
git clone https://github.com/dogue/evaporust.git
cd evaporust
cargo install --path .
```

## Usage

```
Usage: evaporust [OPTIONS]

Options:
  -b, --base-dir <DIRECTORY>  Directory from which to start scanning for projects
  -d, --dry-run               Scan for projects but don't actually run `cargo clean`
  -t, --total                 Print total number of projects found
  -l, --list                  Print a list of all projects found
  -x, --exclude <EXCLUDE>     List of strings. Paths that contain any of these will be skipped
  -h, --help                  Print help
```

## Author

👤 **dogue**

* Twitter: [@magnum\_dingus](https://twitter.com/magnum\_dingus)
* Github: [@dogue](https://github.com/dogue)

## Show your support

Give a ⭐️ if this project helped you!


***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
