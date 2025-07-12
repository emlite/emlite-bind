use super::*;

#[derive(Clone, Debug)]
pub struct XRCubeLayer {
    inner: XRCompositionLayer,
}
impl FromVal for XRCubeLayer {
    fn from_val(v: &emlite::Val) -> Self {
        XRCubeLayer {
            inner: XRCompositionLayer::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRCubeLayer {
    type Target = XRCompositionLayer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRCubeLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRCubeLayer> for emlite::Val {
    fn from(s: XRCubeLayer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRCubeLayer {
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    pub fn set_space(&mut self, value: XRSpace) {
        self.inner.set("space", value);
    }
}
impl XRCubeLayer {
    pub fn orientation(&self) -> DOMPointReadOnly {
        self.inner.get("orientation").as_::<DOMPointReadOnly>()
    }

    pub fn set_orientation(&mut self, value: DOMPointReadOnly) {
        self.inner.set("orientation", value);
    }
}
impl XRCubeLayer {
    pub fn onredraw(&self) -> jsbind::Any {
        self.inner.get("onredraw").as_::<jsbind::Any>()
    }

    pub fn set_onredraw(&mut self, value: jsbind::Any) {
        self.inner.set("onredraw", value);
    }
}
