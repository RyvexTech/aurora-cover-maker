# Aurora Cover Maker

A Linux command-line tool for creating Xbox 360 Aurora `.asset` cover files from JPG or PNG images.

Aurora Cover Maker can automatically look up Xbox 360 games and their Title IDs using the [x360db](https://github.com/xenia-manager/x360db) database. It also supports custom game aliases through `aliases.json`.

No Aurora Asset Importer is required.

## Features

* Create Aurora-compatible `.asset` cover files
* Automatically find Xbox 360 Title IDs by game name
* Search the Xbox 360 game database
* Support custom aliases such as `GTA 4` → `GTA IV`
* Supports JPG/JPEG and PNG images
* Linux x86_64 pre-built release available
* Uses BC3 texture compression for Aurora boxart

## Download

Download the latest pre-built version from the GitHub Releases page.

For Linux x86_64, download:

`aurora-cover-maker-linux-x86_64`

The downloaded file is already compiled, so Rust and Cargo are not required to use the pre-built version.

## Usage

Make the program executable:

```bash
chmod +x aurora-cover-maker
```

### Create a cover

You can enter the official game title:

```bash
./aurora-cover-maker "GTA IV" cover.jpg
```

You can also use an alias:

```bash
./aurora-cover-maker "GTA 4" cover.jpg
```

The program automatically:

1. Looks up the game in the Xbox 360 database
2. Finds the Title ID
3. Creates the Aurora asset
4. Names the asset using the Title ID

Example:

```text
Downloading Xbox 360 game database...
Alias: GTA 4 → GTA IV
Game: GTA IV
Title ID: 545407F2
Created: GC545407F2.asset
```

The resulting file is:

```text
GC545407F2.asset
```

## Search for Games

You can search the Xbox 360 database without creating a cover:

```bash
./aurora-cover-maker search "GTA IV"
```

You can also search using an alias:

```bash
./aurora-cover-maker search "GTA 4"
```

Example:

```text
Downloading Xbox 360 game database...
Alias: GTA 4 → GTA IV

Found 1 game(s):

GTA IV — 545407F2
```

Searches can also return multiple results. For example:

```bash
./aurora-cover-maker search "Minecraft"
```

This can show multiple Xbox 360 games matching the search.

## Game Aliases

Aurora Cover Maker supports a local `aliases.json` file.

This allows common names, abbreviations and community names to point to an official database title.

Example:

```json
{
  "GTA 4": "GTA IV",
  "Sonic 06": "SONIC THE HEDGEHOG",
  "RDR1": "Red Dead Redemption",
  "Skyrim": "The Elder Scrolls V: Skyrim"
}
```

Aliases are resolved before searching the Xbox 360 database.

You can add your own aliases without changing the Rust source code.

## Supported Images

The program supports:

* JPG
* JPEG
* PNG

**Important:** the file extension must match the actual image format.

For example, if a file is actually a JPEG but is named `.png`, the image decoder may fail.

You can check an image with:

```bash
file cover.png
```

If it reports `JPEG image data`, rename or copy it with a `.jpg` extension.

## Aurora Asset Output

The generated asset follows the Aurora naming convention:

```text
GC<TITLEID>.asset
```

For example:

```text
GC545407F2.asset
```

The asset can then be placed in the appropriate Aurora `Data/GameData` directory for the corresponding game.

## Building From Source

Clone the repository and build with Cargo:

```bash
git clone https://github.com/RyvexTech/aurora-cover-maker.git
cd aurora-cover-maker
cargo build --release
```

The compiled program will be located at:

```text
target/release/aurora-cover-maker
```

## Requirements

### Pre-built version

* Linux x86_64
* Internet connection for Xbox 360 game database lookups
* No Rust installation required

### Building from source

* Linux
* Rust
* Cargo

## Database

Game information is retrieved from the x360db project:

https://github.com/xenia-manager/x360db

Database:

```text
https://raw.githubusercontent.com/xenia-manager/x360db/main/games.json
```

Aurora Cover Maker does not host or redistribute Xbox 360 game files.

## Version

Current release:

**v0.1.3**

## Credits

Aurora Cover Maker uses the [`libaustralis`](https://github.com/jrobiche/libaustralis) project to create Aurora-compatible assets.

Thanks to the x360db project for providing Xbox 360 game metadata.

## Disclaimer

Aurora Cover Maker is an independent project and is not affiliated with Aurora, Xbox, or Microsoft.
