use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for HandwritingPoint {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HandwritingPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HandwritingPoint> for emlite::Val {
    fn from(s: HandwritingPoint) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
    pub fn t(&self) -> jsbind::Any {
        self.inner.get("t").as_::<jsbind::Any>()
    }

    pub fn set_t(&mut self, value: jsbind::Any) {
        self.inner.set("t", value);
    }
}
#[derive(Clone, Debug)]
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
impl std::ops::Deref for HandwritingStroke {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HandwritingStroke {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HandwritingStroke> for emlite::Val {
    fn from(s: HandwritingStroke) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn add_point(&self, point: HandwritingPoint) -> jsbind::Undefined {
        self.inner
            .call("addPoint", &[point.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HandwritingStroke {
    pub fn get_points(&self) -> jsbind::Sequence<HandwritingPoint> {
        self.inner
            .call("getPoints", &[])
            .as_::<jsbind::Sequence<HandwritingPoint>>()
    }
}
impl HandwritingStroke {
    pub fn clear(&self) -> jsbind::Undefined {
        self.inner.call("clear", &[]).as_::<jsbind::Undefined>()
    }
}
