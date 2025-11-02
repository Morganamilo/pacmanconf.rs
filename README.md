# pacmanconf.rs

A Rust library for parsing pacman configuration files.

## Overview

pacmanconf.rs is a parser for Arch Linux's pacman configuration format. It provides a simple API to read and parse `pacman.conf` files into structured Rust types.

## Crates

This workspace contains two crates:

- **pacmanconf**: Parser for pacman.conf files
- **cini**: Callback-based INI parser (used internally by pacmanconf)

## Usage

```rust
use pacmanconf::Config;

// Parse the default pacman.conf
let config = Config::new()?;

// Parse a specific config file
let config = Config::options()
    .pacman_conf("/etc/pacman.conf")
    .read()?;

// Access configuration
for repo in &config.repos {
    println!("Repository: {}", repo.name);
}
```

## Documentation

Documentation is available at [docs.rs/pacmanconf](https://docs.rs/pacmanconf).

## License

GPL-3.0
