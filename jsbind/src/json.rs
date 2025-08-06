use crate::any::Any;
use crate::error::*;
use crate::string::JsString;
use alloc::vec;
use emlite::FromVal;

/// Static‐only namespace: `JSON.parse`, `JSON.stringify`, …
pub struct JSON;

impl JSON {
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
    pub fn parse<T>(text: &str) -> Result<T, SyntaxError>
    where
        T: FromVal,
    {
        let res = Self::obj().call("parse", &[text.into()]);
        res.as_::<Result<T, SyntaxError>>()
    }

    /// Serialize a JS value (or Rust wrapper) to a JSON string.
    ///
    /// `space` mirrors the optional 3rd arg of JS `stringify`
    /// (`number` → indent, `string` → prefix).
    ///
    /// # Returns
    /// Result containing the JSON string or a TypeError for serialization errors
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let obj = Object::new();
    /// obj.set("key", "value");
    /// let json = JSON::stringify(&obj, None, None).unwrap();
    /// ```
    pub fn stringify(
        value: &Any,
        replacer: Option<&Any>,
        space: Option<&Any>,
    ) -> Result<JsString, JsError> {
        let mut args = vec![value.into()];
        if let Some(r) = replacer {
            args.push(r.into());
        }
        if let Some(s) = space {
            args.push(s.into());
        }

        Self::obj().call("stringify", &args).as_()
    }
}
