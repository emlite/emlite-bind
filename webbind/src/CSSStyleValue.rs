use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStyleValue {
    inner: emlite::Val,
}
impl FromVal for CSSStyleValue {
    fn from_val(v: &emlite::Val) -> Self {
        CSSStyleValue {
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
impl core::ops::Deref for CSSStyleValue {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSStyleValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSStyleValue {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSStyleValue {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSStyleValue> for emlite::Val {
    fn from(s: CSSStyleValue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSStyleValue> for emlite::Val {
    fn from(s: &CSSStyleValue) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSStyleValue);

impl CSSStyleValue {
    pub fn parse(property: &str, css_text: &str) -> CSSStyleValue {
        emlite::Val::global("CSSStyleValue")
            .call("parse", &[property.into(), css_text.into()])
            .as_::<CSSStyleValue>()
    }
}
impl CSSStyleValue {
    pub fn parse_all(property: &str, css_text: &str) -> Sequence<CSSStyleValue> {
        emlite::Val::global("CSSStyleValue")
            .call("parseAll", &[property.into(), css_text.into()])
            .as_::<Sequence<CSSStyleValue>>()
    }
}
