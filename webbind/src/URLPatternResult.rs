use super::*;

/// The URLPatternResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URLPatternResult {
    inner: Any,
}

impl FromVal for URLPatternResult {
    fn from_val(v: &Any) -> Self {
        URLPatternResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for URLPatternResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for URLPatternResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for URLPatternResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for URLPatternResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<URLPatternResult> for Any {
    fn from(s: URLPatternResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&URLPatternResult> for Any {
    fn from(s: &URLPatternResult) -> Any {
        s.inner.clone()
    }
}

impl URLPatternResult {
    /// Getter of the `inputs` attribute.
    pub fn inputs(&self) -> TypedArray<Any> {
        self.inner.get("inputs").as_::<TypedArray<Any>>()
    }

    /// Setter of the `inputs` attribute.
    pub fn set_inputs(&mut self, value: &TypedArray<Any>) {
        self.inner.set("inputs", value);
    }
}
impl URLPatternResult {
    /// Getter of the `protocol` attribute.
    pub fn protocol(&self) -> URLPatternComponentResult {
        self.inner
            .get("protocol")
            .as_::<URLPatternComponentResult>()
    }

    /// Setter of the `protocol` attribute.
    pub fn set_protocol(&mut self, value: &URLPatternComponentResult) {
        self.inner.set("protocol", value);
    }
}
impl URLPatternResult {
    /// Getter of the `username` attribute.
    pub fn username(&self) -> URLPatternComponentResult {
        self.inner
            .get("username")
            .as_::<URLPatternComponentResult>()
    }

    /// Setter of the `username` attribute.
    pub fn set_username(&mut self, value: &URLPatternComponentResult) {
        self.inner.set("username", value);
    }
}
impl URLPatternResult {
    /// Getter of the `password` attribute.
    pub fn password(&self) -> URLPatternComponentResult {
        self.inner
            .get("password")
            .as_::<URLPatternComponentResult>()
    }

    /// Setter of the `password` attribute.
    pub fn set_password(&mut self, value: &URLPatternComponentResult) {
        self.inner.set("password", value);
    }
}
impl URLPatternResult {
    /// Getter of the `hostname` attribute.
    pub fn hostname(&self) -> URLPatternComponentResult {
        self.inner
            .get("hostname")
            .as_::<URLPatternComponentResult>()
    }

    /// Setter of the `hostname` attribute.
    pub fn set_hostname(&mut self, value: &URLPatternComponentResult) {
        self.inner.set("hostname", value);
    }
}
impl URLPatternResult {
    /// Getter of the `port` attribute.
    pub fn port(&self) -> URLPatternComponentResult {
        self.inner.get("port").as_::<URLPatternComponentResult>()
    }

    /// Setter of the `port` attribute.
    pub fn set_port(&mut self, value: &URLPatternComponentResult) {
        self.inner.set("port", value);
    }
}
impl URLPatternResult {
    /// Getter of the `pathname` attribute.
    pub fn pathname(&self) -> URLPatternComponentResult {
        self.inner
            .get("pathname")
            .as_::<URLPatternComponentResult>()
    }

    /// Setter of the `pathname` attribute.
    pub fn set_pathname(&mut self, value: &URLPatternComponentResult) {
        self.inner.set("pathname", value);
    }
}
impl URLPatternResult {
    /// Getter of the `search` attribute.
    pub fn search(&self) -> URLPatternComponentResult {
        self.inner.get("search").as_::<URLPatternComponentResult>()
    }

    /// Setter of the `search` attribute.
    pub fn set_search(&mut self, value: &URLPatternComponentResult) {
        self.inner.set("search", value);
    }
}
impl URLPatternResult {
    /// Getter of the `hash` attribute.
    pub fn hash(&self) -> URLPatternComponentResult {
        self.inner.get("hash").as_::<URLPatternComponentResult>()
    }

    /// Setter of the `hash` attribute.
    pub fn set_hash(&mut self, value: &URLPatternComponentResult) {
        self.inner.set("hash", value);
    }
}
