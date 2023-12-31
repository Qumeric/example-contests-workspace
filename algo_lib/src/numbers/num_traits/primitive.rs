pub trait Primitive<T>: Copy {
    fn to(self) -> T;
}
/// Provides a way to convert a value to another primitive type.
///
/// This trait is used to define a generic interface for numeric
/// conversions that doesn't depend on the standard library's
/// `From` and `Into` traits, allowing for more granular control
/// and optimization opportunities in algorithms.
macro_rules! primitive_one {
    ($t: ident, $($u: ident)+) => {$(
        impl Primitive<$u> for $t {
            fn to(self) -> $u {
                self as $u
            }
        }
    )+};
}

macro_rules! primitive {
    ($($t: ident)+) => {$(
        primitive_one!($t, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
    )+}
}

primitive!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
