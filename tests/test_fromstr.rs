use std::str::FromStr;
use jkcenum_derive::JkcEnum;



#[derive(Debug, PartialEq, Eq, JkcEnum)]
enum JkcExample {
    #[jenum(alias = "r", alias = "read")]
    Read,
    #[jenum(rename = "WRITE", alias = "w", alias = "write")]
    Write,
}


#[derive(Debug, PartialEq, Eq, JkcEnum)]
#[jenum(uppercase)]
enum JkcExample2 {
    Read,
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

    assert_eq!(JkcExample2::from_str("READ").unwrap(), JkcExample2::Read);
    assert_eq!(JkcExample2::from_str("Read").is_err(), true);
    assert_eq!(JkcExample2::from_str("WRITE").unwrap(), JkcExample2::Write);
    assert_eq!(JkcExample2::from_str("Write").is_err(), true);
}
