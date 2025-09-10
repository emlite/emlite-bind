use super::*;

/// The TextEncoderEncodeIntoResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextEncoderEncodeIntoResult {
    inner: Any,
}

impl FromVal for TextEncoderEncodeIntoResult {
    fn from_val(v: &Any) -> Self {
        TextEncoderEncodeIntoResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TextEncoderEncodeIntoResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TextEncoderEncodeIntoResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TextEncoderEncodeIntoResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TextEncoderEncodeIntoResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TextEncoderEncodeIntoResult> for Any {
    fn from(s: TextEncoderEncodeIntoResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TextEncoderEncodeIntoResult> for Any {
    fn from(s: &TextEncoderEncodeIntoResult) -> Any {
        s.inner.clone()
    }
}

impl TextEncoderEncodeIntoResult {
    /// Getter of the `read` attribute.
    pub fn read(&self) -> u64 {
        self.inner.get("read").as_::<u64>()
    }

    /// Setter of the `read` attribute.
    pub fn set_read(&mut self, value: u64) {
        self.inner.set("read", value);
    }
}
impl TextEncoderEncodeIntoResult {
    /// Getter of the `written` attribute.
    pub fn written(&self) -> u64 {
        self.inner.get("written").as_::<u64>()
    }

    /// Setter of the `written` attribute.
    pub fn set_written(&mut self, value: u64) {
        self.inner.set("written", value);
    }
}
