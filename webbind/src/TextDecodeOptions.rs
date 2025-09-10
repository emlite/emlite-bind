use super::*;

/// The TextDecodeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextDecodeOptions {
    inner: Any,
}

impl FromVal for TextDecodeOptions {
    fn from_val(v: &Any) -> Self {
        TextDecodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TextDecodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TextDecodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TextDecodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TextDecodeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TextDecodeOptions> for Any {
    fn from(s: TextDecodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TextDecodeOptions> for Any {
    fn from(s: &TextDecodeOptions) -> Any {
        s.inner.clone()
    }
}

impl TextDecodeOptions {
    /// Getter of the `stream` attribute.
    pub fn stream(&self) -> bool {
        self.inner.get("stream").as_::<bool>()
    }

    /// Setter of the `stream` attribute.
    pub fn set_stream(&mut self, value: bool) {
        self.inner.set("stream", value);
    }
}
