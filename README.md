# Longer

Tells you when you have been _____ longer than not.

## Description

Have you ever wondered when you will have been married longer than not? Or lived in a certain place longer than not? Or had a college degree longer than not? Or liked guacamole longer than not?

Tough to calculate manually, but easy with Longer! Just enter the start date (like your birthday), the day of the Event you're wondering about (like your wedding day, move date, graduation, or the revelation that guacamole is awesome), and Longer will do the job for you!

## Usage

```bash
longer --start <START_DATE> --event <EVENT_DATE>
```

### Arguments

- `--start, -s <START_DATE>`: The start date, e.g. your birthday (format: "yyyy-mm-dd")
- `--event, -e <EVENT_DATE>`: The event to count as the midpoint, e.g. your wedding date (format: "yyyy-mm-dd")

### Example

```bash
longer --start 1990-05-15 --event 2015-08-20
```

This will calculate when you will have been married (or whatever the event represents) longer than you were single.

## Installation

Make sure you have Rust installed, then:

```bash
cargo build --release
```

The binary will be available at `target/release/longer`.

## Author

Christoph Koehler
