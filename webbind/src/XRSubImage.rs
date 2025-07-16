use super::*;

/// The XRSubImage class.
/// [`XRSubImage`](https://developer.mozilla.org/en-US/docs/Web/API/XRSubImage)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSubImage {
    inner: Any,
}
impl FromVal for XRSubImage {
    fn from_val(v: &Any) -> Self {
        XRSubImage {
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
impl core::ops::Deref for XRSubImage {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRSubImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRSubImage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRSubImage {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRSubImage> for Any {
    fn from(s: XRSubImage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRSubImage> for Any {
    fn from(s: &XRSubImage) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRSubImage);

impl XRSubImage {
    /// Getter of the `viewport` attribute.
    /// [`XRSubImage.viewport`](https://developer.mozilla.org/en-US/docs/Web/API/XRSubImage/viewport)
    pub fn viewport(&self) -> XRViewport {
        self.inner.get("viewport").as_::<XRViewport>()
    }
}
