use super::*;




/// The FontFacePalettes class.
/// [`FontFacePalettes`](https://developer.mozilla.org/en-US/docs/Web/API/FontFacePalettes)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFacePalettes {
    inner: Any,
}

impl FromVal for FontFacePalettes {
    fn from_val(v: &Any) -> Self {
        FontFacePalettes { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FontFacePalettes {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FontFacePalettes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FontFacePalettes {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FontFacePalettes {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<FontFacePalettes> for Any {
    fn from(s: FontFacePalettes) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FontFacePalettes> for Any {
    fn from(s: &FontFacePalettes) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FontFacePalettes);


impl FontFacePalettes {
    /// Getter of the `length` attribute.
    /// [`FontFacePalettes.length`](https://developer.mozilla.org/en-US/docs/Web/API/FontFacePalettes/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
