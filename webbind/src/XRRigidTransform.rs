use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for XRRigidTransform {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRRigidTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRRigidTransform {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRRigidTransform {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRRigidTransform> for emlite::Val {
    fn from(s: XRRigidTransform) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&XRRigidTransform> for emlite::Val {
    fn from(s: &XRRigidTransform) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRRigidTransform);

impl XRRigidTransform {
    pub fn new0() -> XRRigidTransform {
        Self {
            inner: emlite::Val::global("XRRigidTransform")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(position: &DOMPointInit) -> XRRigidTransform {
        Self {
            inner: emlite::Val::global("XRRigidTransform")
                .new(&[position.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(position: &DOMPointInit, orientation: &DOMPointInit) -> XRRigidTransform {
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
    pub fn matrix(&self) -> Float32Array {
        self.inner.get("matrix").as_::<Float32Array>()
    }
}
impl XRRigidTransform {
    pub fn inverse(&self) -> XRRigidTransform {
        self.inner.get("inverse").as_::<XRRigidTransform>()
    }
}
