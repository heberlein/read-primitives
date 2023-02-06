/*!
read-primitives provides traits to read primitive types from any type that implements [std::io::Read](https://doc.rust-lang.org/std/io/trait.Read.html)

 # Examples

```
    use read_primitives::ReadF64;
    let bytes: [u8; 8] = [24, 45, 68, 84, 251, 33, 9, 64];
    let float = bytes.as_slice().read_le_f64().unwrap();
    assert_eq!(std::f64::consts::PI, float)
```
*/

use std::io::{self, Read};

macro_rules! impl_traits {
    ($($type:ty),+) => {
        $(
            ::paste::paste!{
                #[doc = "Trait to read "$type "."]
                pub trait [<Read $type:camel>]: Read {
                    #[doc = "Read " $type "in native byte order"]
                    fn [<read_ne_  $type>](&mut self) -> io::Result<$type> {
                        let mut bytes = [0u8; std::mem::size_of::<$type>()];
                        self.read_exact(&mut bytes)?;
                        Ok($type::from_ne_bytes(bytes))
                    }
                    #[doc = "Read " $type "in little endian byte order"]
                    fn [<read_le_  $type>](&mut self) -> io::Result<$type> {
                        let mut bytes = [0u8; std::mem::size_of::<$type>()];
                        self.read_exact(&mut bytes)?;
                        Ok($type::from_le_bytes(bytes))
                    }
                    #[doc = "Read " $type "in big endian byte order"]
                    fn [<read_be_  $type>](&mut self) -> io::Result<$type> {
                        let mut bytes = [0u8; std::mem::size_of::<$type>()];
                        self.read_exact(&mut bytes)?;
                        Ok($type::from_be_bytes(bytes))
                    }
                }
                impl<T:Read> [<Read $type:camel>] for T{}
            }
        )+
    };
}

impl_traits!(u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

/// Trait to read u8
pub trait ReadU8: Read {
    /// Read a u8
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut bytes = [0u8; 1];
        self.read_exact(&mut bytes)?;
        Ok(u8::from_ne_bytes(bytes))
    }
}
impl<R> ReadU8 for R where R: Read {}

/// Trait to read char
pub trait ReadChar: Read {
    /// Read a char
    ///  It is a assumed that the char is represented in native yte order
    fn read_char(&mut self) -> io::Result<Option<char>> {
        let mut bytes = [0u8; 4];
        self.read_exact(&mut bytes)?;
        Ok(char::from_u32(u32::from_ne_bytes(bytes)))
    }
}
impl<R> ReadChar for R where R: Read {}

/// Trait to read char
pub trait ReadBool: Read {
    /// Read a bool
    fn read_bool(&mut self) -> io::Result<bool> {
        let mut bytes = [0u8; 1];
        self.read_exact(&mut bytes)?;
        Ok(u8::from_ne_bytes(bytes) != 0)
    }
}
impl<R> ReadBool for R where R: Read {}

#[cfg(test)]
mod test {
    use crate::*;

    macro_rules! impl_tests {
        ($($type:ty),+) => {
            $(
                ::paste::paste! {
                    #[test]
                    fn [<read_ne_ $type>]() {
                        let bytes = [<37 $type>].to_ne_bytes();
                        let number = bytes.as_slice().[<read_ne_ $type>]().unwrap();
                        assert_eq!([<37 $type>], number)
                    }
                }
                ::paste::paste! {
                    #[test]
                    fn [<read_le_ $type>]() {
                        let bytes = [<37 $type>].to_le_bytes();
                        let number = bytes.as_slice().[<read_le_ $type>]().unwrap();
                        assert_eq!([<37 $type>], number)
                    }
                }
                ::paste::paste! {
                    #[test]
                    fn [<read_be_ $type>]() {
                        let bytes = [<37 $type>].to_be_bytes();
                        let number = bytes.as_slice().[<read_be_ $type>]().unwrap();
                        assert_eq!([<37 $type>], number)
                    }
                }
            )+
        };
    }
    impl_tests!(u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

    #[test]
    fn read_u8() {
        let bytes = 37u8.to_ne_bytes();
        let byte = bytes.as_slice().read_u8().unwrap();
        assert_eq!(37, byte)
    }
}
