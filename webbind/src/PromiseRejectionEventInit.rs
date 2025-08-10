use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PromiseRejectionEventInit {
    inner: Any,
}
impl FromVal for PromiseRejectionEventInit {
    fn from_val(v: &Any) -> Self {
        PromiseRejectionEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PromiseRejectionEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PromiseRejectionEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PromiseRejectionEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PromiseRejectionEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PromiseRejectionEventInit> for Any {
    fn from(s: PromiseRejectionEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PromiseRejectionEventInit> for Any {
    fn from(s: &PromiseRejectionEventInit) -> Any {
        s.inner.clone()
    }
}

impl PromiseRejectionEventInit {
    pub fn promise(&self) -> Object {
        self.inner.get("promise").as_::<Object>()
    }

    pub fn set_promise(&mut self, value: &Object) {
        self.inner.set("promise", value);
    }
}
impl PromiseRejectionEventInit {
    pub fn reason(&self) -> Any {
        self.inner.get("reason").as_::<Any>()
    }

    pub fn set_reason(&mut self, value: &Any) {
        self.inner.set("reason", value);
    }
}
