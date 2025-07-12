use super::*;

#[derive(Clone, Debug)]
pub struct CSSHWB {
    inner: CSSColorValue,
}
impl FromVal for CSSHWB {
    fn from_val(v: &emlite::Val) -> Self {
        CSSHWB {
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
impl std::ops::Deref for CSSHWB {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSHWB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSHWB> for emlite::Val {
    fn from(s: CSSHWB) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSHWB {
    pub fn new0(h: CSSNumericValue, w: jsbind::Any, b: jsbind::Any) -> CSSHWB {
        Self {
            inner: emlite::Val::global("CSSHWB")
                .new(&[h.into(), w.into(), b.into()])
                .as_::<CSSColorValue>(),
        }
    }

    pub fn new1(h: CSSNumericValue, w: jsbind::Any, b: jsbind::Any, alpha: jsbind::Any) -> CSSHWB {
        Self {
            inner: emlite::Val::global("CSSHWB")
                .new(&[h.into(), w.into(), b.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
impl CSSHWB {
    pub fn h(&self) -> CSSNumericValue {
        self.inner.get("h").as_::<CSSNumericValue>()
    }

    pub fn set_h(&mut self, value: CSSNumericValue) {
        self.inner.set("h", value);
    }
}
impl CSSHWB {
    pub fn w(&self) -> jsbind::Any {
        self.inner.get("w").as_::<jsbind::Any>()
    }

    pub fn set_w(&mut self, value: jsbind::Any) {
        self.inner.set("w", value);
    }
}
impl CSSHWB {
    pub fn b(&self) -> jsbind::Any {
        self.inner.get("b").as_::<jsbind::Any>()
    }

    pub fn set_b(&mut self, value: jsbind::Any) {
        self.inner.set("b", value);
    }
}
impl CSSHWB {
    pub fn alpha(&self) -> jsbind::Any {
        self.inner.get("alpha").as_::<jsbind::Any>()
    }

    pub fn set_alpha(&mut self, value: jsbind::Any) {
        self.inner.set("alpha", value);
    }
}
