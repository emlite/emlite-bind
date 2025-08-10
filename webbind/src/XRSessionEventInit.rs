use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSessionEventInit {
    inner: Any,
}
impl FromVal for XRSessionEventInit {
    fn from_val(v: &Any) -> Self {
        XRSessionEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRSessionEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRSessionEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRSessionEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRSessionEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRSessionEventInit> for Any {
    fn from(s: XRSessionEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRSessionEventInit> for Any {
    fn from(s: &XRSessionEventInit) -> Any {
        s.inner.clone()
    }
}

impl XRSessionEventInit {
    pub fn session(&self) -> XRSession {
        self.inner.get("session").as_::<XRSession>()
    }

    pub fn set_session(&mut self, value: &XRSession) {
        self.inner.set("session", value);
    }
}
