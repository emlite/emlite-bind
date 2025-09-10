use super::*;

/// The TextFormatUpdateEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextFormatUpdateEventInit {
    inner: Any,
}

impl FromVal for TextFormatUpdateEventInit {
    fn from_val(v: &Any) -> Self {
        TextFormatUpdateEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TextFormatUpdateEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TextFormatUpdateEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TextFormatUpdateEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TextFormatUpdateEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TextFormatUpdateEventInit> for Any {
    fn from(s: TextFormatUpdateEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TextFormatUpdateEventInit> for Any {
    fn from(s: &TextFormatUpdateEventInit) -> Any {
        s.inner.clone()
    }
}

impl TextFormatUpdateEventInit {
    /// Getter of the `textFormats` attribute.
    pub fn text_formats(&self) -> TypedArray<TextFormat> {
        self.inner
            .get("textFormats")
            .as_::<TypedArray<TextFormat>>()
    }

    /// Setter of the `textFormats` attribute.
    pub fn set_text_formats(&mut self, value: &TypedArray<TextFormat>) {
        self.inner.set("textFormats", value);
    }
}
