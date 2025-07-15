use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathClamp {
    inner: CSSMathValue,
}
impl FromVal for CSSMathClamp {
    fn from_val(v: &emlite::Val) -> Self {
        CSSMathClamp { inner: CSSMathValue::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSMathClamp {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMathClamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSMathClamp {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSMathClamp {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSMathClamp> for emlite::Val {
    fn from(s: CSSMathClamp) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSMathClamp);



impl CSSMathClamp {
    pub fn new(lower: Any, value: Any, upper: Any) -> CSSMathClamp {
        Self {
            inner: emlite::Val::global("CSSMathClamp").new(&[lower.into(), value.into(), upper.into()]).as_::<CSSMathValue>(),
        }
    }

}
impl CSSMathClamp {
    pub fn lower(&self) -> CSSNumericValue {
        self.inner.get("lower").as_::<CSSNumericValue>()
    }

}
impl CSSMathClamp {
    pub fn value(&self) -> CSSNumericValue {
        self.inner.get("value").as_::<CSSNumericValue>()
    }

}
impl CSSMathClamp {
    pub fn upper(&self) -> CSSNumericValue {
        self.inner.get("upper").as_::<CSSNumericValue>()
    }

}
