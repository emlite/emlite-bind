use alloc::string::String;
use alloc::vec::Vec;
pub use emlite::Console;
pub use emlite::FromVal;

pub use crate::any::{Any, AnyHandle};
pub use crate::array::{
    Array, ArrayBuffer, DataView, Endian, Float32Array, Float64Array, FrozenArray, Int8Array,
    Int32Array, ObservableArray, TypedArray, Uint8Array, Uint32Array,
};
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
pub use crate::response::{fetch, fetch_val};
pub use crate::set::*;
pub use crate::string::JsString;
pub use crate::text::{TextDecoder, TextEncoder};
pub use crate::time::*;
pub use crate::undefined::Undefined;
pub use crate::url::URL;

/// Parse `src` with an optional `radix`.  Mirrors `parseInt(str, radix)`.
pub fn parse_int(src: &str, radix: Option<i32>) -> i32 {
    let g = emlite::Val::global("parseInt");
    match radix {
        Some(r) => g.invoke(&[src.into(), r.into()]).as_::<i32>(),
        None => g.invoke(&[src.into()]).as_::<i32>(),
    }
}

/// Parse a floating-point value – identical to JS `parseFloat(str)`.
pub fn parse_float(src: &str) -> f64 {
    emlite::Val::global("parseFloat")
        .invoke(&[src.into()])
        .as_::<f64>()
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

/// Lossless: bytes -> base64
pub fn btoa_bytes(bytes: &[u8]) -> String {
    // Fast path for Node: Buffer.from(u8[]).toString("base64")
    if Any::global_this().has("Buffer") {
        let buffer = Any::global("Buffer");
        let arr = TypedArray::new_from_slice(bytes); // JS array of numbers 0..255
        let buf = buffer.call("from", &[arr.into()]);
        return buf
            .call("toString", &[Any::from("base64")])
            .as_::<Option<String>>()
            .unwrap_throw();
    }

    // Browser/WASI-in-browser: build a binary string and call btoa()
    // Use chunking to avoid "maximum call stack size exceeded" with large arrays.
    let arr = TypedArray::new_from_slice(bytes);
    let to_b64 = Function::new(
        &["a"],
        r#"
        let s = '';
        const CHUNK = 0x8000; // 32 KiB per apply() call
        for (let i = 0; i < a.length; i += CHUNK) {
            s += String.fromCharCode.apply(null, a.slice(i, i + CHUNK));
        }
        return btoa(s);
        "#,
    );
    to_b64
        .call(&Any::undefined(), &[arr.into()])
        .as_::<Option<String>>()
        .unwrap_throw()
}

/// Lossless: base64 -> bytes
pub fn atob_to_bytes(b64: &str) -> Vec<u8> {
    // Fast path for Node: Array.from(Buffer.from(b64, 'base64'))
    if Any::global_this().has("Buffer") {
        let buffer = Any::global("Buffer");
        let buf = buffer.call("from", &[Any::from(b64), Any::from("base64")]);
        let arr = Any::global("Array").call("from", &[buf]);
        return arr.to_vec::<u8>();
    }

    // Browser/WASI-in-browser: atob() -> binary string -> array of byte values
    let from_b64 = Function::new(
        &["b64"],
        r#"
        const bin = atob(b64);
        const out = new Array(bin.length);
        for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i) & 0xff;
        return out;
        "#,
    );
    from_b64
        .call(&Any::undefined(), &[Any::from(b64)])
        .to_vec::<u8>()
}

/// UTF‑8 text -> base64 (convenience)
pub fn btoa_utf8(s: &str) -> String {
    let enc = TextEncoder::new(); // JS TextEncoder
    let u8s = enc.encode(s); // -> Uint8Array (jsbind alias)
    btoa_bytes(&u8s.to_vec())
}

/// base64 -> UTF‑8 text (convenience)
pub fn atob_utf8(b64: &str) -> Option<String> {
    // Prefer Node's native decode when available
    if Any::global_this().has("Buffer") {
        let buffer = Any::global("Buffer");
        let buf = buffer.call("from", &[Any::from(b64), Any::from("base64")]);
        return buf
            .call("toString", &[Any::from("utf8")])
            .as_::<Option<String>>();
    }

    // Browser/WASI-in-browser: decode to bytes, then TextDecoder
    let bytes = atob_to_bytes(b64);
    let dec = TextDecoder::new(Some("utf-8"), None);
    let seq = TypedArray::new_from_slice(&bytes); // jsbind's Uint8Array alias is a TypedArray<u8>
    dec.decode(&seq) // Option<String>
}
