use super::*;




/// The RelatedApplication dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RelatedApplication {
    inner: Any,
}

impl FromVal for RelatedApplication {
    fn from_val(v: &Any) -> Self {
        RelatedApplication { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RelatedApplication {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RelatedApplication {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RelatedApplication {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RelatedApplication {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RelatedApplication> for Any {
    fn from(s: RelatedApplication) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RelatedApplication> for Any {
    fn from(s: &RelatedApplication) -> Any {
        s.inner.clone()
    }
}

impl RelatedApplication {
    /// Getter of the `platform` attribute.
    pub fn platform(&self) -> JsString {
        self.inner.get("platform").as_::<JsString>()
    }

    /// Setter of the `platform` attribute.
    pub fn set_platform(&mut self, value: &JsString) {
        self.inner.set("platform", value);
    }
}
impl RelatedApplication {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl RelatedApplication {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl RelatedApplication {
    /// Getter of the `version` attribute.
    pub fn version(&self) -> JsString {
        self.inner.get("version").as_::<JsString>()
    }

    /// Setter of the `version` attribute.
    pub fn set_version(&mut self, value: &JsString) {
        self.inner.set("version", value);
    }
}
