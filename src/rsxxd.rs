use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, IsTerminal, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process;
use std::result::Result;

const MY_VERSION: &str = "1.0.0 by Reidho Satria.";

const HEX_LOWER: [&str; 256] = [
    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0a", "0b", "0c", "0d", "0e", "0f",
    "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1a", "1b", "1c", "1d", "1e", "1f",
    "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2a", "2b", "2c", "2d", "2e", "2f",
    "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3a", "3b", "3c", "3d", "3e", "3f",
    "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4a", "4b", "4c", "4d", "4e", "4f",
    "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "5a", "5b", "5c", "5d", "5e", "5f",
    "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "6a", "6b", "6c", "6d", "6e", "6f",
    "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "7a", "7b", "7c", "7d", "7e", "7f",
    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "8a", "8b", "8c", "8d", "8e", "8f",
    "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "9a", "9b", "9c", "9d", "9e", "9f",
    "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "aa", "ab", "ac", "ad", "ae", "af",
    "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8", "b9", "ba", "bb", "bc", "bd", "be", "bf",
    "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "ca", "cb", "cc", "cd", "ce", "cf",
    "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "da", "db", "dc", "dd", "de", "df",
    "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "ea", "eb", "ec", "ed", "ee", "ef",
    "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "fa", "fb", "fc", "fd", "fe", "ff",
];

const HEX_UPPER: [&str; 256] = [
    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0A", "0B", "0C", "0D", "0E", "0F",
    "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1A", "1B", "1C", "1D", "1E", "1F",
    "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2A", "2B", "2C", "2D", "2E", "2F",
    "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3A", "3B", "3C", "3D", "3E", "3F",
    "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4A", "4B", "4C", "4D", "4E", "4F",
    "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "5A", "5B", "5C", "5D", "5E", "5F",
    "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "6A", "6B", "6C", "6D", "6E", "6F",
    "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "7A", "7B", "7C", "7D", "7E", "7F",
    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "8A", "8B", "8C", "8D", "8E", "8F",
    "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "9A", "9B", "9C", "9D", "9E", "9F",
    "A0", "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "A9", "AA", "AB", "AC", "AD", "AE", "AF",
    "B0", "B1", "B2", "B3", "B4", "B5", "B6", "B7", "B8", "B9", "BA", "BB", "BC", "BD", "BE", "BF",
    "C0", "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "CA", "CB", "CC", "CD", "CE", "CF",
    "D0", "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "DA", "DB", "DC", "DD", "DE", "DF",
    "E0", "E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "E9", "EA", "EB", "EC", "ED", "EE", "EF",
    "F0", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "FA", "FB", "FC", "FD", "FE", "FF",
];

#[derive(Debug)]
enum XxdError {
    IoError(io::Error, String),
    ParseError(String),
    InvalidArgument(String),
    IncompatibleOptions(String),
    FileNotFound(String),
    FilePermissionDenied(String),
    FileAlreadyExists(String),
}

impl fmt::Display for XxdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            XxdError::IoError(err, context) => write!(f, "I/O error: {context} ({err})"),
            XxdError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            XxdError::InvalidArgument(msg) => write!(f, "Invalid argument: {msg}"),
            XxdError::IncompatibleOptions(msg) => write!(f, "Incompatible options: {msg}"),
            XxdError::FileNotFound(path) => write!(f, "File not found: {path}"),
            XxdError::FilePermissionDenied(path) => write!(f, "Permission denied: {path}"),
            XxdError::FileAlreadyExists(path) => write!(f, "File already exists: {path}"),
        }
    }
}

impl Error for XxdError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            XxdError::IoError(err, _) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for XxdError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => XxdError::FileNotFound(error.to_string()),
            io::ErrorKind::PermissionDenied => XxdError::FilePermissionDenied(error.to_string()),
            io::ErrorKind::AlreadyExists => XxdError::FileAlreadyExists(error.to_string()),
            _ => XxdError::IoError(error, "I/O operation failed".to_string()),
        }
    }
}

type XxdResult<T> = Result<T, XxdError>;

const EBCDIC_TO_ASCII: [char; 256] = [
    '\u{0000}', '\u{0001}', '\u{0002}', '\u{0003}', '\u{009C}', '\u{0009}', '\u{0086}', '\u{007F}',
    '\u{0097}', '\u{008D}', '\u{008E}', '\u{000B}', '\u{000C}', '\u{000D}', '\u{000E}', '\u{000F}',
    '\u{0010}', '\u{0011}', '\u{0012}', '\u{0013}', '\u{009D}', '\u{0085}', '\u{0008}', '\u{0087}',
    '\u{0018}', '\u{0019}', '\u{0092}', '\u{008F}', '\u{001C}', '\u{001D}', '\u{001E}', '\u{001F}',
    '\u{0080}', '\u{0081}', '\u{0082}', '\u{0083}', '\u{0084}', '\u{000A}', '\u{0017}', '\u{001B}',
    '\u{0088}', '\u{0089}', '\u{008A}', '\u{008B}', '\u{008C}', '\u{0005}', '\u{0006}', '\u{0007}',
    '\u{0090}', '\u{0091}', '\u{0016}', '\u{0093}', '\u{0094}', '\u{0095}', '\u{0096}', '\u{0004}',
    '\u{0098}', '\u{0099}', '\u{009A}', '\u{009B}', '\u{0014}', '\u{0015}', '\u{009E}', '\u{001A}',
    ' ', '\u{00A0}', '\u{00E2}', '\u{00E4}', '\u{00E0}', '\u{00E1}', '\u{00E3}', '\u{00E5}',
    '\u{00E7}', '\u{00F1}', '[', '.', '<', '(', '+', '|', '&', '\u{00E9}', '\u{00EA}', '\u{00EB}',
    '\u{00E8}', '\u{00ED}', '\u{00EE}', '\u{00EF}', '\u{00EC}', '\u{00DF}', ']', '$', '*', ')',
    ';', '^', '-', '/', '\u{00C2}', '\u{00C4}', '\u{00C0}', '\u{00C1}', '\u{00C3}', '\u{00C5}',
    '\u{00C7}', '\u{00D1}', '\u{00A6}', ',', '%', '_', '>', '?', '\u{00F8}', '\u{00C9}',
    '\u{00CA}', '\u{00CB}', '\u{00C8}', '\u{00CD}', '\u{00CE}', '\u{00CF}', '\u{00CC}', '`', ':',
    '#', '@', '\'', '=', '"', '\u{00D8}', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', '\u{00AB}',
    '\u{00BB}', '\u{00F0}', '\u{00FD}', '\u{00FE}', '\u{00B1}', '\u{00B0}', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', '\u{00AA}', '\u{00BA}', '\u{00E6}', '\u{00B8}', '\u{00C6}',
    '\u{00A4}', '\u{00B5}', '~', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '\u{00A1}', '\u{00BF}',
    '\u{00D0}', '\u{00DD}', '\u{00DE}', '\u{00AE}', '\u{00AC}', '\u{00A3}', '\u{00A5}', '\u{00B7}',
    '\u{00A9}', '\u{00A7}', '\u{00B6}', '\u{00BC}', '\u{00BD}', '\u{00BE}', '\u{00D7}', '\u{00F7}',
    '\u{00A8}', '\u{00B4}', '\u{00D6}', '\u{00F6}', '{', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', '\u{00A2}', '.', '<', '(', '+', '|', '}', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    '!', '$', '*', ')', ';', '^', '\\', '\u{00F9}', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '\u{00B2}', '\u{00A2}', '@', '\'', '=', '"', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    '|', ',', '%', '_', '>', '?',
];

struct Colors {
    address: &'static str,
    hex: &'static str,
    ascii: &'static str,
    reset: &'static str,
}

const COLOR_CODES: Colors = Colors {
    address: "\x1b[33m",
    hex: "\x1b[36m",
    ascii: "\x1b[32m",
    reset: "\x1b[0m",
};

struct Options {
    autoskip: bool,
    bin_hex: bool,
    capitalize: bool,
    cols: usize,
    decimal_offset: bool,
    ebcdic: bool,
    little_endian: bool,
    octspergrp: usize,
    fin: Option<String>,
    fout: Option<String>,
    postscript: bool,
    include: bool,
    len: u64,
    var_name: String,
    add_offset: u64,
    revert: bool,
    seek: u64,
    upper_hex: bool,
    colsgiven: bool,
    octspergrpgiven: bool,
    show_filename: bool,
    color_mode: String,
    use_color: bool,
    from_eof: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            autoskip: false,
            bin_hex: false,
            capitalize: false,
            cols: 16,
            decimal_offset: false,
            ebcdic: false,
            little_endian: false,
            octspergrp: 2,
            fin: None,
            fout: None,
            postscript: false,
            include: false,
            len: u64::MAX,
            var_name: String::from("rsxxd_dump"),
            add_offset: 0,
            revert: false,
            seek: 0,
            upper_hex: false,
            colsgiven: false,
            octspergrpgiven: false,
            show_filename: false,
            color_mode: String::from("auto"),
            use_color: false,
            from_eof: false,
        }
    }
}

fn print_usage() -> ! {
    eprintln!("Usage:");
    eprintln!("       rsxxd [options] [infile [outfile]]");
    eprintln!("    or");
    eprintln!("       rsxxd -r [-s [-]offset] [-c cols] [-ps] [infile [outfile]]");
    eprintln!("Options:");
    eprintln!("    -a          toggle autoskip: A single '*' replaces nul-lines. Default off.");
    eprintln!("    -b          binary digit dump (incompatible with -ps). Default hex.");
    eprintln!("    -C          capitalize variable names in C include file style (-i).");
    eprintln!("    -c cols     format <cols> octets per line. Default 16 (-i: 12, -ps: 30).");
    eprintln!("    -E          show characters in EBCDIC. Default ASCII.");
    eprintln!("    -e          little-endian dump (incompatible with -ps,-i,-r).");
    eprintln!("    -F          label output with file name. Default off.");
    eprintln!("    -g bytes    number of octets per group in normal output. Default 2 (-e: 4).");
    eprintln!("    -h          print this summary.");
    eprintln!("    -i          output in C include file style.");
    eprintln!("    -l len      stop after <len> octets.");
    eprintln!("    -n name     set the variable name used in C include output (-i).");
    eprintln!("    -o off      add <off> to the displayed file position.");
    eprintln!("    -p          output in postscript plain hexdump style.");
    eprintln!("    -ps         output in postscript plain hexdump style (same as -p).");
    eprintln!("    -r          reverse operation: convert (or patch) hexdump into binary.");
    eprintln!("    -r -s off   revert with <off> added to file positions found in hexdump.");
    eprintln!("    -d          show offset in decimal instead of hex.");
    eprintln!("    -s [+][-]seek  start at <seek> bytes abs. (or +: rel.) infile offset.");
    eprintln!("    -u          use upper case hex letters.");
    eprintln!(
        "    -R when     colorize the output; <when> can be 'always', 'auto' or 'never'. Default: 'auto'."
    );
    eprintln!("    -v          show version: \"rsxxd {MY_VERSION}\".");
    process::exit(0);
}

fn print_version() -> ! {
    println!("rsxxd {MY_VERSION}");
    process::exit(0);
}

fn is_valid_var_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let first_char = name.chars().next().unwrap();
    if !(first_char.is_alphabetic() || first_char == '_') {
        return false;
    }

    name.chars().all(|c| (c.is_alphanumeric() || c == '_'))
}

fn extract_basename(path: &str) -> String {
    let path = Path::new(path);
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("rsxxd_dump");

    let mut result = String::new();
    let mut chars = file_stem.chars();

    if let Some(c) = chars.next() {
        if c.is_alphabetic() || c == '_' {
            result.push(c);
        } else {
            result.push('_');
        }
    } else {
        return "rsxxd_dump".to_string();
    }

    for c in chars {
        if c.is_alphanumeric() || c == '_' {
            result.push(c);
        } else {
            result.push('_');
        }
    }

    if result.is_empty() {
        "rsxxd_dump".to_string()
    } else {
        result
    }
}

fn parse_numeric_value(value: &str) -> XxdResult<u64> {
    if value.is_empty() {
        return Err(XxdError::InvalidArgument("Empty numeric value".to_string()));
    }

    if value.starts_with("0x") || value.starts_with("0X") {
        return u64::from_str_radix(&value[2..], 16)
            .map_err(|_| XxdError::InvalidArgument(format!("Invalid hex value: '{value}'")));
    }

    if value.starts_with('0') && value.len() > 1 {
        return u64::from_str_radix(&value[1..], 8)
            .map_err(|_| XxdError::InvalidArgument(format!("Invalid octal value: '{value}'")));
    }

    value
        .parse::<u64>()
        .map_err(|_| XxdError::InvalidArgument(format!("Invalid numeric value: '{value}'")))
}

fn parse_args() -> XxdResult<Options> {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        return Err(XxdError::ParseError(
            "Failed to get program arguments".to_string(),
        ));
    }

    let mut options = Options::default();
    let mut i = 1;
    let mut options_ended = false;

    while i < args.len() {
        let arg = &args[i];

        if options_ended || !arg.starts_with('-') || arg == "-" {
            if options.fin.is_none() {
                options.fin = Some(args[i].clone());
            } else if options.fout.is_none() {
                options.fout = Some(args[i].clone());
            } else {
                return Err(XxdError::InvalidArgument(format!(
                    "Too many arguments. Unexpected argument: '{}'",
                    args[i]
                )));
            }
        } else if arg == "--" {
            options_ended = true;
        } else if let Some(option) = arg.strip_prefix("--") {
            match option {
                "help" => print_usage(),
                "version" => print_version(),
                _ => {
                    return Err(XxdError::InvalidArgument(format!(
                        "Unknown option: '{arg}'"
                    )));
                }
            }
        } else if arg == "-ps" {
            options.postscript = true;
        } else {
            let chars_iter = arg.chars().skip(1);
            for c in chars_iter {
                match c {
                    'a' => {
                        options.autoskip = true;
                    }
                    'b' => {
                        options.bin_hex = true;
                    }
                    'C' => {
                        options.capitalize = true;
                    }
                    'c' => {
                        i += 1;
                        if i >= args.len() {
                            return Err(XxdError::InvalidArgument(
                                "Missing value for option -c (columns)".to_string(),
                            ));
                        }
                        options.cols = parse_numeric_value(&args[i])? as usize;
                        if options.cols == 0 {
                            return Err(XxdError::InvalidArgument(
                                "Invalid value for -c (columns): must be greater than zero"
                                    .to_string(),
                            ));
                        }
                        options.colsgiven = true;
                    }
                    'd' => {
                        options.decimal_offset = true;
                    }
                    'E' => {
                        options.ebcdic = true;
                    }
                    'e' => {
                        options.little_endian = true;
                    }
                    'F' => {
                        options.show_filename = true;
                    }
                    'g' => {
                        i += 1;
                        if i >= args.len() {
                            return Err(XxdError::InvalidArgument(
                                "Missing value for option -g (bytes per group)".to_string(),
                            ));
                        }
                        options.octspergrp = parse_numeric_value(&args[i])? as usize;
                        if options.octspergrp == 0 {
                            return Err(XxdError::InvalidArgument(
                                "Invalid value for -g (bytes per group): must be greater than zero"
                                    .to_string(),
                            ));
                        }
                        options.octspergrpgiven = true;
                    }
                    'h' => print_usage(),
                    'i' => {
                        options.include = true;
                    }
                    'l' => {
                        i += 1;
                        if i >= args.len() {
                            return Err(XxdError::InvalidArgument(
                                "Missing value for option -l (length)".to_string(),
                            ));
                        }
                        options.len = parse_numeric_value(&args[i])?;
                    }
                    'n' => {
                        i += 1;
                        if i >= args.len() {
                            return Err(XxdError::InvalidArgument(
                                "Missing value for option -n (variable name)".to_string(),
                            ));
                        }
                        if !is_valid_var_name(&args[i]) {
                            return Err(XxdError::InvalidArgument(format!(
                                "Invalid C variable name for -n: '{}'",
                                args[i]
                            )));
                        }
                        options.var_name = args[i].clone();
                    }
                    'o' => {
                        i += 1;
                        if i >= args.len() {
                            return Err(XxdError::InvalidArgument(
                                "Missing value for option -o (offset)".to_string(),
                            ));
                        }

                        let offset_arg = &args[i];
                        if let Some(stripped) = offset_arg.strip_prefix('+') {
                            options.add_offset = parse_numeric_value(stripped)?;
                        } else if let Some(stripped) = offset_arg.strip_prefix('-') {
                            let val = parse_numeric_value(stripped)?;
                            options.add_offset = u64::MAX - val + 1;
                        } else {
                            options.add_offset = parse_numeric_value(offset_arg)?;
                        }
                    }
                    'p' => {
                        options.postscript = true;
                    }
                    'R' => {
                        i += 1;
                        if i >= args.len() {
                            return Err(XxdError::InvalidArgument(
                                "Missing argument for option -R (color mode)".to_string(),
                            ));
                        }
                        let mode = &args[i];
                        if mode != "auto" && mode != "always" && mode != "never" {
                            return Err(XxdError::InvalidArgument(format!(
                                "Invalid color mode for -R: '{mode}'. Must be 'auto', 'always', or 'never'"
                            )));
                        }
                        options.color_mode = mode.to_string();
                    }
                    'r' => {
                        options.revert = true;
                    }
                    's' => {
                        i += 1;
                        if i >= args.len() {
                            return Err(XxdError::InvalidArgument(
                                "Missing value for option -s (seek position)".to_string(),
                            ));
                        }

                        let seek_arg = &args[i];
                        if let Some(stripped) = seek_arg.strip_prefix('+') {
                            let val = parse_numeric_value(stripped)?;
                            options.seek = val;
                        } else if let Some(stripped) = seek_arg.strip_prefix('-') {
                            let val = parse_numeric_value(stripped)?;
                            options.seek = val;
                            options.from_eof = true;
                        } else {
                            options.seek = parse_numeric_value(seek_arg)?;
                        }
                    }
                    'u' => {
                        options.upper_hex = true;
                    }
                    'v' => print_version(),
                    _ => {
                        return Err(XxdError::InvalidArgument(format!("Unknown option: -{c}")));
                    }
                }
            }
        }
        i += 1;
    }

    if options.include && !options.colsgiven {
        options.cols = 12;
    }
    if options.postscript && !options.colsgiven {
        options.cols = 30;
    }

    if options.bin_hex && !options.colsgiven {
        options.cols = 6;
    }

    if options.little_endian && !options.octspergrpgiven {
        options.octspergrp = 4;
    }

    if options.bin_hex && !options.octspergrpgiven {
        options.octspergrp = 1;
    }

    if options.bin_hex && options.postscript {
        return Err(XxdError::IncompatibleOptions(
            "Binary digit dump (-b) is incompatible with postscript output (-p)".to_string(),
        ));
    }

    if options.bin_hex && options.include {
        return Err(XxdError::IncompatibleOptions(
            "Binary digit dump (-b) is incompatible with C include output (-i)".to_string(),
        ));
    }

    if options.bin_hex && options.revert {
        return Err(XxdError::IncompatibleOptions(
            "Binary digit dump (-b) is incompatible with reverse operation (-r)".to_string(),
        ));
    }

    if options.little_endian && options.postscript {
        return Err(XxdError::IncompatibleOptions(
            "Little-endian dump (-e) is incompatible with postscript output (-p)".to_string(),
        ));
    }

    if options.little_endian && options.include {
        return Err(XxdError::IncompatibleOptions(
            "Little-endian dump (-e) is incompatible with C include output (-i)".to_string(),
        ));
    }

    if options.little_endian && options.revert {
        return Err(XxdError::IncompatibleOptions(
            "Little-endian dump (-e) is incompatible with reverse operation (-r)".to_string(),
        ));
    }

    if options.include && options.var_name == "rsxxd_dump" {
        if let Some(ref path) = options.fin {
            if path != "-" {
                options.var_name = extract_basename(path);
            }
        }
    }

    if let Some(ref path) = options.fin {
        if path != "-" {
            let path_obj = Path::new(path);
            if !path_obj.exists() {
                return Err(XxdError::FileNotFound(path.clone()));
            }
            if !path_obj.is_file() {
                return Err(XxdError::InvalidArgument(format!(
                    "'{path}' is not a regular file"
                )));
            }
            match File::open(path) {
                Ok(_) => {}
                Err(e) => {
                    return Err(XxdError::IoError(
                        e,
                        format!("Cannot access input file '{path}'"),
                    ));
                }
            }
        }
    }

    if let Some(ref path) = options.fout {
        if path != "-" {
            let path_obj = Path::new(path);
            if path_obj.exists() && path_obj.is_dir() {
                return Err(XxdError::InvalidArgument(format!(
                    "Output path '{path}' is a directory"
                )));
            }

            match File::create(path) {
                Ok(_) => {}
                Err(e) => {
                    return Err(XxdError::IoError(
                        e,
                        format!("Cannot create or write to output file '{path}'"),
                    ));
                }
            }
        }
    }

    Ok(options)
}

fn hex2bin(hex: char) -> u8 {
    let c = hex as u8;
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => 0,
    }
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn format_offset(offset: u64, decimal: bool, add_offset: u64) -> String {
    let total = offset + add_offset;
    if decimal {
        format!("{total:08}")
    } else {
        format!("{total:08x}")
    }
}

fn determine_color_usage(options: &mut Options, _outfile: &dyn Write) {
    match options.color_mode.as_str() {
        "always" => {
            options.use_color = true;
        }
        "never" => {
            options.use_color = false;
        }
        "auto" => {
            options.use_color = match options.fout {
                Some(ref name) if name != "-" => false,
                _ => io::stdout().is_terminal(),
            };
        }
        _ => {
            options.use_color = false;
        }
    }
}

fn open_input(filename: &Option<String>) -> XxdResult<Box<dyn Read>> {
    match filename {
        Some(name) if name != "-" => {
            let path = Path::new(name);
            if !path.exists() {
                return Err(XxdError::FileNotFound(name.clone()));
            }

            if !path.is_file() {
                return Err(XxdError::InvalidArgument(format!(
                    "'{name}' is not a regular file"
                )));
            }

            let file = match File::open(name) {
                Ok(f) => f,
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => {
                        return Err(XxdError::FileNotFound(name.clone()));
                    }
                    io::ErrorKind::PermissionDenied => {
                        return Err(XxdError::FilePermissionDenied(name.clone()));
                    }
                    _ => {
                        return Err(XxdError::IoError(
                            e,
                            format!("Failed to open input file '{name}'"),
                        ));
                    }
                },
            };

            Ok(Box::new(BufReader::new(file)))
        }
        _ => {
            let stdin = io::stdin();
            Ok(Box::new(BufReader::new(stdin)))
        }
    }
}

fn open_output(filename: &Option<String>) -> XxdResult<Box<dyn Write>> {
    match filename {
        Some(name) if name != "-" => {
            let path = Path::new(name);
            if path.exists() && path.is_dir() {
                return Err(XxdError::InvalidArgument(format!(
                    "Output path '{name}' is a directory"
                )));
            }

            let file = match File::create(name) {
                Ok(f) => f,
                Err(e) => match e.kind() {
                    io::ErrorKind::PermissionDenied => {
                        return Err(XxdError::FilePermissionDenied(name.clone()));
                    }
                    io::ErrorKind::NotFound => {
                        if let Some(parent) = path.parent() {
                            if !parent.exists() {
                                return Err(XxdError::FileNotFound(format!(
                                    "Parent directory for '{name}' doesn't exist"
                                )));
                            }
                        }
                        return Err(XxdError::IoError(
                            e,
                            format!("Failed to create output file '{name}'"),
                        ));
                    }
                    _ => {
                        return Err(XxdError::IoError(
                            e,
                            format!("Failed to create output file '{name}'"),
                        ));
                    }
                },
            };

            Ok(Box::new(BufWriter::new(file)))
        }
        _ => {
            let stdout = io::stdout();
            Ok(Box::new(BufWriter::new(stdout.lock())))
        }
    }
}

fn seek_input(infile: Box<dyn Read>, options: &Options) -> XxdResult<Box<dyn Read>> {
    if options.seek == 0 && !options.from_eof {
        return Ok(infile);
    }

    if options.fin.is_none() || options.fin.as_ref().unwrap() == "-" {
        if options.from_eof {
            return Err(XxdError::InvalidArgument(
                "Cannot seek from end of file when reading from standard input".to_string(),
            ));
        }

        let mut buffer = [0u8; 8192];
        let mut remaining = options.seek;
        let mut seekable = infile;

        while remaining > 0 {
            let to_read = std::cmp::min(remaining, buffer.len() as u64);
            let bytes_read = match seekable.read(&mut buffer[..to_read as usize]) {
                Ok(n) => n,
                Err(e) => {
                    return Err(XxdError::IoError(
                        e,
                        format!("Failed to seek {} bytes in input stream", options.seek),
                    ));
                }
            };

            if bytes_read == 0 {
                return Err(XxdError::IoError(
                    io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of file"),
                    format!(
                        "Input too short: cannot seek to offset {} (reached EOF after {} bytes)",
                        options.seek,
                        options.seek - remaining
                    ),
                ));
            }
            remaining -= bytes_read as u64;
        }

        Ok(seekable)
    } else {
        let name = options.fin.as_ref().unwrap();
        let mut file = match File::open(name) {
            Ok(f) => f,
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    format!("Failed to open input file '{name}' for seeking"),
                ));
            }
        };

        let file_size = match file.metadata() {
            Ok(meta) => meta.len(),
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    format!("Failed to get file size for '{name}'"),
                ));
            }
        };

        let seek_pos = if options.from_eof {
            if options.seek > file_size {
                return Err(XxdError::InvalidArgument(format!(
                    "Cannot seek backward by {} bytes from end of '{}', file size is only {} bytes",
                    options.seek, name, file_size
                )));
            }
            file_size - options.seek
        } else {
            if options.seek > file_size {
                return Err(XxdError::InvalidArgument(format!(
                    "Cannot seek to position {} in '{}', file size is only {} bytes",
                    options.seek, name, file_size
                )));
            }
            options.seek
        };

        match file.seek(SeekFrom::Start(seek_pos)) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    format!("Failed to seek to position {seek_pos} in '{name}'"),
                ));
            }
        }

        Ok(Box::new(BufReader::new(file)))
    }
}

fn hexdump(options: &mut Options) -> XxdResult<()> {
    let infile = open_input(&options.fin)?;
    let mut infile = seek_input(infile, options)?;
    let mut outfile = open_output(&options.fout)?;

    determine_color_usage(options, &*outfile);

    if options.revert {
        if options.include {
            return Err(XxdError::IncompatibleOptions(
                "Cannot revert this type of dump: C include style not supported in reverse mode"
                    .to_string(),
            ));
        }
        if options.little_endian {
            return Err(XxdError::IncompatibleOptions(
                "Cannot revert this type of dump: little-endian not supported in reverse mode"
                    .to_string(),
            ));
        }
        if options.bin_hex {
            return Err(
                XxdError::IncompatibleOptions(
                    "Cannot revert this type of dump: binary digit format not supported in reverse mode".to_string()
                )
            );
        }
        return hextobin(&mut infile, &mut outfile, options);
    }

    if options.include {
        return include_dump(&mut infile, &mut outfile, options);
    }

    if options.postscript {
        return postscript_dump(&mut infile, &mut outfile, options);
    }

    let mut buffer = vec![0u8; options.cols];
    let mut addr: u64 = options.seek;
    let mut total_read: u64 = 0;

    let mut in_zero_mode = false;
    let mut last_buffer_was_zeros = false;

    if options.show_filename && options.fin.is_some() {
        match writeln!(outfile, "# {}", options.fin.as_ref().unwrap()) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write filename header".to_string(),
                ));
            }
        }
    }

    let hex_table = if options.upper_hex {
        &HEX_UPPER
    } else {
        &HEX_LOWER
    };

    let mut line_buffer = String::with_capacity(256);

    loop {
        let mut bytes_read = match infile.read(&mut buffer) {
            Ok(n) => n,
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    format!("Failed to read from input at offset {addr}"),
                ));
            }
        };

        let mut is_last_chunk = bytes_read == 0 || total_read + (bytes_read as u64) >= options.len;

        if bytes_read == 0 {
            if options.autoskip && in_zero_mode && last_buffer_was_zeros {
                print_line(
                    &mut outfile,
                    &mut line_buffer,
                    &buffer[..options.cols.min(buffer.len())],
                    options.cols,
                    addr,
                    options,
                    hex_table,
                )?;
            }
            break;
        }

        total_read += bytes_read as u64;
        if total_read > options.len {
            bytes_read = ((bytes_read as u64) - (total_read - options.len)) as usize;
            is_last_chunk = true;
        }

        let all_zero = buffer[..bytes_read].iter().all(|&x| x == 0);
        last_buffer_was_zeros = all_zero;

        if options.autoskip && all_zero {
            if !in_zero_mode {
                match writeln!(outfile, "*") {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(XxdError::IoError(
                            e,
                            "Failed to write autoskip marker".to_string(),
                        ));
                    }
                }
                in_zero_mode = true;
            }

            if is_last_chunk {
                print_line(
                    &mut outfile,
                    &mut line_buffer,
                    &buffer[..bytes_read],
                    options.cols,
                    addr,
                    options,
                    hex_table,
                )?;
                in_zero_mode = false;
            }
        } else {
            in_zero_mode = false;
            print_line(
                &mut outfile,
                &mut line_buffer,
                &buffer[..bytes_read],
                options.cols,
                addr,
                options,
                hex_table,
            )?;
        }

        addr += bytes_read as u64;
    }

    match outfile.flush() {
        Ok(_) => {}
        Err(e) => {
            return Err(XxdError::IoError(
                e,
                "Failed to flush output buffer".to_string(),
            ));
        }
    }

    Ok(())
}

fn print_line<W: Write>(
    outfile: &mut W,
    line_buffer: &mut String,
    buffer: &[u8],
    cols: usize,
    addr: u64,
    options: &Options,
    hex_table: &[&str; 256],
) -> XxdResult<()> {
    line_buffer.clear();

    if options.use_color {
        line_buffer.push_str(COLOR_CODES.address);
        line_buffer.push_str(&format_offset(
            addr,
            options.decimal_offset,
            options.add_offset,
        ));
        line_buffer.push_str(": ");
        line_buffer.push_str(COLOR_CODES.reset);
    } else {
        line_buffer.push_str(&format_offset(
            addr,
            options.decimal_offset,
            options.add_offset,
        ));
        line_buffer.push_str(": ");
    }

    let bytes_read = buffer.len();

    if options.bin_hex {
        if options.use_color {
            line_buffer.push_str(COLOR_CODES.hex);
        }

        for &b in buffer {
            line_buffer.push_str(&format!("{b:08b} "));
        }

        for _ in bytes_read..cols {
            line_buffer.push_str("         ");
        }

        if options.use_color {
            line_buffer.push_str(COLOR_CODES.reset);
        }
    } else if options.little_endian {
        if options.use_color {
            line_buffer.push_str(COLOR_CODES.hex);
        }

        for chunk in buffer.chunks(options.octspergrp) {
            for &b in chunk.iter().rev() {
                line_buffer.push_str(hex_table[b as usize]);
            }
            line_buffer.push(' ');
        }

        let remaining = cols - bytes_read;
        let remaining_groups = remaining.div_ceil(options.octspergrp);
        for _ in 0..remaining_groups {
            for _ in 0..options.octspergrp * 2 {
                line_buffer.push(' ');
            }
            line_buffer.push(' ');
        }

        if options.use_color {
            line_buffer.push_str(COLOR_CODES.reset);
        }
    } else {
        if options.use_color {
            line_buffer.push_str(COLOR_CODES.hex);
        }

        buffer.iter().enumerate().for_each(|(i, &b)| {
            line_buffer.push_str(hex_table[b as usize]);
            if (i + 1) % options.octspergrp == 0 && i < bytes_read - 1 {
                line_buffer.push(' ');
            }
        });

        for i in bytes_read..cols {
            line_buffer.push_str("  ");
            if (i + 1) % options.octspergrp == 0 && i < cols - 1 {
                line_buffer.push(' ');
            }
        }

        if options.use_color {
            line_buffer.push_str(COLOR_CODES.reset);
        }
    }

    line_buffer.push_str("  ");

    if options.use_color {
        line_buffer.push_str(COLOR_CODES.ascii);
    }

    if options.ebcdic {
        buffer.iter().for_each(|&b| {
            let ebcdic_char = EBCDIC_TO_ASCII[b as usize];
            line_buffer.push(if ebcdic_char.is_ascii_graphic() || ebcdic_char == ' ' {
                ebcdic_char
            } else {
                '.'
            });
        });
    } else {
        buffer.iter().for_each(|&b| {
            line_buffer.push(if (32..=126).contains(&b) {
                b as char
            } else {
                '.'
            });
        });
    }

    if options.use_color {
        line_buffer.push_str(COLOR_CODES.reset);
    }

    match writeln!(outfile, "{line_buffer}") {
        Ok(_) => {}
        Err(e) => {
            return Err(XxdError::IoError(
                e,
                format!("Failed to write output line for offset {addr}"),
            ));
        }
    }

    Ok(())
}

fn hextobin<R: Read, W: Write>(
    infile: &mut R,
    outfile: &mut W,
    options: &Options,
) -> XxdResult<()> {
    let mut line = String::new();
    let mut line_reader = BufReader::new(infile);
    let mut decoded_bytes = Vec::with_capacity(4096);

    while let Ok(bytes) = line_reader.read_line(&mut line) {
        if bytes == 0 {
            break;
        }

        if line.trim().is_empty() || line.trim_start().starts_with('#') || line.trim() == "*" {
            line.clear();
            continue;
        }

        decoded_bytes.clear();

        if options.postscript {
            let hex_chars: Vec<char> = line.chars().filter(|c| is_hex_digit(*c)).collect();

            if hex_chars.len() >= 2 {
                for i in (0..hex_chars.len() - 1).step_by(2) {
                    let hi = hex2bin(hex_chars[i]);
                    let lo = hex2bin(hex_chars[i + 1]);
                    decoded_bytes.push((hi << 4) | lo);
                }
            }
        } else {
            if !line.contains(':') {
                line.clear();
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() < 2 {
                line.clear();
                continue;
            }

            let hex_part = parts[1];

            let ascii_separator_pos = hex_part.find("  ").unwrap_or(hex_part.len());

            let hex_only = &hex_part[..ascii_separator_pos];

            let hex_chars: Vec<char> = hex_only.chars().filter(|c| is_hex_digit(*c)).collect();

            if hex_chars.len() >= 2 {
                for i in (0..hex_chars.len() - 1).step_by(2) {
                    let hi = hex2bin(hex_chars[i]);
                    let lo = hex2bin(hex_chars[i + 1]);
                    decoded_bytes.push((hi << 4) | lo);
                }
            }
        }

        if !decoded_bytes.is_empty() {
            match outfile.write_all(&decoded_bytes) {
                Ok(_) => {}
                Err(e) => {
                    return Err(XxdError::IoError(
                        e,
                        "Failed to write binary data to output".to_string(),
                    ));
                }
            }
        }

        line.clear();
    }

    match outfile.flush() {
        Ok(_) => {}
        Err(e) => {
            return Err(XxdError::IoError(
                e,
                "Failed to flush output buffer".to_string(),
            ));
        }
    }

    Ok(())
}

fn postscript_dump<R: Read, W: Write>(
    infile: &mut R,
    outfile: &mut W,
    options: &Options,
) -> XxdResult<()> {
    let mut buffer = vec![0u8; options.cols];
    let mut total_read: u64 = 0;
    let mut line_count = 0;

    let mut output_buffer = String::with_capacity(options.cols * 3);

    let hex_table = if options.upper_hex {
        &HEX_UPPER
    } else {
        &HEX_LOWER
    };

    if options.use_color {
        match write!(outfile, "{}", COLOR_CODES.hex) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write color code".to_string(),
                ));
            }
        }
    }

    loop {
        let mut bytes_read = match infile.read(&mut buffer) {
            Ok(n) => n,
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    format!("Failed to read from input at offset {total_read}"),
                ));
            }
        };

        if bytes_read == 0 || total_read >= options.len {
            break;
        }

        total_read += bytes_read as u64;
        if total_read > options.len {
            bytes_read = ((bytes_read as u64) - (total_read - options.len)) as usize;
        }

        output_buffer.clear();
        for i in 0..bytes_read {
            output_buffer.push_str(hex_table[buffer[i] as usize]);

            line_count += 1;

            if line_count >= options.cols {
                output_buffer.push('\n');
                line_count = 0;
            } else {
                output_buffer.push(' ');
            }
        }

        match write!(outfile, "{output_buffer}") {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    format!(
                        "Failed to write hex data at offset {}",
                        total_read - (bytes_read as u64)
                    ),
                ));
            }
        }
    }

    if options.use_color {
        match write!(outfile, "{}", COLOR_CODES.reset) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write color reset".to_string(),
                ));
            }
        }
    }

    if line_count > 0 {
        match writeln!(outfile) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write final newline".to_string(),
                ));
            }
        }
    }

    match outfile.flush() {
        Ok(_) => {}
        Err(e) => {
            return Err(XxdError::IoError(
                e,
                "Failed to flush output buffer".to_string(),
            ));
        }
    }

    Ok(())
}

fn include_dump<R: Read, W: Write>(
    infile: &mut R,
    outfile: &mut W,
    options: &Options,
) -> XxdResult<()> {
    let mut buffer = vec![0u8; 4096];
    let mut addr: u64 = 0;
    let mut total_read: u64 = 0;

    if options.show_filename && options.fin.is_some() {
        match writeln!(
            outfile,
            "// Generated from: {}",
            options.fin.as_ref().unwrap()
        ) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write file header comment".to_string(),
                ));
            }
        }
    }

    let var_name = if options.capitalize {
        options.var_name.to_uppercase()
    } else {
        options.var_name.clone()
    };

    if options.use_color {
        match write!(
            outfile,
            "{}unsigned char{}",
            COLOR_CODES.address, COLOR_CODES.reset
        ) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write type declaration".to_string(),
                ));
            }
        }

        match write!(
            outfile,
            " {}{}{}[]",
            COLOR_CODES.ascii, var_name, COLOR_CODES.reset
        ) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write variable name".to_string(),
                ));
            }
        }

        match writeln!(outfile, " = {{") {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write opening brace".to_string(),
                ));
            }
        }
    } else {
        match writeln!(outfile, "unsigned char {var_name}[] = {{") {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write type declaration".to_string(),
                ));
            }
        }
    }

    let mut first_line = true;

    let mut line_buffer = String::with_capacity(options.cols * 8);

    let hex_table = if options.upper_hex {
        &HEX_UPPER
    } else {
        &HEX_LOWER
    };

    loop {
        let bytes_read = match infile.read(&mut buffer) {
            Ok(n) => n,
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    format!("Failed to read from input at offset {total_read}"),
                ));
            }
        };

        if bytes_read == 0 || total_read >= options.len {
            break;
        }

        total_read += bytes_read as u64;
        let adjusted_read = if total_read > options.len {
            ((bytes_read as u64) - (total_read - options.len)) as usize
        } else {
            bytes_read
        };

        line_buffer.clear();
        for i in 0..adjusted_read {
            if i % options.cols == 0 {
                if !first_line {
                    line_buffer.push_str(",\n  ");
                } else {
                    first_line = false;
                    line_buffer.push_str("  ");
                }
            } else {
                line_buffer.push_str(", ");
            }

            if options.use_color {
                line_buffer.push_str(COLOR_CODES.hex);
            }

            line_buffer.push_str("0x");
            line_buffer.push_str(hex_table[buffer[i] as usize]);

            if options.use_color {
                line_buffer.push_str(COLOR_CODES.reset);
            }
        }

        match write!(outfile, "{line_buffer}") {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    format!("Failed to write C array data at offset {addr}"),
                ));
            }
        }

        addr += adjusted_read as u64;
    }

    if options.use_color {
        match writeln!(outfile, "\n}};{}", COLOR_CODES.reset) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write closing brace".to_string(),
                ));
            }
        }

        match write!(
            outfile,
            "{}unsigned int{} ",
            COLOR_CODES.address, COLOR_CODES.reset
        ) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write length type".to_string(),
                ));
            }
        }

        match write!(
            outfile,
            "{}{}_len{} = ",
            COLOR_CODES.ascii, var_name, COLOR_CODES.reset
        ) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write length variable name".to_string(),
                ));
            }
        }

        match writeln!(outfile, "{}{}{};", COLOR_CODES.hex, addr, COLOR_CODES.reset) {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write length value".to_string(),
                ));
            }
        }
    } else {
        match writeln!(outfile, "\n}};\nunsigned int {var_name}_len = {addr};") {
            Ok(_) => {}
            Err(e) => {
                return Err(XxdError::IoError(
                    e,
                    "Failed to write length declaration".to_string(),
                ));
            }
        }
    }

    match outfile.flush() {
        Ok(_) => {}
        Err(e) => {
            return Err(XxdError::IoError(
                e,
                "Failed to flush output buffer".to_string(),
            ));
        }
    }

    Ok(())
}

fn main() {
    let mut options = match parse_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    if let Err(e) = hexdump(&mut options) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
