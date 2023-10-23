# JkcEnum

## Feature

- [x] from_str
- [x] from_int
- [x] to_string
- [x] to_vec

## Example

> Cargo.toml

```conf
[dependencies]
jkcenum = { git = "https://github.com/caizhengxin/jkcenum.git", features = ["derive"] }
```

> from_str

```rust
use std::str::FromStr;
use jkcenum::JkcEnum;



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

> from_int

```rust
use jkcenum::JkcEnum;
use jkcenum::FromInt;


#[derive(Debug, PartialEq, Eq, JkcEnum)]
pub enum JkcExample {
    Read = 0x01,
    ReadWrite,
    Write = 0x03,
    #[jenum(range="4..=6")]
    Test,
    Test2 = 7,
    #[jenum(default)]
    Unknown,
}

#[test]
fn test_fromint() {
    assert_eq!(JkcExample::from_int(1).unwrap(), JkcExample::Read);
    assert_eq!(JkcExample::from_int(2).unwrap(), JkcExample::ReadWrite);
    assert_eq!(JkcExample::from_int(3).unwrap(), JkcExample::Write);
    assert_eq!(JkcExample::from_int(4).unwrap(), JkcExample::Test);
    assert_eq!(JkcExample::from_int(5).unwrap(), JkcExample::Test);
    assert_eq!(JkcExample::from_int(6).unwrap(), JkcExample::Test);
    assert_eq!(JkcExample::from_int(7).unwrap(), JkcExample::Test2);
    assert_eq!(JkcExample::from_int(8).unwrap(), JkcExample::Unknown);
    assert_eq!(JkcExample::from_int(9).unwrap(), JkcExample::Unknown);
}
```

> to_string

```rust
use jkcenum::JkcEnum;


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

```rust
use jkcenum::JkcEnum;


#[derive(Debug, PartialEq, Eq, JkcEnum)]
#[jenum(rename_all="lowercase")]
enum JkcExample {
    JanKinCai,
}


#[test]
fn test_tostring() {
    assert_eq!(JkcExample::JanKinCai.to_string(), "jankincai");
}
```


```rust
use jkcenum::JkcEnum;


#[derive(Debug, PartialEq, Eq, JkcEnum)]
#[jenum(rename_all="desc")]
enum JkcExample {
    JanKinCai,
}


#[test]
fn test_tostring() {
    assert_eq!(JkcExample::JanKinCai.to_string(), "Jan Kin Cai");
}
```

> to_vec

```rust
use jkcenum::JkcEnum;


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

> serde

- [x] rename
- [x] alias
- [x] lowercase
- [x] uppercase

```rust
use std::str::FromStr;

use jkcenum::JkcEnum;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Eq, JkcEnum, Serialize, Deserialize)]
enum JkcExample {
    Read,
    #[serde(rename="WRITE")]
    Write,
}


#[derive(Debug, PartialEq, Eq, JkcEnum, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
enum JkcExample2 {
    Read,
    Write,
}


#[test]
fn test_serde_tostring() {
    assert_eq!(JkcExample::Read.to_string(), "Read");
    assert_eq!(JkcExample::Write.to_string(), "WRITE");
    assert_eq!(JkcExample::from_str("Read").unwrap(), JkcExample::Read);
    assert_eq!(JkcExample::from_str("WRITE").unwrap(), JkcExample::Write);

    assert_eq!(JkcExample2::Read.to_string(), "read");
    assert_eq!(JkcExample2::Write.to_string(), "write");
    assert_eq!(JkcExample2::from_str("read").unwrap(), JkcExample2::Read);
    assert_eq!(JkcExample2::from_str("write").unwrap(), JkcExample2::Write);
}
```