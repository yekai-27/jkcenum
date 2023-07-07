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
