use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFacePalette {
    inner: emlite::Val,
}
impl FromVal for FontFacePalette {
    fn from_val(v: &emlite::Val) -> Self {
        FontFacePalette { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FontFacePalette {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontFacePalette {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FontFacePalette {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FontFacePalette {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<FontFacePalette> for emlite::Val {
    fn from(s: FontFacePalette) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FontFacePalette);


impl FontFacePalette {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl FontFacePalette {
    pub fn usable_with_light_background(&self) -> bool {
        self.inner.get("usableWithLightBackground").as_::<bool>()
    }

}
impl FontFacePalette {
    pub fn usable_with_dark_background(&self) -> bool {
        self.inner.get("usableWithDarkBackground").as_::<bool>()
    }

}
