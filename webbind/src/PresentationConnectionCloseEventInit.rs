use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationConnectionCloseEventInit {
    inner: Any,
}
impl FromVal for PresentationConnectionCloseEventInit {
    fn from_val(v: &Any) -> Self {
        PresentationConnectionCloseEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PresentationConnectionCloseEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PresentationConnectionCloseEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PresentationConnectionCloseEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PresentationConnectionCloseEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PresentationConnectionCloseEventInit> for Any {
    fn from(s: PresentationConnectionCloseEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PresentationConnectionCloseEventInit> for Any {
    fn from(s: &PresentationConnectionCloseEventInit) -> Any {
        s.inner.clone()
    }
}

impl PresentationConnectionCloseEventInit {
    pub fn reason(&self) -> PresentationConnectionCloseReason {
        self.inner
            .get("reason")
            .as_::<PresentationConnectionCloseReason>()
    }

    pub fn set_reason(&mut self, value: &PresentationConnectionCloseReason) {
        self.inner.set("reason", value);
    }
}
impl PresentationConnectionCloseEventInit {
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }

    pub fn set_message(&mut self, value: &JsString) {
        self.inner.set("message", value);
    }
}
