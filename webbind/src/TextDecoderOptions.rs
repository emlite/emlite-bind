use super::*;

/// The TextDecoderOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextDecoderOptions {
    inner: Any,
}

impl FromVal for TextDecoderOptions {
    fn from_val(v: &Any) -> Self {
        TextDecoderOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TextDecoderOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TextDecoderOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TextDecoderOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TextDecoderOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TextDecoderOptions> for Any {
    fn from(s: TextDecoderOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TextDecoderOptions> for Any {
    fn from(s: &TextDecoderOptions) -> Any {
        s.inner.clone()
    }
}

impl TextDecoderOptions {
    /// Getter of the `fatal` attribute.
    pub fn fatal(&self) -> bool {
        self.inner.get("fatal").as_::<bool>()
    }

    /// Setter of the `fatal` attribute.
    pub fn set_fatal(&mut self, value: bool) {
        self.inner.set("fatal", value);
    }
}
impl TextDecoderOptions {
    /// Getter of the `ignoreBOM` attribute.
    pub fn ignore_bom(&self) -> bool {
        self.inner.get("ignoreBOM").as_::<bool>()
    }

    /// Setter of the `ignoreBOM` attribute.
    pub fn set_ignore_bom(&mut self, value: bool) {
        self.inner.set("ignoreBOM", value);
    }
}
