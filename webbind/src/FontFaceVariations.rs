use super::*;

/// The FontFaceVariations class.
/// [`FontFaceVariations`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceVariations)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFaceVariations {
    inner: Any,
}

impl FromVal for FontFaceVariations {
    fn from_val(v: &Any) -> Self {
        FontFaceVariations {
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

impl core::ops::Deref for FontFaceVariations {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FontFaceVariations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FontFaceVariations {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FontFaceVariations {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FontFaceVariations> for Any {
    fn from(s: FontFaceVariations) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FontFaceVariations> for Any {
    fn from(s: &FontFaceVariations) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FontFaceVariations);
