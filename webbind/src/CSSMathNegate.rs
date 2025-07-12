use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for CSSMathNegate {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSMathNegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSMathNegate> for emlite::Val {
    fn from(s: CSSMathNegate) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSMathNegate {
    pub fn new(arg: jsbind::Any) -> CSSMathNegate {
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
