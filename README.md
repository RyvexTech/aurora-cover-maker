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

# Aurora Cover Maker

A simple Linux command-line tool for creating Xbox 360 Aurora `.asset` cover files from JPG or PNG images.

Aurora Cover Maker creates Aurora-compatible boxart assets directly, without using the Aurora Asset Importer.

## Download

Download the latest pre-built version from the [GitHub Releases](../../releases) page.

For Linux x86_64, download:

`aurora-cover-maker`

The downloaded file is already compiled, so you do not need Rust or Cargo to use it.

## Usage

Make the program executable:

```bash
chmod +x aurora-cover-maker
```

Then run:

```bash
./aurora-cover-maker input.jpg output.asset
```

### Example

For an Aurora cover:

```bash
./aurora-cover-maker sm64-cover.jpg GC00000000.asset
```

The resulting `.asset` file can then be placed in the appropriate Aurora `Data/GameData` directory.

## Supported Images

The program supports:

* JPG / JPEG
* PNG

Make sure the file extension matches the actual image format. For example, a JPEG image should normally use `.jpg` or `.jpeg`, not `.png`.

## Building From Source

If you want to build the program yourself, install Rust and Cargo and run:

```bash
cargo build --release
```

The compiled program will be located at:

```text
target/release/aurora-cover-maker
```

## Requirements

### Pre-built version

* Linux x86_64
* No Rust installation required

### Building from source

* Linux
* Rust
* Cargo

## Credits

Aurora Cover Maker uses the [`libaustralis`](https://github.com/jrobiche/libaustralis) project to create Aurora-compatible assets.

## Disclaimer

Aurora Cover Maker is an independent project and is not affiliated with Aurora, Xbox, or Microsoft.
