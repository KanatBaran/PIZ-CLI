# PIZ-CLI
PIZ CLI is an open-source command-line tool that expands files to a specified size. Instead of compressing data, PIZ makes files larger by appending either the digits of π (Pi) or random digits (`0–9`).
<br>

## Supported File Extensions

PIZ appends data to the end of a file. These formats typically remain usable after expansion:

<p align="center">
  <img src="assets/icons/jpg.svg" alt="JPG" width="80" height="80">&nbsp;&nbsp;
  <img src="assets/icons/png.svg" alt="PNG" width="80" height="80">&nbsp;&nbsp;
  <img src="assets/icons/txt.svg" alt="TXT" width="80" height="80">&nbsp;&nbsp;
  <img src="assets/icons/exe.svg" alt="EXE" width="80" height="80">&nbsp;&nbsp;
  <img src="assets/icons/pdf.svg" alt="PDF" width="80" height="80">
</p>

### Never Supported

<p align="center">
  <img src="assets/icons/rar.svg" alt="RAR" width="80" height="80">
</p>

<p align="center">We respect RAR.</p>
<br>

## Installation

### Windows
1. Download `piz.exe` from the [Latest Release](https://github.com/KanatBaran/PIZ-CLI/releases/latest).
2. Open PowerShell or Command Prompt in the folder containing `piz.exe`.
3. Run:
   ```powershell
   .\piz.exe expand <file> [options]
   ```
*(Tip: Add `piz.exe` to your system PATH to run `piz` directly from anywhere!)*

### Linux & macOS
1. Download the archive for your OS from the [Latest Release](https://github.com/KanatBaran/PIZ-CLI/releases/latest):
   - **Linux (x86_64):** `piz-linux-x86_64.tar.gz`
   - **macOS (Apple Silicon):** `piz-macos-arm64.tar.gz`
2. Extract the archive and make the binary executable:
   ```bash
   tar -xzf piz-linux-x86_64.tar.gz
   chmod +x piz
   ./piz expand <file> [options]
   ```

### From Source (Requires Rust)
```bash
git clone https://github.com/KanatBaran/PIZ-CLI.git
cd PIZ-CLI
cargo install --path .
```
<br>

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
<br>

## Support & Donations 💙

Let's be honest: a tool that makes files unnecessarily huge probably isn't the most practical utility in your daily workflow. But if **PIZ** brought a smile to your face, made you chuckle, or you just appreciate the absurdity of "anti-compression", please consider turning that smile into something meaningful. 

Please consider donating to global humanitarian causes:

<p align="center">
  <a href="https://help.unicef.org/">
    <img src="https://img.shields.io/badge/Donate-UNICEF-00adef?style=for-the-badge&logo=unicef&logoColor=white" alt="Donate to UNICEF" height="40">
  </a>
  &nbsp;&nbsp;
  <a href="https://who.foundation/donate/">
    <img src="https://img.shields.io/badge/Donate-WHO%20Foundation-008dc9?style=for-the-badge&logo=worldhealthorganization&logoColor=white" alt="Donate to WHO" height="40">
  </a>
</p>