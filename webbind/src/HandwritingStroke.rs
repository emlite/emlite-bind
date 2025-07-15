use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingPoint {
    inner: emlite::Val,
}
impl FromVal for HandwritingPoint {
    fn from_val(v: &emlite::Val) -> Self {
        HandwritingPoint { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HandwritingPoint {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HandwritingPoint {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HandwritingPoint {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HandwritingPoint> for emlite::Val {
    fn from(s: HandwritingPoint) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HandwritingPoint> for emlite::Val {
    fn from(s: &HandwritingPoint) -> emlite::Val {
        s.inner.clone()
    }
}

impl HandwritingPoint {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl HandwritingPoint {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl HandwritingPoint {
    pub fn t(&self) -> Any {
        self.inner.get("t").as_::<Any>()
    }

    pub fn set_t(&mut self, value: &Any) {
        self.inner.set("t", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingStroke {
    inner: emlite::Val,
}
impl FromVal for HandwritingStroke {
    fn from_val(v: &emlite::Val) -> Self {
        HandwritingStroke {
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
impl core::ops::Deref for HandwritingStroke {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingStroke {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HandwritingStroke {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HandwritingStroke {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HandwritingStroke> for emlite::Val {
    fn from(s: HandwritingStroke) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HandwritingStroke> for emlite::Val {
    fn from(s: &HandwritingStroke) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HandwritingStroke);

impl HandwritingStroke {
    pub fn new() -> HandwritingStroke {
        Self {
            inner: emlite::Val::global("HandwritingStroke")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl HandwritingStroke {
    pub fn add_point(&self, point: &HandwritingPoint) -> Undefined {
        self.inner
            .call("addPoint", &[point.into()])
            .as_::<Undefined>()
    }
}
impl HandwritingStroke {
    pub fn get_points(&self) -> Sequence<HandwritingPoint> {
        self.inner
            .call("getPoints", &[])
            .as_::<Sequence<HandwritingPoint>>()
    }
}
impl HandwritingStroke {
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
