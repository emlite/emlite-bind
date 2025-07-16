use super::*;

/// The TextFormat class.
/// [`TextFormat`](https://developer.mozilla.org/en-US/docs/Web/API/TextFormat)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextFormat {
    inner: Any,
}
impl FromVal for TextFormat {
    fn from_val(v: &Any) -> Self {
        TextFormat {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextFormat {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TextFormat {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextFormat {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextFormat> for Any {
    fn from(s: TextFormat) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextFormat> for Any {
    fn from(s: &TextFormat) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextFormat);

impl TextFormat {
    /// The `new TextFormat(..)` constructor, creating a new TextFormat instance
    pub fn new0() -> TextFormat {
        Self {
            inner: Any::global("TextFormat").new(&[]).as_::<Any>(),
        }
    }

    /// The `new TextFormat(..)` constructor, creating a new TextFormat instance
    pub fn new1(options: &Any) -> TextFormat {
        Self {
            inner: Any::global("TextFormat")
                .new(&[options.into()])
                .as_::<Any>(),
        }
    }
}
impl TextFormat {
    /// Getter of the `rangeStart` attribute.
    /// [`TextFormat.rangeStart`](https://developer.mozilla.org/en-US/docs/Web/API/TextFormat/rangeStart)
    pub fn range_start(&self) -> u32 {
        self.inner.get("rangeStart").as_::<u32>()
    }
}
impl TextFormat {
    /// Getter of the `rangeEnd` attribute.
    /// [`TextFormat.rangeEnd`](https://developer.mozilla.org/en-US/docs/Web/API/TextFormat/rangeEnd)
    pub fn range_end(&self) -> u32 {
        self.inner.get("rangeEnd").as_::<u32>()
    }
}
impl TextFormat {
    /// Getter of the `underlineStyle` attribute.
    /// [`TextFormat.underlineStyle`](https://developer.mozilla.org/en-US/docs/Web/API/TextFormat/underlineStyle)
    pub fn underline_style(&self) -> UnderlineStyle {
        self.inner.get("underlineStyle").as_::<UnderlineStyle>()
    }
}
impl TextFormat {
    /// Getter of the `underlineThickness` attribute.
    /// [`TextFormat.underlineThickness`](https://developer.mozilla.org/en-US/docs/Web/API/TextFormat/underlineThickness)
    pub fn underline_thickness(&self) -> UnderlineThickness {
        self.inner
            .get("underlineThickness")
            .as_::<UnderlineThickness>()
    }
}
