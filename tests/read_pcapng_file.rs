//! Integration tests for the public function `read_pcapng_file`.

use rusty_pcap::read_pcapng_file;

#[test]
fn test_read_pcapng_file_success() {
    // Test with a valid pcapng file
    let result = read_pcapng_file("tests/resources/test.pcapng");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 37);
}

#[test]
fn test_read_pcapng_file_not_found() {
    // Test with a non-existing file
    let result = read_pcapng_file("non_existing_file.pcapng");
    assert!(result.is_err());
}

#[test]
fn test_read_pcapng_file_invalid_format() {
    // Test with an invalid pcapng file
    let result = read_pcapng_file("tests/resources/invalid.pcapng");
    assert!(result.is_err());
}

#[test]
fn test_read_pcapng_file_empty() {
    // Test with an empty pcapng file
    let result = read_pcapng_file("tests/resources/empty.pcapng");
    assert!(result.is_err());
}
