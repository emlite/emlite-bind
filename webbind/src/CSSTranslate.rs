use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSTranslate {
    inner: CSSTransformComponent,
}
impl FromVal for CSSTranslate {
    fn from_val(v: &emlite::Val) -> Self {
        CSSTranslate {
            inner: CSSTransformComponent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSTranslate {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSTranslate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSTranslate> for emlite::Val {
    fn from(s: CSSTranslate) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSTranslate {
    pub fn new0(x: CSSNumericValue, y: CSSNumericValue) -> CSSTranslate {
        Self {
            inner: emlite::Val::global("CSSTranslate")
                .new(&[x.into(), y.into()])
                .as_::<CSSTransformComponent>(),
        }
    }

    pub fn new1(x: CSSNumericValue, y: CSSNumericValue, z: CSSNumericValue) -> CSSTranslate {
        Self {
            inner: emlite::Val::global("CSSTranslate")
                .new(&[x.into(), y.into(), z.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
impl CSSTranslate {
    pub fn x(&self) -> CSSNumericValue {
        self.inner.get("x").as_::<CSSNumericValue>()
    }

    pub fn set_x(&mut self, value: CSSNumericValue) {
        self.inner.set("x", value);
    }
}
impl CSSTranslate {
    pub fn y(&self) -> CSSNumericValue {
        self.inner.get("y").as_::<CSSNumericValue>()
    }

    pub fn set_y(&mut self, value: CSSNumericValue) {
        self.inner.set("y", value);
    }
}
impl CSSTranslate {
    pub fn z(&self) -> CSSNumericValue {
        self.inner.get("z").as_::<CSSNumericValue>()
    }

    pub fn set_z(&mut self, value: CSSNumericValue) {
        self.inner.set("z", value);
    }
}
