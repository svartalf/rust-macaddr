use assert_matches::assert_matches;

use super::Parser;
use crate::MacAddr;

#[test]
fn test_parse_v6_canonical_format() {
    let mut parser = Parser::new("12-34-56-78-9A-BC");
    let addr = parser.read_v6_addr();

    assert!(addr.is_ok());
    let addr = addr.unwrap();

    assert_eq!(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC], addr.as_bytes());
}

#[test]
fn test_parse_v6_colon_format() {
    let mut parser = Parser::new("12:34:56:78:9A:BC");
    let addr = parser.read_v6_addr();

    assert!(addr.is_ok());
    let addr = addr.unwrap();

    assert_eq!(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC], addr.as_bytes());
}

#[test]
fn test_parse_v6_cisco_format() {
    let mut parser = Parser::new("1234.5678.9ABC");
    let addr = parser.read_v6_addr();

    assert!(addr.is_ok());
    let addr = addr.unwrap();

    assert_eq!(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC], addr.as_bytes());
}

#[test]
fn test_parse_v8_canonical_format() {
    let mut parser = Parser::new("12-34-56-78-9A-BC-DE-F0");
    let addr = parser.read_v8_addr();

    assert!(addr.is_ok());
    let addr = addr.unwrap();

    assert_eq!(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0], addr.as_bytes());
}

#[test]
fn test_parse_v8_colon_format() {
    let mut parser = Parser::new("12:34:56:78:9A:BC:DE:F0");
    let addr = parser.read_v8_addr();

    assert!(addr.is_ok());
    let addr = addr.unwrap();

    assert_eq!(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0], addr.as_bytes());
}

#[test]
fn test_parse_canonical_format() {
    let mut parser = Parser::new("12-34-56-78-9A-BC-DE-F0");
    let addr = parser.read_addr();

    assert!(addr.is_ok());
    let addr = addr.unwrap();
    assert_matches!(addr, MacAddr::V8(..));
    assert_eq!(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0], addr.as_bytes());
}

#[test]
fn test_parse_colon_format() {
    let mut parser = Parser::new("12:34:56:78:9A:BC:DE:F0");
    let addr = parser.read_addr();

    assert!(addr.is_ok());
    let addr = addr.unwrap();
    assert_matches!(addr, MacAddr::V8(..));
    assert_eq!(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0], addr.as_bytes());
}

#[test]
fn test_parse_v6_empty() {
    let mut parser = Parser::new("");
    let addr = parser.read_v6_addr();

    assert!(addr.is_err());
}

#[test]
fn test_parse_v8_empty() {
    let mut parser = Parser::new("");
    let addr = parser.read_v8_addr();

    assert!(addr.is_err());
}

#[test]
fn test_parse_empty() {
    let mut parser = Parser::new("");
    let addr = parser.read_addr();

    assert!(addr.is_err());
}
