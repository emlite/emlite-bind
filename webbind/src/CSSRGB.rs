use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSRGB {
    inner: CSSColorValue,
}
impl FromVal for CSSRGB {
    fn from_val(v: &emlite::Val) -> Self {
        CSSRGB { inner: CSSColorValue::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSRGB {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSRGB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSRGB {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSRGB {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSRGB> for emlite::Val {
    fn from(s: CSSRGB) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSRGB);



impl CSSRGB {
    pub fn new0(r: Any, g: Any, b: Any) -> CSSRGB {
        Self {
            inner: emlite::Val::global("CSSRGB").new(&[r.into(), g.into(), b.into()]).as_::<CSSColorValue>(),
        }
    }

    pub fn new1(r: Any, g: Any, b: Any, alpha: Any) -> CSSRGB {
        Self {
            inner: emlite::Val::global("CSSRGB").new(&[r.into(), g.into(), b.into(), alpha.into()]).as_::<CSSColorValue>(),
        }
    }

}
impl CSSRGB {
    pub fn r(&self) -> Any {
        self.inner.get("r").as_::<Any>()
    }

    pub fn set_r(&mut self, value: Any) {
        self.inner.set("r", value);
    }

}
impl CSSRGB {
    pub fn g(&self) -> Any {
        self.inner.get("g").as_::<Any>()
    }

    pub fn set_g(&mut self, value: Any) {
        self.inner.set("g", value);
    }

}
impl CSSRGB {
    pub fn b(&self) -> Any {
        self.inner.get("b").as_::<Any>()
    }

    pub fn set_b(&mut self, value: Any) {
        self.inner.set("b", value);
    }

}
impl CSSRGB {
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    pub fn set_alpha(&mut self, value: Any) {
        self.inner.set("alpha", value);
    }

}
