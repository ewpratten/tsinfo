# tsinfo
[![Crates.io](https://img.shields.io/crates/v/tsinfo)](https://crates.io/crates/tsinfo) 
[![Docs.rs](https://docs.rs/tsinfo/badge.svg)](https://docs.rs/tsinfo) 
[![Build](https://github.com/Ewpratten/tsinfo/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/tsinfo/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/tsinfo/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/tsinfo/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/tsinfo/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/tsinfo/actions/workflows/audit.yml)


`tsinfo` is a CLI tool for providing information about a specific UNIX timestamp.

## Examples

```sh
# tsinfo 1625858332
1625858332 is July 9 2021, at 19:18:52 UTC (2 minutes ago)
```

```sh
# tsinfo
1625858471 is July 9 2021, at 19:21:11 UTC (now)
```

## Installation

This crate can be installed via `cargo` with:

```sh
cargo install tsinfo
```
