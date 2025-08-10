use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TokenBinding {
    inner: Any,
}
impl FromVal for TokenBinding {
    fn from_val(v: &Any) -> Self {
        TokenBinding { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TokenBinding {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TokenBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TokenBinding {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TokenBinding {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TokenBinding> for Any {
    fn from(s: TokenBinding) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TokenBinding> for Any {
    fn from(s: &TokenBinding) -> Any {
        s.inner.clone()
    }
}

impl TokenBinding {
    pub fn status(&self) -> JsString {
        self.inner.get("status").as_::<JsString>()
    }

    pub fn set_status(&mut self, value: &JsString) {
        self.inner.set("status", value);
    }
}
impl TokenBinding {
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
