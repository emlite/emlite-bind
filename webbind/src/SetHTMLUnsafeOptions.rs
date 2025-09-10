use super::*;

/// The SetHTMLUnsafeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SetHTMLUnsafeOptions {
    inner: Any,
}

impl FromVal for SetHTMLUnsafeOptions {
    fn from_val(v: &Any) -> Self {
        SetHTMLUnsafeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SetHTMLUnsafeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SetHTMLUnsafeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SetHTMLUnsafeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SetHTMLUnsafeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SetHTMLUnsafeOptions> for Any {
    fn from(s: SetHTMLUnsafeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SetHTMLUnsafeOptions> for Any {
    fn from(s: &SetHTMLUnsafeOptions) -> Any {
        s.inner.clone()
    }
}

impl SetHTMLUnsafeOptions {
    /// Getter of the `sanitizer` attribute.
    pub fn sanitizer(&self) -> Any {
        self.inner.get("sanitizer").as_::<Any>()
    }

    /// Setter of the `sanitizer` attribute.
    pub fn set_sanitizer(&mut self, value: &Any) {
        self.inner.set("sanitizer", value);
    }
}
