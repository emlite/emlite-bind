use super::*;

/// The MediaImage dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaImage {
    inner: Any,
}

impl FromVal for MediaImage {
    fn from_val(v: &Any) -> Self {
        MediaImage { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaImage {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaImage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaImage {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaImage> for Any {
    fn from(s: MediaImage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaImage> for Any {
    fn from(s: &MediaImage) -> Any {
        s.inner.clone()
    }
}

impl MediaImage {
    /// Getter of the `src` attribute.
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

    /// Setter of the `src` attribute.
    pub fn set_src(&mut self, value: &JsString) {
        self.inner.set("src", value);
    }
}
impl MediaImage {
    /// Getter of the `sizes` attribute.
    pub fn sizes(&self) -> JsString {
        self.inner.get("sizes").as_::<JsString>()
    }

    /// Setter of the `sizes` attribute.
    pub fn set_sizes(&mut self, value: &JsString) {
        self.inner.set("sizes", value);
    }
}
impl MediaImage {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
