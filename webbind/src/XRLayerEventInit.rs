use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRLayerEventInit {
    inner: Any,
}
impl FromVal for XRLayerEventInit {
    fn from_val(v: &Any) -> Self {
        XRLayerEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRLayerEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRLayerEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRLayerEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRLayerEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRLayerEventInit> for Any {
    fn from(s: XRLayerEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRLayerEventInit> for Any {
    fn from(s: &XRLayerEventInit) -> Any {
        s.inner.clone()
    }
}

impl XRLayerEventInit {
    pub fn layer(&self) -> XRLayer {
        self.inner.get("layer").as_::<XRLayer>()
    }

    pub fn set_layer(&mut self, value: &XRLayer) {
        self.inner.set("layer", value);
    }
}
