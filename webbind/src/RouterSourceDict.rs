use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RouterSourceDict {
    inner: Any,
}
impl FromVal for RouterSourceDict {
    fn from_val(v: &Any) -> Self {
        RouterSourceDict { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RouterSourceDict {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RouterSourceDict {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RouterSourceDict {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RouterSourceDict {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RouterSourceDict> for Any {
    fn from(s: RouterSourceDict) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RouterSourceDict> for Any {
    fn from(s: &RouterSourceDict) -> Any {
        s.inner.clone()
    }
}

impl RouterSourceDict {
    pub fn cache_name(&self) -> JsString {
        self.inner.get("cacheName").as_::<JsString>()
    }

    pub fn set_cache_name(&mut self, value: &JsString) {
        self.inner.set("cacheName", value);
    }
}
