use super::*;

/// The TextFormatInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextFormatInit {
    inner: Any,
}

impl FromVal for TextFormatInit {
    fn from_val(v: &Any) -> Self {
        TextFormatInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TextFormatInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TextFormatInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TextFormatInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TextFormatInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TextFormatInit> for Any {
    fn from(s: TextFormatInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TextFormatInit> for Any {
    fn from(s: &TextFormatInit) -> Any {
        s.inner.clone()
    }
}

impl TextFormatInit {
    /// Getter of the `rangeStart` attribute.
    pub fn range_start(&self) -> u32 {
        self.inner.get("rangeStart").as_::<u32>()
    }

    /// Setter of the `rangeStart` attribute.
    pub fn set_range_start(&mut self, value: u32) {
        self.inner.set("rangeStart", value);
    }
}
impl TextFormatInit {
    /// Getter of the `rangeEnd` attribute.
    pub fn range_end(&self) -> u32 {
        self.inner.get("rangeEnd").as_::<u32>()
    }

    /// Setter of the `rangeEnd` attribute.
    pub fn set_range_end(&mut self, value: u32) {
        self.inner.set("rangeEnd", value);
    }
}
impl TextFormatInit {
    /// Getter of the `underlineStyle` attribute.
    pub fn underline_style(&self) -> UnderlineStyle {
        self.inner.get("underlineStyle").as_::<UnderlineStyle>()
    }

    /// Setter of the `underlineStyle` attribute.
    pub fn set_underline_style(&mut self, value: &UnderlineStyle) {
        self.inner.set("underlineStyle", value);
    }
}
impl TextFormatInit {
    /// Getter of the `underlineThickness` attribute.
    pub fn underline_thickness(&self) -> UnderlineThickness {
        self.inner
            .get("underlineThickness")
            .as_::<UnderlineThickness>()
    }

    /// Setter of the `underlineThickness` attribute.
    pub fn set_underline_thickness(&mut self, value: &UnderlineThickness) {
        self.inner.set("underlineThickness", value);
    }
}
