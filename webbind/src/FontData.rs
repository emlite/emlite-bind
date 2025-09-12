use super::*;

/// The FontData class.
/// [`FontData`](https://developer.mozilla.org/en-US/docs/Web/API/FontData)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontData {
    inner: Any,
}

impl FromVal for FontData {
    fn from_val(v: &Any) -> Self {
        FontData {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FontData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FontData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FontData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FontData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FontData> for Any {
    fn from(s: FontData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FontData> for Any {
    fn from(s: &FontData) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FontData);

impl FontData {
    /// Getter of the `postscriptName` attribute.
    /// [`FontData.postscriptName`](https://developer.mozilla.org/en-US/docs/Web/API/FontData/postscriptName)
    pub fn postscript_name(&self) -> JsString {
        self.inner.get("postscriptName").as_::<JsString>()
    }
}
impl FontData {
    /// Getter of the `fullName` attribute.
    /// [`FontData.fullName`](https://developer.mozilla.org/en-US/docs/Web/API/FontData/fullName)
    pub fn full_name(&self) -> JsString {
        self.inner.get("fullName").as_::<JsString>()
    }
}
impl FontData {
    /// Getter of the `family` attribute.
    /// [`FontData.family`](https://developer.mozilla.org/en-US/docs/Web/API/FontData/family)
    pub fn family(&self) -> JsString {
        self.inner.get("family").as_::<JsString>()
    }
}
impl FontData {
    /// Getter of the `style` attribute.
    /// [`FontData.style`](https://developer.mozilla.org/en-US/docs/Web/API/FontData/style)
    pub fn style(&self) -> JsString {
        self.inner.get("style").as_::<JsString>()
    }
}
impl FontData {
    /// The blob method.
    /// [`FontData.blob`](https://developer.mozilla.org/en-US/docs/Web/API/FontData/blob)
    pub fn blob(&self) -> Promise<Blob> {
        self.inner.call("blob", &[]).as_::<Promise<Blob>>()
    }
}
