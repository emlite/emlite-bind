use super::*;

/// The SubmitEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SubmitEventInit {
    inner: Any,
}

impl FromVal for SubmitEventInit {
    fn from_val(v: &Any) -> Self {
        SubmitEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SubmitEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SubmitEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SubmitEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SubmitEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SubmitEventInit> for Any {
    fn from(s: SubmitEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SubmitEventInit> for Any {
    fn from(s: &SubmitEventInit) -> Any {
        s.inner.clone()
    }
}

impl SubmitEventInit {
    /// Getter of the `submitter` attribute.
    pub fn submitter(&self) -> HTMLElement {
        self.inner.get("submitter").as_::<HTMLElement>()
    }

    /// Setter of the `submitter` attribute.
    pub fn set_submitter(&mut self, value: &HTMLElement) {
        self.inner.set("submitter", value);
    }
}
