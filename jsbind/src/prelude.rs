pub use emlite;
pub use emlite::Console;
pub use emlite::FromVal;

use alloc::{format, vec};

pub use crate::any::{Any, AnyHandle};
pub use crate::array::{
    Array, ArrayBuffer, DataView, Endian, Float32Array, Float64Array, FrozenArray, Int8Array,
    Int32Array, ObservableArray, TypedArray, Uint8Array, Uint32Array,
};
pub use crate::bigint::BigInt;
pub use crate::date::Date;
pub use crate::error::*;
pub use crate::function::{Closure, Function};
pub use crate::json::JSON;
pub use crate::map::*;
pub use crate::math::Math;
pub use crate::null::Null;
pub use crate::object::Object;
pub use crate::promise::Promise;
pub use crate::record::Record;
pub use crate::reflect::Reflect;
pub use crate::regexp::{RegExp, RegExpFlags};
pub use crate::response::{fetch, fetch_val};
pub use crate::set::*;
pub use crate::string::JsString;
pub use crate::symbol::Symbol;
pub use crate::text::{TextDecoder, TextEncoder};
pub use crate::time::*;
pub use crate::undefined::Undefined;
pub use crate::url::URL;

/// Parse `src` with an optional `radix`.  Mirrors `parseInt(str, radix)`.
///
/// # Arguments
/// * `src` - String to parse
/// * `radix` - Optional radix (2-36)
///
/// # Returns
/// Result containing parsed integer or error
///
/// # Examples
/// ```rust
/// use jsbind::prelude::*;
///
/// let result = parse_int("42", None);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), 42);
///
/// let error_result = parse_int("not_a_number", None);
/// assert!(error_result.is_err());
/// ```
pub fn parse_int(src: &str, radix: Option<i32>) -> Result<i32, JsError> {
    let g = emlite::Val::global("parseInt");
    let result = match radix {
        Some(r) => {
            if r < 2 || r > 36 {
                return Err(JsError::new("Radix must be between 2 and 36"));
            }
            g.invoke(&[src.into(), r.into()])
        }
        None => g.invoke(&[src.into()]),
    };

    if is_nan(&result) {
        Err(JsError::new(&format!("Invalid number format: '{}'", src)))
    } else {
        Ok(result.as_::<i32>())
    }
}

/// Parse a floating-point value – identical to JS `parseFloat(str)`.
///
/// # Arguments
/// * `src` - String to parse
///
/// # Returns
/// Result containing parsed float or error
///
/// # Examples
/// ```rust
/// use jsbind::prelude::*;
///
/// let result = parse_float("3.14");
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), 3.14);
///
/// let error_result = parse_float("not_a_number");
/// assert!(error_result.is_err());
/// ```
pub fn parse_float(src: &str) -> Result<f64, JsError> {
    let result = emlite::Val::global("parseFloat").invoke(&[src.into()]);

    if is_nan(&result) {
        Err(JsError::new(&format!("Invalid number format: '{}'", src)))
    } else {
        Ok(result.as_::<f64>())
    }
}

/// Trait analogous to `wasm-bindgen::JsCast`.
///
/// Automatically available on every wrapper that is:
/// * holds a single `emlite::Val`
/// * implements `AsRef<Val>` and `Into<Val>`
pub trait DynCast
where
    Self: AsRef<emlite::Val> + Into<emlite::Val> + AsMut<emlite::Val>,
{
    fn has_type<T>(&self) -> bool
    where
        T: DynCast,
    {
        T::is_type_of(self.as_ref())
    }

    fn dyn_into<T>(self) -> Result<T, Self>
    where
        T: DynCast,
    {
        if self.has_type::<T>() {
            Ok(self.unchecked_into())
        } else {
            Err(self)
        }
    }

    fn dyn_ref<T>(&self) -> Option<&T>
    where
        T: DynCast,
    {
        if self.has_type::<T>() {
            Some(self.unchecked_ref())
        } else {
            None
        }
    }

    fn dyn_mut<T>(&mut self) -> Option<&mut T>
    where
        T: DynCast,
    {
        if self.has_type::<T>() {
            Some(self.unchecked_mut())
        } else {
            None
        }
    }

    fn unchecked_into<T>(self) -> T
    where
        T: DynCast,
    {
        T::unchecked_from_val(self.into())
    }

    fn unchecked_ref<T>(&self) -> &T
    where
        T: DynCast,
    {
        T::unchecked_from_val_ref(self.as_ref())
    }

    fn unchecked_mut<T>(&mut self) -> &mut T
    where
        T: DynCast,
    {
        T::unchecked_from_val_mut(self.as_mut())
    }

    fn is_instance_of<T>(&self) -> bool
    where
        T: DynCast,
    {
        T::instanceof(self.as_ref())
    }

    /// Implementation of `val instanceof ThisType`.
    fn instanceof(val: &emlite::Val) -> bool;

    /// Customisable brand check – defaults to `instanceof`.
    fn is_type_of(val: &emlite::Val) -> bool {
        Self::instanceof(val)
    }

    /// Zero-cost unchecked conversion from `Val` into `Self`.
    fn unchecked_from_val(v: emlite::Val) -> Self;

    /// Zero-cost unchecked conversion from `&Val` into `&Self`.
    fn unchecked_from_val_ref(v: &emlite::Val) -> &Self;

    /// Zero-cost unchecked conversion from `&mut Val` into `&mut Self`.
    fn unchecked_from_val_mut(v: &mut emlite::Val) -> &mut Self;
}

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

/// Encode string to base64.
///
/// # Arguments  
/// * `data` - String to encode
///
/// # Returns
/// Result containing base64 encoded string or error
///
/// # Examples
/// ```rust
/// use jsbind::prelude::*;
///
/// let input = JsString::from("Hello World");
/// let result = btoa(&input);
/// assert!(result.is_ok());
/// ```
pub fn btoa(data: &JsString) -> Result<JsString, JsError> {
    let result = emlite::Val::global("btoa").invoke(&[data.into()]);
    result.as_::<Result<JsString, JsError>>()
}

/// Decode base64 string.
///
/// # Arguments
/// * `encoded` - Base64 encoded string  
///
/// # Returns
/// Result containing decoded string or error
///
/// # Examples
/// ```rust
/// use jsbind::prelude::*;
///
/// let encoded = JsString::from("SGVsbG8gV29ybGQ=");
/// let result = atob(&encoded);
/// assert!(result.is_ok());
/// ```
pub fn atob(encoded: &JsString) -> Result<JsString, JsError> {
    let result = emlite::Val::global("atob").invoke(&[encoded.into()]);
    result.as_::<Result<JsString, JsError>>()
}

/// Checks if a value is NaN.
///
/// # Arguments
/// * `value` - Value to check
///
/// # Returns
/// `true` if the value is NaN, `false` otherwise
///
/// # Examples
/// ```rust
/// use jsbind::prelude::*;
///
/// assert!(is_nan(&(0.0/0.0).into()));
/// assert!(!is_nan(&42.into()));
/// ```
pub fn is_nan<T>(value: &T) -> bool
where
    T: AsRef<emlite::Val>,
{
    emlite::Val::global("isNaN")
        .invoke(&[value.as_ref().clone()])
        .as_::<bool>()
}

/// Queues a microtask to be executed.
///
/// # Arguments
/// * `callback` - Function to execute as microtask
///
/// # Examples
/// ```rust
/// use jsbind::prelude::*;
///
/// let callback = Function::from_closure(|| {
///     Console::get().log(&["Microtask executed!".into()]);
/// });
/// queue_microtask(&callback);
/// ```
pub fn queue_microtask(callback: &Function) {
    emlite::Val::global("queueMicrotask").invoke(&[callback.into()]);
}

/// Options for structured cloning operations.
#[derive(Clone, Debug)]
pub struct JsStructuredSerializeOptions {
    inner: emlite::Val,
}

impl JsStructuredSerializeOptions {
    /// Creates a new JsStructuredSerializeOptions object.
    pub fn new() -> Self {
        Self {
            inner: emlite::Val::object(),
        }
    }

    /// Gets the transfer list for transferable objects.
    pub fn transfer(&self) -> Option<TypedArray<Object>> {
        let val = self.inner.get("transfer");
        if val.is_undefined() {
            None
        } else {
            Some(val.as_::<TypedArray<Object>>())
        }
    }

    /// Sets the transfer list for transferable objects.
    pub fn set_transfer(&self, transfer: &TypedArray<Object>) {
        self.inner.set("transfer", transfer);
    }
}

impl Default for JsStructuredSerializeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<emlite::Val> for JsStructuredSerializeOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl From<JsStructuredSerializeOptions> for emlite::Val {
    fn from(options: JsStructuredSerializeOptions) -> Self {
        options.inner
    }
}

impl From<&JsStructuredSerializeOptions> for emlite::Val {
    fn from(options: &JsStructuredSerializeOptions) -> Self {
        options.inner.clone()
    }
}

/// Performs a structured clone of a value.
///
/// # Arguments
/// * `value` - The value to clone
/// * `options` - Optional structured clone options
///
/// # Returns
/// Deep clone of the input value
///
/// # Examples
/// ```rust
/// use jsbind::prelude::*;
///
/// let obj = Object::new();
/// obj.set("key", "value");
///
/// let cloned = structured_clone(&obj, None);
/// // cloned is a deep copy of obj
/// ```
pub fn structured_clone<T>(value: &T, options: Option<&JsStructuredSerializeOptions>) -> T
where
    T: emlite::FromVal + AsRef<emlite::Val>,
{
    let args = match options {
        Some(opts) => vec![value.as_ref().clone(), opts.into()],
        None => vec![value.as_ref().clone()],
    };

    emlite::Val::global("structuredClone")
        .invoke(&args)
        .as_::<T>()
}
