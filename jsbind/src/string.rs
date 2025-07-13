use crate::utils::bind;

macro_rules! declare_string {
    ($name:ident) => {
        #[doc = concat!("Wrapper for WebIDL `", stringify!($name), "`.\n\n")]
        #[derive(Clone, Debug)]
        pub struct $name {
            inner: emlite::Val,
        }

        bind!($name);

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                if std::any::TypeId::of::<$name>() == std::any::TypeId::of::<ByteString>() {
                    assert!(s.is_ascii(), "ByteString must be ASCII/Latin-1");
                }
                emlite::Val::from(s).as_::<Self>()
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                emlite::Val::from(&s).as_::<Self>()
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl $name {
            /// Number of UTF‑8 bytes (same as `str::len()`).
            pub fn byte_len(&self) -> usize {
                self.as_str().len()
            }
            /// `len() == 0` convenience.
            pub fn is_empty(&self) -> bool {
                self.length() == 0
            }
            /// Scalar lookup; returns `None` if index is out of bounds.
            pub fn char_at(&self, i: usize) -> Option<char> {
                self.as_str().chars().nth(i)
            }
            /// Borrow the JavaScript string as `&str` (UTF‑8 view).
            pub fn as_str(&self) -> &str {
                self.inner.as_::<&str>()
            }
            
            /// Number of UTF-16 code units (`JSString.length`).
            pub fn length(&self) -> usize {
                self.inner.get("length").as_::<usize>()
            }

            /// Returns the 16-bit code unit at `idx` (like `charCodeAt`).
            pub fn char_code_at(&self, idx: usize) -> u16 {
                self.inner.call("charCodeAt", &[idx.into()]).as_::<u16>()
            }

            pub fn set(&self, idx: usize, val: char)
            {
                self.inner.set(idx, val as u8);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                self.call("concat", &[rhs.into()]).as_::<Self>()
            }
        }

        impl PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }
    };
}

declare_string!(ByteString);
declare_string!(DOMString);
declare_string!(CSSOMString);
declare_string!(USVString);
