use super::*;

/// The ClipboardUnsanitizedFormats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ClipboardUnsanitizedFormats {
    inner: Any,
}

impl FromVal for ClipboardUnsanitizedFormats {
    fn from_val(v: &Any) -> Self {
        ClipboardUnsanitizedFormats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ClipboardUnsanitizedFormats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ClipboardUnsanitizedFormats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ClipboardUnsanitizedFormats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ClipboardUnsanitizedFormats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ClipboardUnsanitizedFormats> for Any {
    fn from(s: ClipboardUnsanitizedFormats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ClipboardUnsanitizedFormats> for Any {
    fn from(s: &ClipboardUnsanitizedFormats) -> Any {
        s.inner.clone()
    }
}

impl ClipboardUnsanitizedFormats {
    /// Getter of the `unsanitized` attribute.
    pub fn unsanitized(&self) -> TypedArray<JsString> {
        self.inner.get("unsanitized").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `unsanitized` attribute.
    pub fn set_unsanitized(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("unsanitized", value);
    }
}
