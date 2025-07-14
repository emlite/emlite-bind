use crate::utils::bind;
use emlite::FromVal;

/// Macro to generate `struct RangeError { inner: Val }` plus boilerplate.
macro_rules! declare_error {
    ($base:ident $(, $name:ident)*) => {
        #[derive(Clone, Debug)]
        pub struct $base { inner: emlite::Val }
        bind!($base);

        impl $base {
            /// `new Error(message?)`
            pub fn new(msg: &str) -> Self {
                emlite::Val::global(stringify!($base))
                    .new(&[msg.into()])
                    .as_::<Self>()
            }
            /// JS `err.message`
            pub fn message(&self) -> String  { self.inner.get("message").as_() }
            /// JS `err.name`
            pub fn name(&self)    -> String  { self.inner.get("name").as_() }
            /// JS `err.stack` (may be `undefined`)
            pub fn stack(&self)   -> Option<String> {
                let s = self.inner.get("stack");
                if s.as_handle() == 1 { None } else { Some(s.as_()) }
            }
        }

        impl std::fmt::Display for $base {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", self.name(), self.message())
            }
        }
        impl std::error::Error for $base {}

        $(
            #[derive(Clone, Debug)]
            pub struct $name { inner: emlite::Val }
            bind!($name);

            impl $name {
                /// Construct `new $Name(message?)`.
                pub fn new(msg: &str) -> Self {
                    emlite::Val::global(stringify!($name))
                        .new(&[msg.into()])
                        .as_::<Self>()
                }
                // Re-export the common helpers by delegation.
                pub fn message(&self) -> String        { <$base>::from(self.clone()).message() }
                pub fn name(&self)    -> String        { <$base>::from(self.clone()).name() }
                pub fn stack(&self)   -> Option<String>{ <$base>::from(self.clone()).stack() }
            }
            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}: {}", self.name(), self.message())
                }
            }
            impl std::error::Error for $name {}
            impl From<$name> for $base {
                fn from(e: $name) -> $base { e.inner.clone().as_::<$base>() }
            }
        )*
    };
}

declare_error!(
    Error,
    EvalError,
    RangeError,
    ReferenceError,
    SyntaxError,
    TypeError,
    URIError,
    AggregateError
);
