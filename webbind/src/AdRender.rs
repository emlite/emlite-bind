use super::*;




/// The AdRender dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AdRender {
    inner: Any,
}

impl FromVal for AdRender {
    fn from_val(v: &Any) -> Self {
        AdRender { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AdRender {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AdRender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AdRender {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AdRender {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AdRender> for Any {
    fn from(s: AdRender) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AdRender> for Any {
    fn from(s: &AdRender) -> Any {
        s.inner.clone()
    }
}

impl AdRender {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl AdRender {
    /// Getter of the `width` attribute.
    pub fn width(&self) -> JsString {
        self.inner.get("width").as_::<JsString>()
    }

    /// Setter of the `width` attribute.
    pub fn set_width(&mut self, value: &JsString) {
        self.inner.set("width", value);
    }
}
impl AdRender {
    /// Getter of the `height` attribute.
    pub fn height(&self) -> JsString {
        self.inner.get("height").as_::<JsString>()
    }

    /// Setter of the `height` attribute.
    pub fn set_height(&mut self, value: &JsString) {
        self.inner.set("height", value);
    }
}
