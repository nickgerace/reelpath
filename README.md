# reelpath

[![GitHub](https://img.shields.io/github/license/nickgerace/reelpath?style=flat-square)](./LICENSE)
[![Latest SemVer GitHub Tag](https://img.shields.io/github/v/tag/nickgerace/reelpath?label=version&style=flat-square)](https://github.com/nickgerace/reelpath/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/reelpath?style=flat-square)](https://crates.io/crates/reelpath)

`reelpath` is a CLI application that prints the absolute path for a given file or directory.

```bash
user at host in ~/github.com/nickgerace/reelpath
% reelpath README.md
/home/user/github.com/nickgerace/reelpath/README.md
```

## Motivation

Could this exist as a shell function?
Probably.
Did I want a cross-platform method to get the absolute path for a given file or directory?
Yes.

## Installation

There are multiple ways to install `reelpath`, but this section contains the recommended methods.

> Since `reelpath` does not use any external dependencies, it supports all platforms that [Rust](https://www.rust-lang.org/) supports.
> Please file an [issue](https://github.com/nickgerace/reelpath/issues) if you encounter an error on your platform of choice.

### Homebrew

You can use [Homebrew](https://brew.sh) to install the [tap](https://github.com/nickgerace/homebrew-reelpath) for `reelpath`.

```bash
brew install nickgerace/reelpath/reelpath
```

Alternatively, you can execute:

```bash
brew tap nickgerace/reelpath
brew install reelpath
```

Running `brew help` or `man brew` can help you use `brew` locally.
You can check out [Homebrew's documentation](https://docs.brew.sh) as well.

### Cargo Install

You can install from [crates.io](https://crates.io/crates/reelpath) by executing the following:

```bash
cargo install reelpath
```

Keeping the package up to date is easy with [cargo-update](https://crates.io/crates/cargo-update).

## Additional Nodes

### Where is `CHANGELOG.md`?

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html), but does not leverage popular changelog formats, like [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), because of its tiny size and purpose.
Using GitHub to compare two tags, commits, etc. should suffice for this project.

### Where is the continuous integration?

Local developer testing suffices for this project due to its tiny size and purpose.
Continuous integration would be wasteful, but this is subject to change.

### Where are the binaries?

There are no binaries uploaded to GitHub since this application is best distributed via a package manager, such as [cargo](https://crates.io/). 
Use one of the README's installation methods to build and install `reelpath`.

## Code of Conduct

This repository follows and enforces the Rust programming language's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Additional Information

- Author: [Nick Gerace](https://nickgerace.dev)
- License: [Apache 2.0](./LICENSE)
