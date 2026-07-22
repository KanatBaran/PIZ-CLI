# What is PIZ?
**Technology is more meaningful when it benefits people.**

PIZ is a movement that aims to blend technology with social good, elevating software development beyond just writing code. Our goal is to encourage developers to discover social initiatives and charities around the world while leveraging their technical skills.

## Why Join PIZ?
Technology becomes truly meaningful when it impacts human lives. At PIZ, we want to turn the process of writing code into an act of awareness. PIZ-CLI is just the beginning of this vision; over time, we will expand this ecosystem with new tools and components focused on social benefit.

## How Can You Help?
The PIZ movement thrives on the power of its community. You can contribute to the PIZ-CLI project by improving the code, enhancing the documentation, or simply spreading the word. Every contribution helps push forward not just a single tool, but a movement that bridges technology and empathy.

# PIZ-CLI
PIZ CLI is an open-source command-line tool that expands files to a specified size. Instead of compressing data, PIZ makes files larger by appending either the digits of π (Pi)* or random data. *PIZ-CLI is the debut project of the PIZ movement, intentionally designed with an unconventional functionality to spark curiosity and draw attention to social good.*

*\*Note: Rather than computing infinite digits of Pi, we use the first 101 digits of Pi as a seed with the BLAKE3 cryptographic hash function (XOF) to deterministically generate a stream of bytes.*

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

# Add 500 MB of random data
piz expand photo.jpg --add 500MB --fill random

# Expand photo.jpg to a target size of 2 GB
piz expand photo.jpg --size 2GB

# Expand photo.jpg to 2 GB and save as photo-2gb.jpg
piz expand photo.jpg --size 2GB -o photo-2gb.jpg
```