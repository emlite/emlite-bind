use super::*;

#[derive(Clone, Debug)]
pub struct XRRigidTransform {
    inner: emlite::Val,
}
impl FromVal for XRRigidTransform {
    fn from_val(v: &emlite::Val) -> Self {
        XRRigidTransform {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRRigidTransform {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRRigidTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRRigidTransform> for emlite::Val {
    fn from(s: XRRigidTransform) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRRigidTransform {
    pub fn new0() -> XRRigidTransform {
        Self {
            inner: emlite::Val::global("XRRigidTransform")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(position: DOMPointInit) -> XRRigidTransform {
        Self {
            inner: emlite::Val::global("XRRigidTransform")
                .new(&[position.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(position: DOMPointInit, orientation: DOMPointInit) -> XRRigidTransform {
        Self {
            inner: emlite::Val::global("XRRigidTransform")
                .new(&[position.into(), orientation.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl XRRigidTransform {
    pub fn position(&self) -> DOMPointReadOnly {
        self.inner.get("position").as_::<DOMPointReadOnly>()
    }
}
impl XRRigidTransform {
    pub fn orientation(&self) -> DOMPointReadOnly {
        self.inner.get("orientation").as_::<DOMPointReadOnly>()
    }
}
impl XRRigidTransform {
    pub fn matrix(&self) -> jsbind::Float32Array {
        self.inner.get("matrix").as_::<jsbind::Float32Array>()
    }
}
impl XRRigidTransform {
    pub fn inverse(&self) -> XRRigidTransform {
        self.inner.get("inverse").as_::<XRRigidTransform>()
    }
}
