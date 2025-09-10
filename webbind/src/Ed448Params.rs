use super::*;

/// The Ed448Params dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Ed448Params {
    inner: Any,
}

impl FromVal for Ed448Params {
    fn from_val(v: &Any) -> Self {
        Ed448Params { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Ed448Params {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Ed448Params {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Ed448Params {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Ed448Params {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Ed448Params> for Any {
    fn from(s: Ed448Params) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Ed448Params> for Any {
    fn from(s: &Ed448Params) -> Any {
        s.inner.clone()
    }
}

impl Ed448Params {
    /// Getter of the `context` attribute.
    pub fn context(&self) -> Any {
        self.inner.get("context").as_::<Any>()
    }

    /// Setter of the `context` attribute.
    pub fn set_context(&mut self, value: &Any) {
        self.inner.set("context", value);
    }
}
