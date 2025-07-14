use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSKeywordValue {
    inner: CSSStyleValue,
}
impl FromVal for CSSKeywordValue {
    fn from_val(v: &emlite::Val) -> Self {
        CSSKeywordValue {
            inner: CSSStyleValue::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSKeywordValue {
    type Target = CSSStyleValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSKeywordValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSKeywordValue> for emlite::Val {
    fn from(s: CSSKeywordValue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSKeywordValue {
    pub fn new(value: jsbind::USVString) -> CSSKeywordValue {
        Self {
            inner: emlite::Val::global("CSSKeywordValue")
                .new(&[value.into()])
                .as_::<CSSStyleValue>(),
        }
    }
}
impl CSSKeywordValue {
    pub fn value(&self) -> jsbind::USVString {
        self.inner.get("value").as_::<jsbind::USVString>()
    }

    pub fn set_value(&mut self, value: jsbind::USVString) {
        self.inner.set("value", value);
    }
}
