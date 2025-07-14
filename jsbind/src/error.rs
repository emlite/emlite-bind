use crate::Any;
use crate::utils::bind;
use alloc::string::String;
use emlite::FromVal;

/// Macro to generate `struct RangeError { inner: Val }` plus boilerplate.
macro_rules! declare_error {
    ($base:ident $(, $name:ident)*) => {
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
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
                if s.is_undefined() { None } else { Some(s.as_()) }
            }
        }

        impl core::fmt::Display for $base {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}: {}", self.name(), self.message())
            }
        }
        impl core::error::Error for $base {}

        $(
            #[derive(Clone, Debug, PartialEq, PartialOrd)]
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
            impl core::fmt::Display for $name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{}: {}", self.name(), self.message())
                }
            }
            impl core::error::Error for $name {}
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

/// Throws a JS exception.
#[cold]
#[inline(never)]
pub fn throw_str(s: &str) -> ! {
    throw_val(s.into())
}

/// Throws a JS exception
#[cold]
#[inline(never)]
pub fn throw_val(s: Any) -> ! {
    unsafe {
        let handle = s.as_handle();
        core::mem::forget(s);
        emlite::env::emlite_val_throw(handle)
    }
}

// Implementations copied from wasm-bindgen
pub trait UnwrapThrowExt<T>: Sized {
    fn unwrap_throw(self) -> T {
        let loc = core::panic::Location::caller();
        let msg = alloc::format!(
            "called `{}::unwrap_throw()` ({}:{}:{})",
            core::any::type_name::<Self>(),
            loc.file(),
            loc.line(),
            loc.column()
        );
        self.expect_throw(&msg)
    }

    fn expect_throw(self, message: &str) -> T;
}

// Implementations copied from wasm-bindgen
impl<T> UnwrapThrowExt<T> for Option<T> {
    fn unwrap_throw(self) -> T {
        const MSG: &str = "called `Option::unwrap_throw()` on a `None` value";
        if let Some(val) = self {
            val
        } else if cfg!(debug_assertions) {
            let loc = core::panic::Location::caller();
            let msg = alloc::format!("{} ({}:{}:{})", MSG, loc.file(), loc.line(), loc.column(),);

            throw_str(&msg)
        } else {
            throw_str(MSG)
        }
    }

    fn expect_throw(self, message: &str) -> T {
        if let Some(val) = self {
            val
        } else if cfg!(debug_assertions) {
            let loc = core::panic::Location::caller();
            let msg = alloc::format!(
                "{} ({}:{}:{})",
                message,
                loc.file(),
                loc.line(),
                loc.column(),
            );

            throw_str(&msg)
        } else {
            throw_str(message)
        }
    }
}

// Implementations copied from wasm-bindgen
impl<T, E> UnwrapThrowExt<T> for Result<T, E>
where
    E: core::fmt::Debug,
{
    fn unwrap_throw(self) -> T {
        const MSG: &str = "called `Result::unwrap_throw()` on an `Err` value";
        match self {
            Ok(val) => val,
            Err(err) => {
                if cfg!(debug_assertions) {
                    let loc = core::panic::Location::caller();
                    let msg = alloc::format!(
                        "{} ({}:{}:{}): {:?}",
                        MSG,
                        loc.file(),
                        loc.line(),
                        loc.column(),
                        err
                    );

                    throw_str(&msg)
                } else {
                    throw_str(MSG)
                }
            }
        }
    }

    fn expect_throw(self, message: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                if cfg!(debug_assertions) {
                    let loc = core::panic::Location::caller();
                    let msg = alloc::format!(
                        "{} ({}:{}:{}): {:?}",
                        message,
                        loc.file(),
                        loc.line(),
                        loc.column(),
                        err
                    );

                    throw_str(&msg)
                } else {
                    throw_str(message)
                }
            }
        }
    }
}
