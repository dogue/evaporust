# evaporust
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](#)

> CLI tool for cleaning up all those Rust projects you're totally going to work on at some point.

Evaporust scans for existing Rust projects (any directory containing a `Cargo.toml`) and runs `cargo clean` on them.

By default, it will only clean projects that contain a `target` directory, indicating the existence of build artifacts.

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
  -a, --all-projects          Scan and clean all projects, even if they are already clean
  -b, --base-dir <DIRECTORY>  Directory from which to start scanning for projects
  -d, --dry-run               Scan for projects but don't actually run `cargo clean`
  -t, --total                 Print total number of projects found
  -l, --list                  Print a list of all projects found
  -x, --exclude <EXCLUDE>     Comma-separated list of strings. Paths that contain any of these will be skipped
  -h, --help                  Print help
```
