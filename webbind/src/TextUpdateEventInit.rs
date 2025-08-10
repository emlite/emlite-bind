use super::*;

/// The TextUpdateEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextUpdateEventInit {
    inner: Any,
}

impl FromVal for TextUpdateEventInit {
    fn from_val(v: &Any) -> Self {
        TextUpdateEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TextUpdateEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TextUpdateEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TextUpdateEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TextUpdateEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TextUpdateEventInit> for Any {
    fn from(s: TextUpdateEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TextUpdateEventInit> for Any {
    fn from(s: &TextUpdateEventInit) -> Any {
        s.inner.clone()
    }
}

impl TextUpdateEventInit {
    /// Getter of the `updateRangeStart` attribute.
    pub fn update_range_start(&self) -> u32 {
        self.inner.get("updateRangeStart").as_::<u32>()
    }

    /// Setter of the `updateRangeStart` attribute.
    pub fn set_update_range_start(&mut self, value: u32) {
        self.inner.set("updateRangeStart", value);
    }
}
impl TextUpdateEventInit {
    /// Getter of the `updateRangeEnd` attribute.
    pub fn update_range_end(&self) -> u32 {
        self.inner.get("updateRangeEnd").as_::<u32>()
    }

    /// Setter of the `updateRangeEnd` attribute.
    pub fn set_update_range_end(&mut self, value: u32) {
        self.inner.set("updateRangeEnd", value);
    }
}
impl TextUpdateEventInit {
    /// Getter of the `text` attribute.
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }

    /// Setter of the `text` attribute.
    pub fn set_text(&mut self, value: &JsString) {
        self.inner.set("text", value);
    }
}
impl TextUpdateEventInit {
    /// Getter of the `selectionStart` attribute.
    pub fn selection_start(&self) -> u32 {
        self.inner.get("selectionStart").as_::<u32>()
    }

    /// Setter of the `selectionStart` attribute.
    pub fn set_selection_start(&mut self, value: u32) {
        self.inner.set("selectionStart", value);
    }
}
impl TextUpdateEventInit {
    /// Getter of the `selectionEnd` attribute.
    pub fn selection_end(&self) -> u32 {
        self.inner.get("selectionEnd").as_::<u32>()
    }

    /// Setter of the `selectionEnd` attribute.
    pub fn set_selection_end(&mut self, value: u32) {
        self.inner.set("selectionEnd", value);
    }
}
impl TextUpdateEventInit {
    /// Getter of the `compositionStart` attribute.
    pub fn composition_start(&self) -> u32 {
        self.inner.get("compositionStart").as_::<u32>()
    }

    /// Setter of the `compositionStart` attribute.
    pub fn set_composition_start(&mut self, value: u32) {
        self.inner.set("compositionStart", value);
    }
}
impl TextUpdateEventInit {
    /// Getter of the `compositionEnd` attribute.
    pub fn composition_end(&self) -> u32 {
        self.inner.get("compositionEnd").as_::<u32>()
    }

    /// Setter of the `compositionEnd` attribute.
    pub fn set_composition_end(&mut self, value: u32) {
        self.inner.set("compositionEnd", value);
    }
}
