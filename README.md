<!-- Optional: Project Banner/Logo -->
<!-- ![Longer Logo](https://raw.githubusercontent.com/ckoehler/longer/main/assets/logo.png) -->

[![CI](https://github.com/ckoehler/longer/actions/workflows/ci.yml/badge.svg)](https://github.com/ckoehler/longer/actions/workflows/ci.yml)
[![Release](https://github.com/ckoehler/longer/actions/workflows/release.yml/badge.svg)](https://github.com/ckoehler/longer/actions/workflows/release.yml)

# Longer

Tells you when you have been _____ longer than not.

---

## Table of Contents

- [Description](#description)
- [Quickstart](#quickstart)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Releases](#releases)
- [Contributing](#contributing)
- [License](#license)
- [Links](#links)
- [Maintainer](#maintainer)

---

## Description

Have you ever wondered when you will have been married longer than not? Or lived in a certain place longer than not? Or had a college degree longer than not? Or liked guacamole longer than not?

Tough to calculate manually, but easy with Longer! Just enter the start date (like your birthday), the day of the Event you're wondering about (like your wedding day, move date, graduation, or the revelation that guacamole is awesome), and Longer will do the job for you!

---

## Quickstart

Download a release binary and run:

```bash
./longer-linux-x86_64 --start 1990-05-15 --event 2015-08-20
```

---

## Features

- Calculates "longer than not" dates for any event
- Supports multiple platforms (Linux, macOS, Windows)
- Simple CLI interface
- Fast and lightweight
- Open source

---

## Installation

You can either download a pre-built binary from [GitHub Releases](https://github.com/ckoehler/longer/releases) or build from source:

Make sure you have Rust installed, then:

```bash
cargo build --release
```

The binary will be available at `target/release/longer`.

Alternatively, install via Cargo (requires Rust):

```bash
cargo install longer
```

---

## Usage

```bash
longer --start <START_DATE> --event <EVENT_DATE>
```

### Arguments

- `--start, -s <START_DATE>`: The start date, e.g. your birthday (format: "yyyy-mm-dd")
- `--event, -e <EVENT_DATE>`: The event to count as the midpoint, e.g. your wedding date (format: "yyyy-mm-dd")

### Example

```bash
./longer-linux-x86_64 --start 1990-05-15 --event 2015-08-20
```

This will calculate when you will have been married (or whatever the event represents) longer than you were single.

---

## Releases

Pre-built binaries for Linux, Windows, and macOS (both x86_64 and aarch64) are available on the [GitHub Releases page](https://github.com/ckoehler/longer/releases).

To use a release binary:
1. Download the appropriate file for your operating system and architecture.
2. Unpack it if necessary.
3. Run the binary from your terminal.

Example (Linux):
```bash
./longer-linux-x86_64 --start 1990-05-15 --event 2015-08-20
```

Example (macOS x86_64):
```bash
./longer-macos-x86_64 --start 1990-05-15 --event 2015-08-20
```

Example (macOS aarch64):
```bash
./longer-macos-aarch64 --start 1990-05-15 --event 2015-08-20
```

Example (Windows):
```powershell
longer-windows-x86_64.exe --start 1990-05-15 --event 2015-08-20
```

---

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## License

This project is licensed under the MIT License.

---

## Links

- [Documentation](https://github.com/ckoehler/longer/wiki)
- [Report Issues](https://github.com/ckoehler/longer/issues)

---

## Maintainer

[@ckoehler](https://github.com/ckoehler)
