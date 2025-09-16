use super::*;

/// The ContextParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContextParams {
    inner: Any,
}

impl FromVal for ContextParams {
    fn from_val(v: &Any) -> Self {
        ContextParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ContextParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ContextParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ContextParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ContextParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ContextParams> for Any {
    fn from(s: ContextParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ContextParams> for Any {
    fn from(s: &ContextParams) -> Any {
        s.inner.clone()
    }
}

impl ContextParams {
    /// Getter of the `context` attribute.
    pub fn context(&self) -> Any {
        self.inner.get("context").as_::<Any>()
    }

    /// Setter of the `context` attribute.
    pub fn set_context(&mut self, value: &Any) {
        self.inner.set("context", value);
    }
}
