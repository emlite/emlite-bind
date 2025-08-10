use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ResponseInit {
    inner: Any,
}
impl FromVal for ResponseInit {
    fn from_val(v: &Any) -> Self {
        ResponseInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ResponseInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ResponseInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ResponseInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ResponseInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ResponseInit> for Any {
    fn from(s: ResponseInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ResponseInit> for Any {
    fn from(s: &ResponseInit) -> Any {
        s.inner.clone()
    }
}

impl ResponseInit {
    pub fn status(&self) -> u16 {
        self.inner.get("status").as_::<u16>()
    }

    pub fn set_status(&mut self, value: u16) {
        self.inner.set("status", value);
    }
}
impl ResponseInit {
    pub fn status_text(&self) -> JsString {
        self.inner.get("statusText").as_::<JsString>()
    }

    pub fn set_status_text(&mut self, value: &JsString) {
        self.inner.set("statusText", value);
    }
}
impl ResponseInit {
    pub fn headers(&self) -> Any {
        self.inner.get("headers").as_::<Any>()
    }

    pub fn set_headers(&mut self, value: &Any) {
        self.inner.set("headers", value);
    }
}
