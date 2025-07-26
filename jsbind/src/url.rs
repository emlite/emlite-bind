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
    /// `new URL(input, base?)`
    pub fn new(input: &str, base: Option<&str>) -> Self {
        let ctor = emlite::Val::global("URL");
        match base {
            Some(b) => ctor.new(&[input.into(), b.into()]).as_::<Self>(),
            None => ctor.new(&[input.into()]).as_::<Self>(),
        }
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
    pub fn get(&self, key: &str) -> Option<Option<String>> {
        let v = self.inner.get(key);
        if v.is_null() {
            None
        } else {
            Some(v.as_::<Option<String>>())
        }
    }

    pub fn append(&mut self, key: &str, value: &str) {
        self.inner.call("append", &[key.into(), value.into()]);
    }
}
