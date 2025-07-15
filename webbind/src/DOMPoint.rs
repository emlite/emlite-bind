use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMPoint {
    inner: DOMPointReadOnly,
}
impl FromVal for DOMPoint {
    fn from_val(v: &emlite::Val) -> Self {
        DOMPoint {
            inner: DOMPointReadOnly::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DOMPoint {
    type Target = DOMPointReadOnly;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMPoint {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMPoint {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DOMPoint> for emlite::Val {
    fn from(s: DOMPoint) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DOMPoint);

impl DOMPoint {
    pub fn new0() -> DOMPoint {
        Self {
            inner: emlite::Val::global("DOMPoint")
                .new(&[])
                .as_::<DOMPointReadOnly>(),
        }
    }

    pub fn new1(x: f64) -> DOMPoint {
        Self {
            inner: emlite::Val::global("DOMPoint")
                .new(&[x.into()])
                .as_::<DOMPointReadOnly>(),
        }
    }

    pub fn new2(x: f64, y: f64) -> DOMPoint {
        Self {
            inner: emlite::Val::global("DOMPoint")
                .new(&[x.into(), y.into()])
                .as_::<DOMPointReadOnly>(),
        }
    }

    pub fn new3(x: f64, y: f64, z: f64) -> DOMPoint {
        Self {
            inner: emlite::Val::global("DOMPoint")
                .new(&[x.into(), y.into(), z.into()])
                .as_::<DOMPointReadOnly>(),
        }
    }

    pub fn new4(x: f64, y: f64, z: f64, w: f64) -> DOMPoint {
        Self {
            inner: emlite::Val::global("DOMPoint")
                .new(&[x.into(), y.into(), z.into(), w.into()])
                .as_::<DOMPointReadOnly>(),
        }
    }
}
impl DOMPoint {
    pub fn from_point0() -> DOMPoint {
        emlite::Val::global("DOMPoint")
            .call("fromPoint", &[])
            .as_::<DOMPoint>()
    }

    pub fn from_point1(other: DOMPointInit) -> DOMPoint {
        emlite::Val::global("DOMPoint")
            .call("fromPoint", &[other.into()])
            .as_::<DOMPoint>()
    }
}
impl DOMPoint {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl DOMPoint {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl DOMPoint {
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }

    pub fn set_z(&mut self, value: f64) {
        self.inner.set("z", value);
    }
}
impl DOMPoint {
    pub fn w(&self) -> f64 {
        self.inner.get("w").as_::<f64>()
    }

    pub fn set_w(&mut self, value: f64) {
        self.inner.set("w", value);
    }
}
