pub use emlite::Console;

mod null;
mod utils;
pub use null::Null;
mod function;
pub use function::{Closure, Function};
mod undefined;
pub use undefined::Undefined;
mod any;
pub use any::Any;
mod object;
pub use object::Object;
mod string;
pub use string::{ByteString, CSSOMString, DOMString, USVString};
mod record;
pub use record::Record;
mod promise;
pub use promise::Promise;
mod sequence;
pub use sequence::Sequence;
mod array;
pub use array::{
    Array, ArrayBuffer, DataView, Float32Array, Float64Array, FrozenArray, Int8Array, Int32Array,
    ObservableArray, TypedArray, Uint8Array, Uint32Array,
};
mod reflect;
pub use reflect::Reflect;
mod json;
pub use json::JSON;
mod date;
pub use date::Date;
mod math;
pub use math::Math;
pub mod error;
mod text;
pub use text::{TextDecoder, TextEncoder};
mod url;
pub use url::{URL, URLSearchParams};
mod time;
pub use time::*;
mod response;
pub use response::*;
mod map;
pub use map::*;
mod set;
pub use set::*;

/// Parse `src` with an optional `radix`.  Mirrors `parseInt(str, radix)`.
pub fn parse_int(src: &str, radix: Option<i32>) -> i32 {
    let g = emlite::Val::global("parseInt");
    match radix {
        Some(r) => g.call("", &[src.into(), r.into()]).as_::<i32>(),
        None => g.call("", &[src.into()]).as_::<i32>(),
    }
}

/// Parse a floating-point value – identical to JS `parseFloat(str)`.
pub fn parse_float(src: &str) -> f64 {
    emlite::Val::global("parseFloat")
        .call("", &[src.into()])
        .as_::<f64>()
}
