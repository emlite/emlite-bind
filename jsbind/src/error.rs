use crate::utils::*;
use alloc::string::String;
use emlite::{FromJsError, Val};

macro_rules! declare_error {
    ($base:ident $(, $name:ident)*) => {
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $base { inner: emlite::Val }
        bind!($base);
        impl_dyn_cast!($base);

        impl $base {
            /// `new Error(message?)`
            pub fn new(msg: &str) -> Self {
                emlite::Val::global(stringify!($base))
                    .new(&[msg.into()])
                    .as_::<Self>()
            }
            /// JS `err.message`
            pub fn message(&self) -> Option<String>  { self.inner.get("message").as_() }
            /// JS `err.name`
            pub fn name(&self)    -> Option<String>  { self.inner.get("name").as_() }
            /// JS `err.stack` (may be `undefined`)
            pub fn stack(&self)   -> Option<String> {
                let s = self.inner.get("stack");
                if s.is_undefined() { None } else { s.as_() }
            }
        }

        impl core::fmt::Display for $base {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:?}: {:?}", self.name(), self.message())
            }
        }
        impl core::error::Error for $base {}

        /// Implement FromJsError for the base error type to enable Result<T, Error> conversions
        impl FromJsError for $base {
            fn from_js_error(val: &Val) -> Self {
                val.as_::<Self>()
            }
        }

        $(
            #[derive(Clone, Debug, PartialEq, PartialOrd)]
            #[repr(transparent)]
            pub struct $name { inner: emlite::Val }
            bind!($name);
            impl_dyn_cast!($name);

            impl $name {
                /// Construct `new $Name(message?)`.
                pub fn new(msg: &str) -> Self {
                    emlite::Val::global(stringify!($name))
                        .new(&[msg.into()])
                        .as_::<Self>()
                }
                // Re-export the common helpers by delegation.
                pub fn message(&self) -> Option<String>        { <$base>::from(self.clone()).message() }
                pub fn name(&self)    -> Option<String>        { <$base>::from(self.clone()).name() }
                pub fn stack(&self)   -> Option<String>{ <$base>::from(self.clone()).stack() }
            }
            impl core::fmt::Display for $name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{:?}: {:?}", self.name(), self.message())
                }
            }
            impl core::error::Error for $name {}
            impl From<$name> for $base {
                fn from(e: $name) -> $base { e.inner.clone().as_::<$base>() }
            }

            /// Implement FromJsError for derived error types
            impl FromJsError for $name {
                fn from_js_error(val: &Val) -> Self {
                    val.as_::<Self>()
                }
            }
        )*
    };
}

declare_error!(
    JsError,
    EvalError,
    RangeError,
    ReferenceError,
    SyntaxError,
    TypeError,
    URIError,
    AggregateError
);
