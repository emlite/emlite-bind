use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathMax {
    inner: CSSMathValue,
}
impl FromVal for CSSMathMax {
    fn from_val(v: &emlite::Val) -> Self {
        CSSMathMax {
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
impl core::ops::Deref for CSSMathMax {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMathMax {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSMathMax {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSMathMax {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSMathMax> for emlite::Val {
    fn from(s: CSSMathMax) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSMathMax> for emlite::Val {
    fn from(s: &CSSMathMax) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSMathMax);

impl CSSMathMax {
    pub fn new(args: Any) -> CSSMathMax {
        Self {
            inner: emlite::Val::global("CSSMathMax")
                .new(&[args.into()])
                .as_::<CSSMathValue>(),
        }
    }
}
impl CSSMathMax {
    pub fn values(&self) -> CSSNumericArray {
        self.inner.get("values").as_::<CSSNumericArray>()
    }
}
