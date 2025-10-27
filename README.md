# insert-vocab

A simple Rust project for inserting vocabulary along with their definitions into Obsidian notes.

## Environment configuration

**Essential Step:**
Create a `.env` file in the project root with the `OBSIDIAN_VAULT_PATH` variable. For example:

```
OBSIDIAN_VAULT_PATH="/absolute/path/to/your/Obsidian/Vault/🔤 Vocabulary/"
```

Replace the value with the absolute path to your own Obsidian vault vocabulary folder. Make sure to wrap the path in double quotes if it contains spaces or special characters. This step is required for the script to work correctly.

## Usage

### Build

To build the project:

```
cargo build
```

For an optimized release build:

```
cargo build --release
```

### Run

The release binary will be located at `./target/release/insert-vocab`.

Pass the words to be inserted into Obsidian as command line arguments. You can redirect a file containing words (e.g., `test.txt`) like this:

```
./target/release/insert-vocab $(< test.txt)
```

## Requirements

- Rust 1.70+

## API Reference

This project uses the [Free Dictionary API](https://freedictionaryapi.com/) to fetch vocabulary data.

## License

MIT

https://help.obsidian.md/Extending+Obsidian/Obsidian+URI
