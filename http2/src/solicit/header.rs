use std::str;
use std::fmt;
use std::borrow::Cow;
use std::borrow::Borrow;

use bytes::Bytes;

use misc::BsDebug;

/// A convenience struct representing a part of a header (either the name or the value) that can be
/// either an owned or a borrowed byte sequence.
pub struct HeaderPart(Bytes);

impl fmt::Debug for HeaderPart {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "b\"")?;
        let u8a: &[u8] = self.0.borrow();
        for &c in u8a {
            // ASCII printable
            if c >= 0x20 && c < 0x7f {
                write!(fmt, "{}", c as char)?;
            } else {
                write!(fmt, "\\x{:02x}", c)?;
            }
        }
        write!(fmt, "\"")?;
        Ok(())
    }
}

impl From<Vec<u8>> for HeaderPart {
    fn from(vec: Vec<u8>) -> HeaderPart {
        HeaderPart(Bytes::from(vec))
    }
}

impl<'a> From<&'a [u8]> for HeaderPart {
    fn from(buf: &'a [u8]) -> HeaderPart {
        HeaderPart(Bytes::from(buf))
    }
}

impl<'a> From<Cow<'a, [u8]>> for HeaderPart {
    fn from(cow: Cow<'a, [u8]>) -> HeaderPart {
        HeaderPart(Bytes::from(cow.into_owned()))
    }
}

macro_rules! from_static_size_array {
    ($N:expr) => (
        impl<'a> From<&'a [u8; $N]> for HeaderPart {
            fn from(buf: &'a [u8; $N]) -> HeaderPart {
                buf[..].into()
            }
        }
    );
}

macro_rules! impl_from_static_size_array {
    ($($N:expr,)+) => {
        $(
            from_static_size_array!($N);
        )+
    }
}

impl_from_static_size_array!(
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
);

impl From<String> for HeaderPart {
    fn from(s: String) -> HeaderPart {
        From::from(s.into_bytes())
    }
}

impl<'a> From<&'a str> for HeaderPart {
    fn from(s: &'a str) -> HeaderPart {
        From::from(s.as_bytes())
    }
}

impl<'a> From<Cow<'a, str>> for HeaderPart {
    fn from(cow: Cow<'a, str>) -> HeaderPart {
        From::from(cow.into_owned())
    }
}

#[derive(Clone, PartialEq)]
pub struct Header {
    pub name: Bytes,
    pub value: Bytes,
}

impl Header {
    /// Creates a new `Header` with the given name and value.
    ///
    /// The name and value need to be convertible into a `HeaderPart`.
    pub fn new<N: Into<HeaderPart>, V: Into<HeaderPart>>(name: N,
                                                                 value: V)
                                                                 -> Header {
        Header {
            name: name.into().0,
            value: value.into().0,
        }
    }

    /// Return a borrowed representation of the `Header` name.
    pub fn name(&self) -> &[u8] {
        &self.name
    }
    /// Return a borrowed representation of the `Header` value.
    pub fn value(&self) -> &[u8] {
        &self.value
    }
}

impl<N: Into<HeaderPart>, V: Into<HeaderPart>> From<(N, V)> for Header {
    fn from(p: (N, V)) -> Header {
        Header::new(p.0, p.1)
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "Header {{ name: {:?}, value: {:?} }}",
            BsDebug(self.name()), BsDebug(self.value()))
    }
}


#[derive(Default)]
pub struct Headers(pub Vec<Header>);

impl Headers {
    pub fn new() -> Headers {
        Default::default()
    }

    pub fn get<'a>(&'a self, name: &str) -> &'a str {
        str::from_utf8(&self.0.iter().filter(|&h| h.name() == name.as_bytes()).next().unwrap().value()).unwrap()
    }

    pub fn add(&mut self, name: &str, value: &str) {
        self.0.push(Header::new(name, value));
    }
}