use crate::utils::bind;

macro_rules! declare_string {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name {
            inner: emlite::Val,
        }

        bind!($name);

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                emlite::Val::from(s).as_::<Self>()
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                emlite::Val::from(&s).as_::<Self>()
            }
        }

        impl $name {
            pub fn len(&self) -> usize {
                self.as_str().len()
            }
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }
            pub fn char_at(&self, i: usize) -> Option<char> {
                self.as_str().chars().nth(i)
            }

            pub fn as_str(&self) -> &str {
                self.inner.as_::<&str>()
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
