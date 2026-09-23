# Aurora Cover Maker

A Linux command-line tool for creating Xbox 360 Aurora cover `.asset` files from images.

## Features

* Runs on Linux
* Converts images into Aurora-compatible `.asset` files
* Supports Aurora boxart/cover assets
* Useful for manually adding artwork to Xbox 360 homebrew and games
* No Aurora Asset Importer required

## Requirements

* Linux
* Rust/Cargo
* An image file such as JPG or PNG

## Usage

```bash
aurora-cover-maker input.jpg output.asset
```

### Example

```bash
aurora-cover-maker sm64-cover.jpg GC00000000.asset
```

## Building

```bash
cargo build --release
```

The compiled program will be located at:

```text
target/release/aurora-cover-maker
```

## Credits

Built using the `libaustralis` project.

