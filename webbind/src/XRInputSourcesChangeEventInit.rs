use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRInputSourcesChangeEventInit {
    inner: Any,
}
impl FromVal for XRInputSourcesChangeEventInit {
    fn from_val(v: &Any) -> Self {
        XRInputSourcesChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRInputSourcesChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRInputSourcesChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRInputSourcesChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRInputSourcesChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRInputSourcesChangeEventInit> for Any {
    fn from(s: XRInputSourcesChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRInputSourcesChangeEventInit> for Any {
    fn from(s: &XRInputSourcesChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl XRInputSourcesChangeEventInit {
    pub fn session(&self) -> XRSession {
        self.inner.get("session").as_::<XRSession>()
    }

    pub fn set_session(&mut self, value: &XRSession) {
        self.inner.set("session", value);
    }
}
impl XRInputSourcesChangeEventInit {
    pub fn added(&self) -> TypedArray<XRInputSource> {
        self.inner.get("added").as_::<TypedArray<XRInputSource>>()
    }

    pub fn set_added(&mut self, value: &TypedArray<XRInputSource>) {
        self.inner.set("added", value);
    }
}
impl XRInputSourcesChangeEventInit {
    pub fn removed(&self) -> TypedArray<XRInputSource> {
        self.inner.get("removed").as_::<TypedArray<XRInputSource>>()
    }

    pub fn set_removed(&mut self, value: &TypedArray<XRInputSource>) {
        self.inner.set("removed", value);
    }
}
