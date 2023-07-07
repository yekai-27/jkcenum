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
