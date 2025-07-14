use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathMin {
    inner: CSSMathValue,
}
impl FromVal for CSSMathMin {
    fn from_val(v: &emlite::Val) -> Self {
        CSSMathMin {
            inner: CSSMathValue::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSMathMin {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMathMin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSMathMin {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSMathMin {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSMathMin> for emlite::Val {
    fn from(s: CSSMathMin) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSMathMin);

impl CSSMathMin {
    pub fn new(args: jsbind::Any) -> CSSMathMin {
        Self {
            inner: emlite::Val::global("CSSMathMin")
                .new(&[args.into()])
                .as_::<CSSMathValue>(),
        }
    }
}
impl CSSMathMin {
    pub fn values(&self) -> CSSNumericArray {
        self.inner.get("values").as_::<CSSNumericArray>()
    }
}
