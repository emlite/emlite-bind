use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSMathValue {
    inner: CSSNumericValue,
}
impl FromVal for CSSMathValue {
    fn from_val(v: &emlite::Val) -> Self {
        CSSMathValue {
            inner: CSSNumericValue::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSMathValue {
    type Target = CSSNumericValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMathValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSMathValue> for emlite::Val {
    fn from(s: CSSMathValue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSMathValue {
    pub fn operator(&self) -> CSSMathOperator {
        self.inner.get("operator").as_::<CSSMathOperator>()
    }
}
