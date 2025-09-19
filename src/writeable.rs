use std::borrow::Cow;
use std::fmt::Debug;

pub trait Writeable: Copy + Debug {
    fn as_bytes(&self) -> Cow<[u8]>;
    fn is_empty(&self) -> bool;
}

impl Writeable for &str {
    fn as_bytes(&self) -> Cow<[u8]> {
        Cow::Borrowed(str::as_bytes(self))
    }
    
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

impl Writeable for char {
    fn as_bytes(&self) -> Cow<[u8]> {
        let mut buf = [0u8; 4]; // Max UTF-8 bytes for a char
        let encoded = self.encode_utf8(&mut buf);
        Cow::Owned(encoded.as_bytes().to_vec())
    }

    fn is_empty(&self) -> bool {
        false
    }
}
