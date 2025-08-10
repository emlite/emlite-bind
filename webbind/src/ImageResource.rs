use super::*;

/// The ImageResource dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageResource {
    inner: Any,
}

impl FromVal for ImageResource {
    fn from_val(v: &Any) -> Self {
        ImageResource { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageResource {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageResource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageResource {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ImageResource> for Any {
    fn from(s: ImageResource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageResource> for Any {
    fn from(s: &ImageResource) -> Any {
        s.inner.clone()
    }
}

impl ImageResource {
    /// Getter of the `src` attribute.
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

    /// Setter of the `src` attribute.
    pub fn set_src(&mut self, value: &JsString) {
        self.inner.set("src", value);
    }
}
impl ImageResource {
    /// Getter of the `sizes` attribute.
    pub fn sizes(&self) -> JsString {
        self.inner.get("sizes").as_::<JsString>()
    }

    /// Setter of the `sizes` attribute.
    pub fn set_sizes(&mut self, value: &JsString) {
        self.inner.set("sizes", value);
    }
}
impl ImageResource {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl ImageResource {
    /// Getter of the `label` attribute.
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
