use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CloseEventInit {
    inner: Any,
}
impl FromVal for CloseEventInit {
    fn from_val(v: &Any) -> Self {
        CloseEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CloseEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CloseEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CloseEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CloseEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CloseEventInit> for Any {
    fn from(s: CloseEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CloseEventInit> for Any {
    fn from(s: &CloseEventInit) -> Any {
        s.inner.clone()
    }
}

impl CloseEventInit {
    pub fn was_clean(&self) -> bool {
        self.inner.get("wasClean").as_::<bool>()
    }

    pub fn set_was_clean(&mut self, value: bool) {
        self.inner.set("wasClean", value);
    }
}
impl CloseEventInit {
    pub fn code(&self) -> u16 {
        self.inner.get("code").as_::<u16>()
    }

    pub fn set_code(&mut self, value: u16) {
        self.inner.set("code", value);
    }
}
impl CloseEventInit {
    pub fn reason(&self) -> JsString {
        self.inner.get("reason").as_::<JsString>()
    }

    pub fn set_reason(&mut self, value: &JsString) {
        self.inner.set("reason", value);
    }
}
