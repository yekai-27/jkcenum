# JkcEnum

## Feature

- [x] to_string
- [x] from_str
- [x] to_vec

## Example

> from_str

```rust
use std::str::FromStr;
use jkcenum_derive::JkcEnum;



#[derive(Debug, PartialEq, Eq, JkcEnum)]
enum JkcExample {
    #[jenum(alias = "r", alias = "read")]
    Read,
    #[jenum(rename = "WRITE", alias = "w", alias = "write")]
    Write,
}


#[test]
fn test_fromstr() {
    assert_eq!(JkcExample::from_str("Read").unwrap(), JkcExample::Read);
    assert_eq!(JkcExample::from_str("r").unwrap(), JkcExample::Read);
    assert_eq!(JkcExample::from_str("read").unwrap(), JkcExample::Read);
    assert_eq!(JkcExample::from_str("WRITE").unwrap(), JkcExample::Write);
    assert_eq!(JkcExample::from_str("w").unwrap(), JkcExample::Write);
    assert_eq!(JkcExample::from_str("write").unwrap(), JkcExample::Write);
    assert_eq!(JkcExample::from_str("Write").is_err(), true);
}
```

> to_string

```rust
use jkcenum_derive::JkcEnum;


#[derive(Debug, PartialEq, Eq, JkcEnum)]
enum JkcExample {
    Read,
    #[jenum(rename = "WRITE")]
    Write,
}


#[test]
fn test_tostring() {
    assert_eq!(JkcExample::Read.to_string(), "Read");
    assert_eq!(JkcExample::Write.to_string(), "WRITE");
}
```

> to_vec

```rust
use jkcenum_derive::JkcEnum;


#[derive(Debug, Clone, Copy, PartialEq, Eq, JkcEnum)]
pub enum JkcExample {
    Read,
    #[jenum(rename = "WRITE")]
    Write,
}


#[test]
fn test_to_vec() {
    assert_eq!(JkcExample::to_vec(), vec![
        JkcExample::Read,
        JkcExample::Write,
    ]);
}
```