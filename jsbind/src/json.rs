use crate::Any;
use emlite::FromVal;

/// Static‐only namespace: `JSON.parse`, `JSON.stringify`, …
pub struct JSON;

impl JSON {
    /// Internal helper to grab `globalThis.JSON`.
    #[inline]
    fn obj() -> emlite::Val {
        emlite::Val::global("JSON")
    }

    /// Parse a UTF-8 JSON string into an arbitrary JS value
    /// (caller picks the concrete type with `T`).
    ///
    /// ```
    /// use jsbind::{JSON, Any};
    /// let v: Any = JSON::parse(r#"{"a":1}"#).unwrap();
    /// ```
    pub fn parse<T>(text: &str) -> Result<T, Any>
    where
        T: FromVal,
    {
        let res = Self::obj().call("parse", &[text.into()]);
        if res.instanceof(emlite::Val::global("Error")) {
            Err(res.into())
        } else {
            Ok(res.as_::<T>())
        }
    }

    /// Serialize a JS value (or Rust wrapper) to a JSON string.
    ///
    /// `space` mirrors the optional 3rd arg of JS `stringify`
    /// (`number` → indent, `string` → prefix).
    pub fn stringify(value: &Any, replacer: Option<&Any>, space: Option<&Any>) -> String {
        let mut args = vec![value.clone()];
        if let Some(r) = replacer {
            args.push(r.clone());
        }
        if let Some(s) = space {
            args.push(s.clone());
        }
        Self::obj().call("stringify", &args).as_::<String>()
    }
}
