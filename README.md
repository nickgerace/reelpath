# reelpath

[![Latest SemVer GitHub Tag](https://img.shields.io/github/v/tag/nickgerace/reelpath?label=version&style=flat-square)](https://github.com/nickgerace/reelpath/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/reelpath?style=flat-square)](https://crates.io/crates/reelpath)
[![License](https://img.shields.io/github/license/nickgerace/reelpath?style=flat-square)](./LICENSE)

`reelpath` is a CLI application that prints the absolute path for a given file or directory.

```sh
user at host in ~/github.com/nickgerace/reelpath
% reelpath README.md
/home/user/github.com/nickgerace/reelpath/README.md
```

## Motivation

Could this exist as a shell function?
Probably.
Did I want a cross-platform method to get the absolute path for a given file or directory?
Yes.

Since `reelpath` does not use any external dependencies, it supports all platforms that [Rust](https://www.rust-lang.org/) supports.
**Please file an [issue](https://github.com/nickgerace/reelpath/issues)** if you encounter an error on your platform of choice.

## Usage

You can supply multiple arguments to the CLI, use pipes with `xargs`, and use wildcards in a single argument.

```sh
reelpath file
reelpath directory
reelpath ../relative/path/to/file
reelpath ../relative/path/to/directory
reelpath $(ls)
reelpath file.* file.extension directory/*
ls | xargs reelpath
```

## Installation

You can use [cargo](https://crates.io) to install the [crate](https://crates.io/crates/reelpath) on almost any platform.

```sh
cargo install reelpath
```

Keeping the crate up to date is easy with [cargo-update](https://crates.io/crates/cargo-update).

```sh
cargo install cargo-update
cargo install-update -a
```

## Additional Notes

### Where is `CHANGELOG.md`?

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html), but does not leverage popular changelog formats, like [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), because of its tiny size and purpose.
Using GitHub to compare two tags, commits, etc. should suffice for this project.

```
https://github.com/nickgerace/reelpath/compare/<one-tag>...<another-tag>
https://github.com/nickgerace/reelpath/compare/<latest-tag>...HEAD
```

### Where is the continuous integration?

Local developer testing suffices for this project due to its tiny size and purpose.
Continuous integration would be wasteful, but this is subject to change.

## Code of Conduct

This repository follows and enforces the Rust programming language's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
