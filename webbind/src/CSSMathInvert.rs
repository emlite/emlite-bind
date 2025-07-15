use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathInvert {
    inner: CSSMathValue,
}
impl FromVal for CSSMathInvert {
    fn from_val(v: &emlite::Val) -> Self {
        CSSMathInvert { inner: CSSMathValue::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSMathInvert {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMathInvert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSMathInvert {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSMathInvert {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSMathInvert> for emlite::Val {
    fn from(s: CSSMathInvert) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSMathInvert);



impl CSSMathInvert {
    pub fn new(arg: Any) -> CSSMathInvert {
        Self {
            inner: emlite::Val::global("CSSMathInvert").new(&[arg.into()]).as_::<CSSMathValue>(),
        }
    }

}
impl CSSMathInvert {
    pub fn value(&self) -> CSSNumericValue {
        self.inner.get("value").as_::<CSSNumericValue>()
    }

}
