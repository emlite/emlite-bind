use super::*;

/// The Font class.
/// [`Font`](https://developer.mozilla.org/en-US/docs/Web/API/Font)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Font {
    inner: Any,
}
impl FromVal for Font {
    fn from_val(v: &Any) -> Self {
        Font {
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
impl core::ops::Deref for Font {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Font {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Font {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Font {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Font> for Any {
    fn from(s: Font) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Font> for Any {
    fn from(s: &Font) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Font);

impl Font {
    /// Getter of the `name` attribute.
    /// [`Font.name`](https://developer.mozilla.org/en-US/docs/Web/API/Font/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl Font {
    /// Getter of the `glyphsRendered` attribute.
    /// [`Font.glyphsRendered`](https://developer.mozilla.org/en-US/docs/Web/API/Font/glyphsRendered)
    pub fn glyphs_rendered(&self) -> u32 {
        self.inner.get("glyphsRendered").as_::<u32>()
    }
}
