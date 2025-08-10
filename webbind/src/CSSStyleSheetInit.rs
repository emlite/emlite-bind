use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStyleSheetInit {
    inner: Any,
}
impl FromVal for CSSStyleSheetInit {
    fn from_val(v: &Any) -> Self {
        CSSStyleSheetInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSStyleSheetInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSStyleSheetInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSStyleSheetInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSStyleSheetInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSStyleSheetInit> for Any {
    fn from(s: CSSStyleSheetInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSStyleSheetInit> for Any {
    fn from(s: &CSSStyleSheetInit) -> Any {
        s.inner.clone()
    }
}

impl CSSStyleSheetInit {
    pub fn base_url(&self) -> JsString {
        self.inner.get("baseURL").as_::<JsString>()
    }

    pub fn set_base_url(&mut self, value: &JsString) {
        self.inner.set("baseURL", value);
    }
}
impl CSSStyleSheetInit {
    pub fn media(&self) -> Any {
        self.inner.get("media").as_::<Any>()
    }

    pub fn set_media(&mut self, value: &Any) {
        self.inner.set("media", value);
    }
}
impl CSSStyleSheetInit {
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
