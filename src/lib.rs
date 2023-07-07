pub mod errors;


pub trait FromInt {
    type Err;

    fn from_int(v: isize) -> Result<Self, Self::Err>
        where
            Self: Sized,
    ;
}
