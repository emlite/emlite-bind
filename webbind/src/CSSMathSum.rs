use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathSum {
    inner: CSSMathValue,
}
impl FromVal for CSSMathSum {
    fn from_val(v: &emlite::Val) -> Self {
        CSSMathSum {
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
impl core::ops::Deref for CSSMathSum {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMathSum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSMathSum {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSMathSum {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSMathSum> for emlite::Val {
    fn from(s: CSSMathSum) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSMathSum);

impl CSSMathSum {
    pub fn new(args: Any) -> CSSMathSum {
        Self {
            inner: emlite::Val::global("CSSMathSum")
                .new(&[args.into()])
                .as_::<CSSMathValue>(),
        }
    }
}
impl CSSMathSum {
    pub fn values(&self) -> CSSNumericArray {
        self.inner.get("values").as_::<CSSNumericArray>()
    }
}
