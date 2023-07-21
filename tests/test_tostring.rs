use jkcenum_derive::JkcEnum;


#[derive(Debug, PartialEq, Eq, JkcEnum)]
enum JkcExample {
    Read,
    #[jenum(rename = "WRITE")]
    Write,
}


#[derive(Debug, PartialEq, Eq, JkcEnum)]
#[jenum(lowercase)]
enum JkcExample2 {
    Read,
    Write,
}


#[test]
fn test_tostring() {
    assert_eq!(JkcExample::Read.to_string(), "Read");
    assert_eq!(JkcExample::Write.to_string(), "WRITE");

    assert_eq!(JkcExample2::Read.to_string(), "read");
    assert_eq!(JkcExample2::Write.to_string(), "write");
}
