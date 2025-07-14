use crate::utils::bind;
use emlite::FromVal;

#[derive(Clone, Debug)]
pub struct URL {
    inner: emlite::Val,
}
bind!(URL);

impl URL {
    /// `new URL(input, base?)`
    pub fn new(input: &str, base: Option<&str>) -> Self {
        let ctor = emlite::Val::global("URL");
        match base {
            Some(b) => ctor.new(&[input.into(), b.into()]).as_::<Self>(),
            None => ctor.new(&[input.into()]).as_::<Self>(),
        }
    }

    pub fn href(&self) -> String {
        self.inner.get("href").as_::<String>()
    }
    pub fn set_href(&mut self, v: &str) {
        self.inner.set("href", v);
    }

    pub fn protocol(&self) -> String {
        self.inner.get("protocol").as_::<String>()
    }
    pub fn set_protocol(&mut self, v: &str) {
        self.inner.set("protocol", v);
    }

    pub fn pathname(&self) -> String {
        self.inner.get("pathname").as_::<String>()
    }
    pub fn set_pathname(&mut self, v: &str) {
        self.inner.set("pathname", v);
    }

    pub fn search_params(&self) -> URLSearchParams {
        self.inner.get("searchParams").as_::<URLSearchParams>()
    }
}

/// `URLSearchParams` – minimal wrapper.
#[derive(Clone, Debug)]
pub struct URLSearchParams {
    inner: emlite::Val,
}
bind!(URLSearchParams);

impl URLSearchParams {
    pub fn get(&self, key: &str) -> Option<String> {
        let v = self.inner.call("get", &[key.into()]);
        if v.as_handle() == 0 {
            None
        } else {
            Some(v.as_::<String>())
        }
    }

    pub fn append(&mut self, key: &str, value: &str) {
        self.inner.call("append", &[key.into(), value.into()]);
    }
}
