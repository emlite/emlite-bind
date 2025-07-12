use super::*;

#[derive(Clone, Debug)]
pub struct CSSLCH {
    inner: CSSColorValue,
}
impl FromVal for CSSLCH {
    fn from_val(v: &emlite::Val) -> Self {
        CSSLCH {
            inner: CSSColorValue::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSLCH {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSLCH {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSLCH> for emlite::Val {
    fn from(s: CSSLCH) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSLCH {
    pub fn new0(l: jsbind::Any, c: jsbind::Any, h: jsbind::Any) -> CSSLCH {
        Self {
            inner: emlite::Val::global("CSSLCH")
                .new(&[l.into(), c.into(), h.into()])
                .as_::<CSSColorValue>(),
        }
    }

    pub fn new1(l: jsbind::Any, c: jsbind::Any, h: jsbind::Any, alpha: jsbind::Any) -> CSSLCH {
        Self {
            inner: emlite::Val::global("CSSLCH")
                .new(&[l.into(), c.into(), h.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
impl CSSLCH {
    pub fn l(&self) -> jsbind::Any {
        self.inner.get("l").as_::<jsbind::Any>()
    }

    pub fn set_l(&mut self, value: jsbind::Any) {
        self.inner.set("l", value);
    }
}
impl CSSLCH {
    pub fn c(&self) -> jsbind::Any {
        self.inner.get("c").as_::<jsbind::Any>()
    }

    pub fn set_c(&mut self, value: jsbind::Any) {
        self.inner.set("c", value);
    }
}
impl CSSLCH {
    pub fn h(&self) -> jsbind::Any {
        self.inner.get("h").as_::<jsbind::Any>()
    }

    pub fn set_h(&mut self, value: jsbind::Any) {
        self.inner.set("h", value);
    }
}
impl CSSLCH {
    pub fn alpha(&self) -> jsbind::Any {
        self.inner.get("alpha").as_::<jsbind::Any>()
    }

    pub fn set_alpha(&mut self, value: jsbind::Any) {
        self.inner.set("alpha", value);
    }
}
