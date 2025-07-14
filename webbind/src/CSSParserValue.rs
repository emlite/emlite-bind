use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSParserValue {
    inner: emlite::Val,
}
impl FromVal for CSSParserValue {
    fn from_val(v: &emlite::Val) -> Self {
        CSSParserValue {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSParserValue {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSParserValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSParserValue> for emlite::Val {
    fn from(s: CSSParserValue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
