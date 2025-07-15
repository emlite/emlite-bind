use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSLCH {
    inner: CSSColorValue,
}
impl FromVal for CSSLCH {
    fn from_val(v: &emlite::Val) -> Self {
        CSSLCH { inner: CSSColorValue::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSLCH {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSLCH {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSLCH {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSLCH {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSLCH> for emlite::Val {
    fn from(s: CSSLCH) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSLCH);



impl CSSLCH {
    pub fn new0(l: Any, c: Any, h: Any) -> CSSLCH {
        Self {
            inner: emlite::Val::global("CSSLCH").new(&[l.into(), c.into(), h.into()]).as_::<CSSColorValue>(),
        }
    }

    pub fn new1(l: Any, c: Any, h: Any, alpha: Any) -> CSSLCH {
        Self {
            inner: emlite::Val::global("CSSLCH").new(&[l.into(), c.into(), h.into(), alpha.into()]).as_::<CSSColorValue>(),
        }
    }

}
impl CSSLCH {
    pub fn l(&self) -> Any {
        self.inner.get("l").as_::<Any>()
    }

    pub fn set_l(&mut self, value: Any) {
        self.inner.set("l", value);
    }

}
impl CSSLCH {
    pub fn c(&self) -> Any {
        self.inner.get("c").as_::<Any>()
    }

    pub fn set_c(&mut self, value: Any) {
        self.inner.set("c", value);
    }

}
impl CSSLCH {
    pub fn h(&self) -> Any {
        self.inner.get("h").as_::<Any>()
    }

    pub fn set_h(&mut self, value: Any) {
        self.inner.set("h", value);
    }

}
impl CSSLCH {
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    pub fn set_alpha(&mut self, value: Any) {
        self.inner.set("alpha", value);
    }

}
