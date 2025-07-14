use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSLab {
    inner: CSSColorValue,
}
impl FromVal for CSSLab {
    fn from_val(v: &emlite::Val) -> Self {
        CSSLab {
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
impl core::ops::Deref for CSSLab {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSLab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSLab> for emlite::Val {
    fn from(s: CSSLab) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSLab {
    pub fn new0(l: jsbind::Any, a: jsbind::Any, b: jsbind::Any) -> CSSLab {
        Self {
            inner: emlite::Val::global("CSSLab")
                .new(&[l.into(), a.into(), b.into()])
                .as_::<CSSColorValue>(),
        }
    }

    pub fn new1(l: jsbind::Any, a: jsbind::Any, b: jsbind::Any, alpha: jsbind::Any) -> CSSLab {
        Self {
            inner: emlite::Val::global("CSSLab")
                .new(&[l.into(), a.into(), b.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
impl CSSLab {
    pub fn l(&self) -> jsbind::Any {
        self.inner.get("l").as_::<jsbind::Any>()
    }

    pub fn set_l(&mut self, value: jsbind::Any) {
        self.inner.set("l", value);
    }
}
impl CSSLab {
    pub fn a(&self) -> jsbind::Any {
        self.inner.get("a").as_::<jsbind::Any>()
    }

    pub fn set_a(&mut self, value: jsbind::Any) {
        self.inner.set("a", value);
    }
}
impl CSSLab {
    pub fn b(&self) -> jsbind::Any {
        self.inner.get("b").as_::<jsbind::Any>()
    }

    pub fn set_b(&mut self, value: jsbind::Any) {
        self.inner.set("b", value);
    }
}
impl CSSLab {
    pub fn alpha(&self) -> jsbind::Any {
        self.inner.get("alpha").as_::<jsbind::Any>()
    }

    pub fn set_alpha(&mut self, value: jsbind::Any) {
        self.inner.set("alpha", value);
    }
}
