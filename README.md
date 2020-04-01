# Bikuna TCP

[![Build Status](https://travis-ci.com/whoastonic/Bikuna.svg?branch=master)](https://travis-ci.com/whoastonic/Bikuna)

A command-line based TCP client

## Installation

To install this crate, you can install it from [crates.io](https://crates.io/)
If `cargo` is already installed:

```bash
$ cargo install bikuna
```

## Usage

```bash
# connecting & writing
# if the IP address wasn't specified 
# it'll default to 127.0.0.1
# but port is required

$ bikuna -p 3000 -m "Hello, Server!"
```

Connects Bikuna to `tcp://127.0.0.1:3000` and writes *"Hello, Server"* to the server

## Extra support

If you need additional support join the [Discord](https://discordapp.com/invite/eqwAFJW)
