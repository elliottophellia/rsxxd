use std::env;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn get_binary_path() -> PathBuf {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let binary_name = if cfg!(windows) { "rsxxd.exe" } else { "rsxxd" };
    Path::new(&target_dir).join("release").join(binary_name)
}

fn run_cmd(args: &[&str]) -> Result<String, i32> {
    if args.is_empty() {
        eprintln!("No command provided");
        return Err(-1);
    }

    let binary_path = get_binary_path().to_string_lossy().into_owned();
    let mut cmd = Command::new(&binary_path);

    if args.len() > 1 {
        cmd.args(&args[1..]);
    }

    cmd.stderr(Stdio::null());
    let output = cmd.output().expect("failed to execute process");
    let exit_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    if exit_code != 0 {
        let mut cmd_str = binary_path;
        for &arg in args.iter().skip(1) {
            cmd_str.push(' ');
            cmd_str.push_str(arg);
        }
        eprintln!("Error running command {cmd_str}: exit code {exit_code}");
        return Err(exit_code);
    }
    Ok(stdout)
}

fn files_identical(file1: &str, file2: &str) -> bool {
    let data1 = fs::read(file1).unwrap_or_default();
    let data2 = fs::read(file2).unwrap_or_default();
    data1 == data2
}

fn create_test_file(fname: &str) -> u64 {
    let mut f = File::create(fname).expect("Failed to create test file");
    f.write_all(b"Hello, World!\n").unwrap();
    f.write_all(&[0u8, 255u8]).unwrap();
    f.write_all(b"Data: ").unwrap();
    f.write_all(&[1, 2, 3]).unwrap();
    f.write_all(&[0, 0, 0]).unwrap();
    f.write_all(b"End-Pattern").unwrap();
    f.write_all(&[0xAB, 0xCD, 0xEF]).unwrap();
    f.write_all(&[0xAA, 0xBB, 0xCC]).unwrap();
    f.write_all(b"Done!").unwrap();
    drop(f);
    let file_size = fs::metadata(fname).unwrap().len();
    println!("Created binary file '{fname}' ({file_size} bytes)");
    file_size
}

#[test]
fn test_standard_hex_dump() {
    let fname = "test_std_hex.bin";
    create_test_file(fname);

    let output = run_cmd(&["rsxxd", fname]).expect("Failed to run rsxxd");
    assert!(
        output.starts_with("00000000:"),
        "Standard hex dump should start with offset"
    );

    let _ = fs::remove_file(fname);
}

#[test]
fn test_reverse_round_trip() {
    let fname = "test_reverse.bin";
    create_test_file(fname);
    let dump_file = "rsxxd_dump.txt";
    let rev_file = "reversed.bin";

    let dump = run_cmd(&["rsxxd", fname]).expect("Failed to create hex dump");
    fs::write(dump_file, &dump).unwrap();

    run_cmd(&["rsxxd", "-r", dump_file, rev_file]).expect("Failed to reverse hex dump");
    assert!(
        files_identical(fname, rev_file),
        "Files should be identical after round trip"
    );

    let _ = fs::remove_file(fname);
    let _ = fs::remove_file(dump_file);
    let _ = fs::remove_file(rev_file);
}

#[test]
fn test_offset_skip_and_length() {
    let fname = "test_offset.bin";
    create_test_file(fname);

    let skip = 5;
    let length = 10;

    let output = run_cmd(&[
        "rsxxd",
        "-p",
        "-s",
        &skip.to_string(),
        "-l",
        &length.to_string(),
        fname,
    ])
    .expect("Failed to run rsxxd with offset and length");

    let mut f2 = File::open(fname).unwrap();
    f2.seek(SeekFrom::Start(skip)).unwrap();
    let mut buf = vec![0u8; length];
    let n = f2.read(&mut buf).unwrap();
    let expected: String = buf[..n].iter().map(|b| format!("{b:02x}")).collect();
    let actual: String = output.chars().filter(|c| !c.is_whitespace()).collect();

    assert_eq!(expected, actual, "Offset and length output should match");

    let _ = fs::remove_file(fname);
}

#[test]
fn test_negative_offset() {
    let fname = "test_neg_offset.bin";
    create_test_file(fname);

    let neg = -5;
    let output = run_cmd(&["rsxxd", "-p", "-s", &neg.to_string(), fname])
        .expect("Failed to run rsxxd with negative offset");

    let len = fs::metadata(fname).unwrap().len();
    let mut f3 = File::open(fname).unwrap();
    let start = len.saturating_sub(5);
    f3.seek(SeekFrom::Start(start)).unwrap();
    let mut tail = [0u8; 5];
    f3.read_exact(&mut tail).unwrap();
    let expected_tail: String = tail.iter().map(|b| format!("{b:02x}")).collect();
    let actual_tail: String = output.chars().filter(|c| !c.is_whitespace()).collect();

    assert_eq!(
        expected_tail, actual_tail,
        "Negative offset should read from file end"
    );

    let _ = fs::remove_file(fname);
}

#[test]
fn test_plain_hex() {
    let fname = "test_plain.bin";
    create_test_file(fname);

    let output = run_cmd(&["rsxxd", "-p", fname]).expect("Failed to run rsxxd in plain mode");

    let data = fs::read(fname).unwrap();
    let expected_all: String = data.iter().map(|b| format!("{b:02x}")).collect();
    let actual_all: String = output.chars().filter(|c| !c.is_whitespace()).collect();

    assert_eq!(expected_all, actual_all, "Plain hex output should match");

    let _ = fs::remove_file(fname);
}

#[test]
fn test_uppercase_hex() {
    let fname = "test_upper.bin";
    create_test_file(fname);

    let output = run_cmd(&["rsxxd", "-u", fname]).expect("Failed to run rsxxd in uppercase mode");

    let mut hex_data = String::new();
    for line in output.lines() {
        if line.len() >= 10 && line.chars().nth(8) == Some(':') {
            let after_offset = &line[10..];
            if let Some(pos) = after_offset.find("  ") {
                let hex_part = &after_offset[..pos];
                let hex_no_space: String =
                    hex_part.chars().filter(|c| !c.is_whitespace()).collect();
                hex_data.push_str(&hex_no_space);
            }
        }
    }

    assert!(
        !hex_data.chars().any(|c| c.is_ascii_lowercase()),
        "Uppercase hex should not contain lowercase letters"
    );

    let _ = fs::remove_file(fname);
}

#[test]
fn test_c_style_array() {
    let fname = "test_c_array.bin";
    let file_size = create_test_file(fname);

    let output = run_cmd(&["rsxxd", "-i", fname]).expect("Failed to run rsxxd in C array mode");

    assert!(
        output.contains("unsigned char"),
        "C array output should contain 'unsigned char'"
    );
    assert!(
        output.contains("unsigned int"),
        "C array output should contain 'unsigned int'"
    );

    let mut found_size = false;
    for line in output.lines() {
        if line.contains("_len =") {
            if let Some(idx) = line.find('=') {
                let num_str = line[idx + 1..].trim().trim_end_matches(';');
                if let Ok(val) = num_str.parse::<u64>() {
                    if val == file_size {
                        found_size = true;
                        break;
                    }
                }
            }
        }
    }

    assert!(found_size, "C array length should match file size");

    let _ = fs::remove_file(fname);
}
