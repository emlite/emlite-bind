use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMMatrixInit {
    inner: emlite::Val,
}
impl FromVal for DOMMatrixInit {
    fn from_val(v: &emlite::Val) -> Self {
        DOMMatrixInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DOMMatrixInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMMatrixInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMMatrixInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMMatrixInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DOMMatrixInit> for emlite::Val {
    fn from(s: DOMMatrixInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DOMMatrixInit {
    pub fn m13(&self) -> f64 {
        self.inner.get("m13").as_::<f64>()
    }

    pub fn set_m13(&mut self, value: f64) {
        self.inner.set("m13", value);
    }
}
impl DOMMatrixInit {
    pub fn m14(&self) -> f64 {
        self.inner.get("m14").as_::<f64>()
    }

    pub fn set_m14(&mut self, value: f64) {
        self.inner.set("m14", value);
    }
}
impl DOMMatrixInit {
    pub fn m23(&self) -> f64 {
        self.inner.get("m23").as_::<f64>()
    }

    pub fn set_m23(&mut self, value: f64) {
        self.inner.set("m23", value);
    }
}
impl DOMMatrixInit {
    pub fn m24(&self) -> f64 {
        self.inner.get("m24").as_::<f64>()
    }

    pub fn set_m24(&mut self, value: f64) {
        self.inner.set("m24", value);
    }
}
impl DOMMatrixInit {
    pub fn m31(&self) -> f64 {
        self.inner.get("m31").as_::<f64>()
    }

    pub fn set_m31(&mut self, value: f64) {
        self.inner.set("m31", value);
    }
}
impl DOMMatrixInit {
    pub fn m32(&self) -> f64 {
        self.inner.get("m32").as_::<f64>()
    }

    pub fn set_m32(&mut self, value: f64) {
        self.inner.set("m32", value);
    }
}
impl DOMMatrixInit {
    pub fn m33(&self) -> f64 {
        self.inner.get("m33").as_::<f64>()
    }

    pub fn set_m33(&mut self, value: f64) {
        self.inner.set("m33", value);
    }
}
impl DOMMatrixInit {
    pub fn m34(&self) -> f64 {
        self.inner.get("m34").as_::<f64>()
    }

    pub fn set_m34(&mut self, value: f64) {
        self.inner.set("m34", value);
    }
}
impl DOMMatrixInit {
    pub fn m43(&self) -> f64 {
        self.inner.get("m43").as_::<f64>()
    }

    pub fn set_m43(&mut self, value: f64) {
        self.inner.set("m43", value);
    }
}
impl DOMMatrixInit {
    pub fn m44(&self) -> f64 {
        self.inner.get("m44").as_::<f64>()
    }

    pub fn set_m44(&mut self, value: f64) {
        self.inner.set("m44", value);
    }
}
impl DOMMatrixInit {
    pub fn is2_d(&self) -> bool {
        self.inner.get("is2D").as_::<bool>()
    }

    pub fn set_is2_d(&mut self, value: bool) {
        self.inner.set("is2D", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMPointReadOnly {
    inner: emlite::Val,
}
impl FromVal for DOMPointReadOnly {
    fn from_val(v: &emlite::Val) -> Self {
        DOMPointReadOnly {
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
impl core::ops::Deref for DOMPointReadOnly {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMPointReadOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMPointReadOnly {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMPointReadOnly {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DOMPointReadOnly> for emlite::Val {
    fn from(s: DOMPointReadOnly) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DOMPointReadOnly);

impl DOMPointReadOnly {
    pub fn new0() -> DOMPointReadOnly {
        Self {
            inner: emlite::Val::global("DOMPointReadOnly")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(x: f64) -> DOMPointReadOnly {
        Self {
            inner: emlite::Val::global("DOMPointReadOnly")
                .new(&[x.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(x: f64, y: f64) -> DOMPointReadOnly {
        Self {
            inner: emlite::Val::global("DOMPointReadOnly")
                .new(&[x.into(), y.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new3(x: f64, y: f64, z: f64) -> DOMPointReadOnly {
        Self {
            inner: emlite::Val::global("DOMPointReadOnly")
                .new(&[x.into(), y.into(), z.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new4(x: f64, y: f64, z: f64, w: f64) -> DOMPointReadOnly {
        Self {
            inner: emlite::Val::global("DOMPointReadOnly")
                .new(&[x.into(), y.into(), z.into(), w.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl DOMPointReadOnly {
    pub fn from_point0() -> DOMPointReadOnly {
        emlite::Val::global("dompointreadonly")
            .call("fromPoint", &[])
            .as_::<DOMPointReadOnly>()
    }

    pub fn from_point1(other: DOMPointInit) -> DOMPointReadOnly {
        emlite::Val::global("dompointreadonly")
            .call("fromPoint", &[other.into()])
            .as_::<DOMPointReadOnly>()
    }
}
impl DOMPointReadOnly {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl DOMPointReadOnly {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl DOMPointReadOnly {
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }
}
impl DOMPointReadOnly {
    pub fn w(&self) -> f64 {
        self.inner.get("w").as_::<f64>()
    }
}
impl DOMPointReadOnly {
    pub fn matrix_transform0(&self) -> DOMPoint {
        self.inner.call("matrixTransform", &[]).as_::<DOMPoint>()
    }

    pub fn matrix_transform1(&self, matrix: DOMMatrixInit) -> DOMPoint {
        self.inner
            .call("matrixTransform", &[matrix.into()])
            .as_::<DOMPoint>()
    }
}
impl DOMPointReadOnly {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
