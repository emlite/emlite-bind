use super::*;

/// The SetHTMLOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SetHTMLOptions {
    inner: Any,
}

impl FromVal for SetHTMLOptions {
    fn from_val(v: &Any) -> Self {
        SetHTMLOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SetHTMLOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SetHTMLOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SetHTMLOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SetHTMLOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SetHTMLOptions> for Any {
    fn from(s: SetHTMLOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SetHTMLOptions> for Any {
    fn from(s: &SetHTMLOptions) -> Any {
        s.inner.clone()
    }
}

impl SetHTMLOptions {
    /// Getter of the `sanitizer` attribute.
    pub fn sanitizer(&self) -> Any {
        self.inner.get("sanitizer").as_::<Any>()
    }

    /// Setter of the `sanitizer` attribute.
    pub fn set_sanitizer(&mut self, value: &Any) {
        self.inner.set("sanitizer", value);
    }
}
