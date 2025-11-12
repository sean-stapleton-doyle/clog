# clog

A `pest`-based parser for [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) formatted `CHANGELOG.md` files.

## Binary

The project includes a binary driver to parse a CHANGELOG file and print its debug-formatted structure. See TODOs for why I am doing this.

### Usage

```bash
cargo run -- <path/to/CHANGELOG.md>
```

## Library

The core logic is available as a library.

  * **`clog::models`**: Defines the core data structures (e.g., `Changelog`, `VersionEntry`, `ChangeType`).
  * **`clog::parser::parse_changelog`**: The main function, which takes a string slice (`&str`) and returns a `Result<Changelog, ...>`.

## TODO

  * Add CLI for validating and updating changelogs.
  * Split changelog parsing to its own, separate library.
