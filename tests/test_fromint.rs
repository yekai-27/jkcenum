#![feature(exclusive_range_pattern)]
use jkcenum_derive::JkcEnum;
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

// #[jenum(default)]

// impl jkcenum::FromInt for JkcExample
// {
//     type Err = jkcenum::errors::FromIntParseError;

//     fn from_int(v: isize) -> Result <Self, Self::Err>
//     {
//         match v
//         {
//             1 => Ok(Self::Read),
//             2 => Ok(Self::ReadWrite),
//             3 => Ok(Self::Write),
//             4..=6 => Ok(Self::Test),
//             7 => Ok(Self::Test2),
//             _ => Ok(Self::Unknown),
//         }
//     }
// }


// not modifier #[jenum(default)]


// impl jkcenum::FromInt for JkcExample
// {
//     type Err = jkcenum::errors::FromIntParseError;

//     fn from_int(v: isize) -> Result <Self, Self::Err>
//     {
//         match v
//         {
//             1 => Ok(Self::Read),
//             2 => Ok(Self::ReadWrite),
//             3 => Ok(Self::Write),
//             4..=6 => Ok(Self::Test),
//             7 => Ok(Self::Test2),
//             _ => Err(Self::Err::InvalidInt(v)),
//         }
//     }
// }


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
