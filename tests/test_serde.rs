use std::str::FromStr;

use jkcenum_derive::JkcEnum;
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
