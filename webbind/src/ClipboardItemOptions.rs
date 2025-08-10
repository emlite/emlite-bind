use super::*;

/// The ClipboardItemOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ClipboardItemOptions {
    inner: Any,
}

impl FromVal for ClipboardItemOptions {
    fn from_val(v: &Any) -> Self {
        ClipboardItemOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ClipboardItemOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ClipboardItemOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ClipboardItemOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ClipboardItemOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ClipboardItemOptions> for Any {
    fn from(s: ClipboardItemOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ClipboardItemOptions> for Any {
    fn from(s: &ClipboardItemOptions) -> Any {
        s.inner.clone()
    }
}

impl ClipboardItemOptions {
    /// Getter of the `presentationStyle` attribute.
    pub fn presentation_style(&self) -> PresentationStyle {
        self.inner
            .get("presentationStyle")
            .as_::<PresentationStyle>()
    }

    /// Setter of the `presentationStyle` attribute.
    pub fn set_presentation_style(&mut self, value: &PresentationStyle) {
        self.inner.set("presentationStyle", value);
    }
}
