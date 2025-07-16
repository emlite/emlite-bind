use super::*;

/// The FontFacePalette class.
/// [`FontFacePalette`](https://developer.mozilla.org/en-US/docs/Web/API/FontFacePalette)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFacePalette {
    inner: Any,
}
impl FromVal for FontFacePalette {
    fn from_val(v: &Any) -> Self {
        FontFacePalette {
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
impl core::ops::Deref for FontFacePalette {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontFacePalette {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FontFacePalette {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FontFacePalette {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FontFacePalette> for Any {
    fn from(s: FontFacePalette) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FontFacePalette> for Any {
    fn from(s: &FontFacePalette) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FontFacePalette);

impl FontFacePalette {
    /// Getter of the `length` attribute.
    /// [`FontFacePalette.length`](https://developer.mozilla.org/en-US/docs/Web/API/FontFacePalette/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl FontFacePalette {
    /// Getter of the `usableWithLightBackground` attribute.
    /// [`FontFacePalette.usableWithLightBackground`](https://developer.mozilla.org/en-US/docs/Web/API/FontFacePalette/usableWithLightBackground)
    pub fn usable_with_light_background(&self) -> bool {
        self.inner.get("usableWithLightBackground").as_::<bool>()
    }
}
impl FontFacePalette {
    /// Getter of the `usableWithDarkBackground` attribute.
    /// [`FontFacePalette.usableWithDarkBackground`](https://developer.mozilla.org/en-US/docs/Web/API/FontFacePalette/usableWithDarkBackground)
    pub fn usable_with_dark_background(&self) -> bool {
        self.inner.get("usableWithDarkBackground").as_::<bool>()
    }
}
