# PIZ-CLI
PIZ CLI is an open-source command-line tool that expands files to a specified size. Instead of compressing data, PIZ makes files larger by appending either the digits of π (Pi) or random digits (`0–9`).

## Supported File Extensions

PIZ appends data to the end of a file. These formats typically remain usable after expansion:

<p>
  <img src="assets/icons/jpg.svg" alt="JPG" width="64" height="64">
  <img src="assets/icons/png.svg" alt="PNG" width="64" height="64">
  <img src="assets/icons/txt.svg" alt="TXT" width="64" height="64">
  <img src="assets/icons/exe.svg" alt="EXE" width="64" height="64">
</p>

### Never Supported

<p>
  <img src="assets/icons/rar.svg" alt="RAR" width="64" height="64">
</p>

We respect RAR.

## Usage

```bash
piz expand <file> [options]
```

### Options

*   `--add <size>`: Appends the specified amount of data (e.g., `1GB`, `500MB`).
*   `--size <size>`: Expands the file to the target size.
*   `--fill <pi|random>`: The pattern to fill with (default: `pi`).
*   `-o, --output <file>`: Specifies the output file path. By default, a new file is created without modifying the original.

> * You must specify either `--add` or `--size`, but not both.
> * The target `--size` cannot be smaller than the current file size.

### Examples

```bash
# Add 1 GB of Pi digits (default) to photo.jpg
piz expand photo.jpg --add 1GB

# Add 500 MB of random digits
piz expand photo.jpg --add 500MB --fill random

# Expand photo.jpg to a target size of 2 GB
piz expand photo.jpg --size 2GB

# Expand photo.jpg to 2 GB and save as photo-2gb.jpg
piz expand photo.jpg --size 2GB -o photo-2gb.jpg
```