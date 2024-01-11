# 1brc 1️⃣🐝🏎️

[![Rust](https://github.com/gabrieledarrigo/1brc/actions/workflows/build.yml/badge.svg)](https://github.com/gabrieledarrigo/1brc/actions/workflows/build.yml)

[**The One Billion Row Challenge**](https://www.morling.dev/blog/one-billion-row-challenge/)

A fun exploration of how quickly 1B rows from a text file can be aggregated with Rust.

## Generate the data file

To generate the data file, run the following command:

```sh
time cargo run --release --bin create-measurements 1000000000
```

## Run the challenge

```sh
$ cargo build --release && time target/release/1brc >/dev/null
```
