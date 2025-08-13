use super::*;




/// The ShareData dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ShareData {
    inner: Any,
}

impl FromVal for ShareData {
    fn from_val(v: &Any) -> Self {
        ShareData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ShareData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ShareData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ShareData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ShareData {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ShareData> for Any {
    fn from(s: ShareData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ShareData> for Any {
    fn from(s: &ShareData) -> Any {
        s.inner.clone()
    }
}

impl ShareData {
    /// Getter of the `files` attribute.
    pub fn files(&self) -> TypedArray<File> {
        self.inner.get("files").as_::<TypedArray<File>>()
    }

    /// Setter of the `files` attribute.
    pub fn set_files(&mut self, value: &TypedArray<File>) {
        self.inner.set("files", value);
    }
}
impl ShareData {
    /// Getter of the `title` attribute.
    pub fn title(&self) -> JsString {
        self.inner.get("title").as_::<JsString>()
    }

    /// Setter of the `title` attribute.
    pub fn set_title(&mut self, value: &JsString) {
        self.inner.set("title", value);
    }
}
impl ShareData {
    /// Getter of the `text` attribute.
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }

    /// Setter of the `text` attribute.
    pub fn set_text(&mut self, value: &JsString) {
        self.inner.set("text", value);
    }
}
impl ShareData {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
