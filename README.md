# rsxxd

A Rust implementation of the popular hex dumping utility xxd.

## Description

rsxxd is a command-line utility for creating hexadecimal dumps of binary files or converting hexadecimal dumps back to binary form. It offers various output formats, byte grouping options, and colorized output to help with binary file analysis and manipulation.

## Features

- Create hexadecimal dumps with customizable output formats
- Convert hexadecimal dumps back to binary files
- Display binary data in different formats:
  - Standard hexadecimal dump with ASCII representation
  - Plain hexadecimal dump (PostScript style)
  - C include file style output
  - Binary digit representation
- Support for little-endian byte ordering
- EBCDIC character display
- Colorized output options
- Customizable byte grouping and columns

## Installation

### From Source

To build and install rsxxd from source:

```sh
git clone https://github.com/elliottophellia/rsxxd.git
cd rsxxd
cargo build --release
```

The compiled binary will be available at `target/release/rsxxd`.

### Using Cargo

```sh
cargo install rsxxd
```

## Usage

```
Usage:
       rsxxd [options] [infile [outfile]]
    or
       rsxxd -r [-s [-]offset] [-c cols] [-ps] [infile [outfile]]
Options:
    -a          toggle autoskip: A single '*' replaces nul-lines. Default off.
    -b          binary digit dump (incompatible with -ps). Default hex.
    -C          capitalize variable names in C include file style (-i).
    -c cols     format <cols> octets per line. Default 16 (-i: 12, -ps: 30).
    -E          show characters in EBCDIC. Default ASCII.
    -e          little-endian dump (incompatible with -ps,-i,-r).
    -F          label output with file name. Default off.
    -g bytes    number of octets per group in normal output. Default 2 (-e: 4).
    -h          print this summary.
    -i          output in C include file style.
    -l len      stop after <len> octets.
    -n name     set the variable name used in C include output (-i).
    -o off      add <off> to the displayed file position.
    -p          output in postscript plain hexdump style.
    -ps         output in postscript plain hexdump style (same as -p).
    -r          reverse operation: convert (or patch) hexdump into binary.
    -r -s off   revert with <off> added to file positions found in hexdump.
    -d          show offset in decimal instead of hex.
    -s [+][-]seek  start at <seek> bytes abs. (or +: rel.) infile offset.
    -u          use upper case hex letters.
    -R when     colorize the output; <when> can be 'always', 'auto' or 'never'. Default: 'auto'.
    -v          show version: "rsxxd 1.1.0 by Reidho Satria.".
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes between versions.

## License

```
                    GNU GENERAL PUBLIC LICENSE
                       Version 3, 29 June 2007

 Copyright (C) 2007 Free Software Foundation, Inc. <https://fsf.org/>
 Everyone is permitted to copy and distribute verbatim copies
 of this license document, but changing it is not allowed.
```