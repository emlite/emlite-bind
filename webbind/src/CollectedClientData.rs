use super::*;




/// The CollectedClientData dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CollectedClientData {
    inner: Any,
}

impl FromVal for CollectedClientData {
    fn from_val(v: &Any) -> Self {
        CollectedClientData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CollectedClientData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CollectedClientData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CollectedClientData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CollectedClientData {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CollectedClientData> for Any {
    fn from(s: CollectedClientData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CollectedClientData> for Any {
    fn from(s: &CollectedClientData) -> Any {
        s.inner.clone()
    }
}

impl CollectedClientData {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl CollectedClientData {
    /// Getter of the `challenge` attribute.
    pub fn challenge(&self) -> JsString {
        self.inner.get("challenge").as_::<JsString>()
    }

    /// Setter of the `challenge` attribute.
    pub fn set_challenge(&mut self, value: &JsString) {
        self.inner.set("challenge", value);
    }
}
impl CollectedClientData {
    /// Getter of the `origin` attribute.
    pub fn origin(&self) -> JsString {
        self.inner.get("origin").as_::<JsString>()
    }

    /// Setter of the `origin` attribute.
    pub fn set_origin(&mut self, value: &JsString) {
        self.inner.set("origin", value);
    }
}
impl CollectedClientData {
    /// Getter of the `crossOrigin` attribute.
    pub fn cross_origin(&self) -> bool {
        self.inner.get("crossOrigin").as_::<bool>()
    }

    /// Setter of the `crossOrigin` attribute.
    pub fn set_cross_origin(&mut self, value: bool) {
        self.inner.set("crossOrigin", value);
    }
}
impl CollectedClientData {
    /// Getter of the `topOrigin` attribute.
    pub fn top_origin(&self) -> JsString {
        self.inner.get("topOrigin").as_::<JsString>()
    }

    /// Setter of the `topOrigin` attribute.
    pub fn set_top_origin(&mut self, value: &JsString) {
        self.inner.set("topOrigin", value);
    }
}
