use super::*;




/// The BackgroundFetchUIOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchUIOptions {
    inner: Any,
}

impl FromVal for BackgroundFetchUIOptions {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchUIOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BackgroundFetchUIOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BackgroundFetchUIOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BackgroundFetchUIOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BackgroundFetchUIOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BackgroundFetchUIOptions> for Any {
    fn from(s: BackgroundFetchUIOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BackgroundFetchUIOptions> for Any {
    fn from(s: &BackgroundFetchUIOptions) -> Any {
        s.inner.clone()
    }
}

impl BackgroundFetchUIOptions {
    /// Getter of the `icons` attribute.
    pub fn icons(&self) -> TypedArray<ImageResource> {
        self.inner.get("icons").as_::<TypedArray<ImageResource>>()
    }

    /// Setter of the `icons` attribute.
    pub fn set_icons(&mut self, value: &TypedArray<ImageResource>) {
        self.inner.set("icons", value);
    }
}
impl BackgroundFetchUIOptions {
    /// Getter of the `title` attribute.
    pub fn title(&self) -> JsString {
        self.inner.get("title").as_::<JsString>()
    }

    /// Setter of the `title` attribute.
    pub fn set_title(&mut self, value: &JsString) {
        self.inner.set("title", value);
    }
}
