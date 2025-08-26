use crate::error::TypeError;
use crate::utils::*;
use alloc::string::String;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URL {
    inner: emlite::Val,
}
bind!(URL);
impl_dyn_cast!(URL);

impl URL {
    /// Creates a new URL object with error handling.
    ///
    /// # Arguments
    /// * `input` - URL string to parse
    /// * `base` - Optional base URL string
    ///
    /// # Returns
    /// Result containing the URL object or a TypeError if the URL is malformed
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let url = URL::new("https://example.com", None).unwrap();
    /// let relative = URL::new("/path", Some("https://example.com")).unwrap();
    /// assert!(URL::new("invalid-url", None).is_err());
    /// ```
    pub fn new(input: &str, base: Option<String>) -> Result<Self, TypeError> {
        let ctor = emlite::Val::global("URL");
        let result = match base {
            Some(b) => ctor.new(&[input.into(), b.into()]),
            None => ctor.new(&[input.into()]),
        };
        result.as_::<Result<Self, TypeError>>()
    }

    pub fn href(&self) -> Option<String> {
        self.inner.get("href").as_::<Option<String>>()
    }
    pub fn set_href(&mut self, v: &str) {
        self.inner.set("href", v);
    }

    pub fn protocol(&self) -> Option<String> {
        self.inner.get("protocol").as_::<Option<String>>()
    }
    pub fn set_protocol(&mut self, v: &str) {
        self.inner.set("protocol", v);
    }

    pub fn pathname(&self) -> Option<String> {
        self.inner.get("pathname").as_::<Option<String>>()
    }
    pub fn set_pathname(&mut self, v: &str) {
        self.inner.set("pathname", v);
    }

    pub fn search_params(&self) -> URLSearchParams {
        self.inner.get("searchParams").as_::<URLSearchParams>()
    }
}

/// `URLSearchParams` – minimal wrapper. There is a URLSearchParams in webbind as well
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URLSearchParams {
    inner: emlite::Val,
}
bind!(URLSearchParams);
impl_dyn_cast!(URLSearchParams);

impl URLSearchParams {
    pub fn get(&self, key: &str) -> Option<String> {
        let v = self.inner.call("get", &[key.into()]);
        if v.is_null() {
            None
        } else {
            v.as_::<Option<String>>()
        }
    }

    pub fn append(&mut self, key: &str, value: &str) {
        self.inner.call("append", &[key.into(), value.into()]);
    }
}
