use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSOKLCH {
    inner: CSSColorValue,
}
impl FromVal for CSSOKLCH {
    fn from_val(v: &emlite::Val) -> Self {
        CSSOKLCH {
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
impl core::ops::Deref for CSSOKLCH {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSOKLCH {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSOKLCH {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSOKLCH {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSOKLCH> for emlite::Val {
    fn from(s: CSSOKLCH) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSOKLCH);

impl CSSOKLCH {
    pub fn new0(l: jsbind::Any, c: jsbind::Any, h: jsbind::Any) -> CSSOKLCH {
        Self {
            inner: emlite::Val::global("CSSOKLCH")
                .new(&[l.into(), c.into(), h.into()])
                .as_::<CSSColorValue>(),
        }
    }

    pub fn new1(l: jsbind::Any, c: jsbind::Any, h: jsbind::Any, alpha: jsbind::Any) -> CSSOKLCH {
        Self {
            inner: emlite::Val::global("CSSOKLCH")
                .new(&[l.into(), c.into(), h.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
impl CSSOKLCH {
    pub fn l(&self) -> jsbind::Any {
        self.inner.get("l").as_::<jsbind::Any>()
    }

    pub fn set_l(&mut self, value: jsbind::Any) {
        self.inner.set("l", value);
    }
}
impl CSSOKLCH {
    pub fn c(&self) -> jsbind::Any {
        self.inner.get("c").as_::<jsbind::Any>()
    }

    pub fn set_c(&mut self, value: jsbind::Any) {
        self.inner.set("c", value);
    }
}
impl CSSOKLCH {
    pub fn h(&self) -> jsbind::Any {
        self.inner.get("h").as_::<jsbind::Any>()
    }

    pub fn set_h(&mut self, value: jsbind::Any) {
        self.inner.set("h", value);
    }
}
impl CSSOKLCH {
    pub fn alpha(&self) -> jsbind::Any {
        self.inner.get("alpha").as_::<jsbind::Any>()
    }

    pub fn set_alpha(&mut self, value: jsbind::Any) {
        self.inner.set("alpha", value);
    }
}
