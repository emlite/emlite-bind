use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathNegate {
    inner: CSSMathValue,
}
impl FromVal for CSSMathNegate {
    fn from_val(v: &emlite::Val) -> Self {
        CSSMathNegate {
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
impl core::ops::Deref for CSSMathNegate {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMathNegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSMathNegate {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSMathNegate {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSMathNegate> for emlite::Val {
    fn from(s: CSSMathNegate) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSMathNegate> for emlite::Val {
    fn from(s: &CSSMathNegate) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSMathNegate);

impl CSSMathNegate {
    pub fn new(arg: Any) -> CSSMathNegate {
        Self {
            inner: emlite::Val::global("CSSMathNegate")
                .new(&[arg.into()])
                .as_::<CSSMathValue>(),
        }
    }
}
impl CSSMathNegate {
    pub fn value(&self) -> CSSNumericValue {
        self.inner.get("value").as_::<CSSNumericValue>()
    }
}
