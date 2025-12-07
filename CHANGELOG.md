# Changelog

All notable changes to this project will be documented in this file.

## [1.1.0] - 2025-12-04

### Fixed
- Fixed TOCTOU (Time-of-Check-Time-of-Use) race condition by removing premature file validation in argument parsing
- Fixed issue where output files were created during argument parsing, even if validation failed later
- Removed unnecessary parentheses around closure body that generated compiler warnings

### Changed
- Removed unused `_outfile` parameter from `determine_color_usage()` function
- File opening and validation now deferred to actual I/O operations for better error handling
- Added clear comments explaining deferred file operations to prevent TOCTOU issues

### Added
- Buffer size constants (`SEEK_BUFFER_SIZE`, `HEX_DECODE_BUFFER_SIZE`, `FILE_READ_BUFFER_SIZE`) for better maintainability
- This CHANGELOG.md file to track project changes
- Comprehensive inline documentation for complex functions
- Additional edge case tests for error conditions

### Security
- Eliminated TOCTOU race condition in file access validation
- Improved file handle management to prevent resource leaks

## [1.0.0] - 2025-06-30

### Added
- Initial release of rsxxd - A Rust implementation of the xxd hex dump utility
- **Full feature parity with xxd**:
  - Create hexadecimal dumps with customizable output formats
  - Convert hexadecimal dumps back to binary files (reverse operation)
  - Multiple output formats:
    - Standard hexadecimal dump with ASCII representation
    - Plain hexadecimal dump (PostScript style with `-p`/`-ps`)
    - C include file style output (`-i`)
    - Binary digit representation (`-b`)
  - Little-endian byte ordering support (`-e`)
  - EBCDIC character display (`-E`)
  - Colorized output with auto-detection (`-R`)
  - Customizable byte grouping (`-g`) and columns (`-c`)
  - File offset control (`-s`, `-o`)
  - Length limiting (`-l`)
  - Autoskip for null-line compression (`-a`)
  - Upper/lowercase hex letters (`-u`)
  - Decimal offset display (`-d`)
  - File name labeling (`-F`)
- **Zero dependencies** (only uses Rust standard library)
- **Cross-platform support**: Linux, macOS, Windows
- **Comprehensive test suite**: 7 integration tests covering all major features
- **Optimized release profile**: LTO, single codegen unit, stripped binaries
- **CI/CD pipeline**:
  - Multi-platform testing (Linux, macOS, Windows)
  - Multiple Rust versions (stable, beta, nightly)
  - Automated clippy linting
  - Code formatting verification
  - Spell checking
  - Performance benchmarking with hyperfine
  - Automated releases to crates.io and GitHub
- **Documentation**:
  - Comprehensive README with usage examples
  - Contributing guidelines
  - GPL-3.0-or-later license

### Performance
- Comparable or better performance than original xxd
- Efficient memory usage with buffered I/O
- Optimized hex lookup tables for fast encoding

[1.1.0]: https://github.com/elliottophellia/rsxxd/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/elliottophellia/rsxxd/releases/tag/v1.0.0
