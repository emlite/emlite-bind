use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSHSL {
    inner: CSSColorValue,
}
impl FromVal for CSSHSL {
    fn from_val(v: &emlite::Val) -> Self {
        CSSHSL {
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
impl core::ops::Deref for CSSHSL {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSHSL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSHSL {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSHSL {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSHSL> for emlite::Val {
    fn from(s: CSSHSL) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSHSL);

impl CSSHSL {
    pub fn new0(h: Any, s: Any, l: Any) -> CSSHSL {
        Self {
            inner: emlite::Val::global("CSSHSL")
                .new(&[h.into(), s.into(), l.into()])
                .as_::<CSSColorValue>(),
        }
    }

    pub fn new1(h: Any, s: Any, l: Any, alpha: Any) -> CSSHSL {
        Self {
            inner: emlite::Val::global("CSSHSL")
                .new(&[h.into(), s.into(), l.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
impl CSSHSL {
    pub fn h(&self) -> Any {
        self.inner.get("h").as_::<Any>()
    }

    pub fn set_h(&mut self, value: Any) {
        self.inner.set("h", value);
    }
}
impl CSSHSL {
    pub fn s(&self) -> Any {
        self.inner.get("s").as_::<Any>()
    }

    pub fn set_s(&mut self, value: Any) {
        self.inner.set("s", value);
    }
}
impl CSSHSL {
    pub fn l(&self) -> Any {
        self.inner.get("l").as_::<Any>()
    }

    pub fn set_l(&mut self, value: Any) {
        self.inner.set("l", value);
    }
}
impl CSSHSL {
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    pub fn set_alpha(&mut self, value: Any) {
        self.inner.set("alpha", value);
    }
}
