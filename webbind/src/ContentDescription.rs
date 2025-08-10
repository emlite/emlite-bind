use super::*;

/// The ContentDescription dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContentDescription {
    inner: Any,
}

impl FromVal for ContentDescription {
    fn from_val(v: &Any) -> Self {
        ContentDescription { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ContentDescription {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ContentDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ContentDescription {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ContentDescription {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ContentDescription> for Any {
    fn from(s: ContentDescription) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ContentDescription> for Any {
    fn from(s: &ContentDescription) -> Any {
        s.inner.clone()
    }
}

impl ContentDescription {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl ContentDescription {
    /// Getter of the `title` attribute.
    pub fn title(&self) -> JsString {
        self.inner.get("title").as_::<JsString>()
    }

    /// Setter of the `title` attribute.
    pub fn set_title(&mut self, value: &JsString) {
        self.inner.set("title", value);
    }
}
impl ContentDescription {
    /// Getter of the `description` attribute.
    pub fn description(&self) -> JsString {
        self.inner.get("description").as_::<JsString>()
    }

    /// Setter of the `description` attribute.
    pub fn set_description(&mut self, value: &JsString) {
        self.inner.set("description", value);
    }
}
impl ContentDescription {
    /// Getter of the `category` attribute.
    pub fn category(&self) -> ContentCategory {
        self.inner.get("category").as_::<ContentCategory>()
    }

    /// Setter of the `category` attribute.
    pub fn set_category(&mut self, value: &ContentCategory) {
        self.inner.set("category", value);
    }
}
impl ContentDescription {
    /// Getter of the `icons` attribute.
    pub fn icons(&self) -> TypedArray<ImageResource> {
        self.inner.get("icons").as_::<TypedArray<ImageResource>>()
    }

    /// Setter of the `icons` attribute.
    pub fn set_icons(&mut self, value: &TypedArray<ImageResource>) {
        self.inner.set("icons", value);
    }
}
impl ContentDescription {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
