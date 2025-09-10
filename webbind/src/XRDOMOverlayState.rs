use super::*;

/// The XRDOMOverlayState dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRDOMOverlayState {
    inner: Any,
}

impl FromVal for XRDOMOverlayState {
    fn from_val(v: &Any) -> Self {
        XRDOMOverlayState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRDOMOverlayState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRDOMOverlayState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRDOMOverlayState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRDOMOverlayState {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRDOMOverlayState> for Any {
    fn from(s: XRDOMOverlayState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRDOMOverlayState> for Any {
    fn from(s: &XRDOMOverlayState) -> Any {
        s.inner.clone()
    }
}

impl XRDOMOverlayState {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> XRDOMOverlayType {
        self.inner.get("type").as_::<XRDOMOverlayType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &XRDOMOverlayType) {
        self.inner.set("type", value);
    }
}
