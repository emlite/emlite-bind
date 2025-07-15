use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextFormat {
    inner: emlite::Val,
}
impl FromVal for TextFormat {
    fn from_val(v: &emlite::Val) -> Self {
        TextFormat { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TextFormat {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TextFormat {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<TextFormat> for emlite::Val {
    fn from(s: TextFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TextFormat);



impl TextFormat {
    pub fn new0() -> TextFormat {
        Self {
            inner: emlite::Val::global("TextFormat").new(&[]).as_::<emlite::Val>(),
        }
    }

    pub fn new1(options: Any) -> TextFormat {
        Self {
            inner: emlite::Val::global("TextFormat").new(&[options.into()]).as_::<emlite::Val>(),
        }
    }

}
impl TextFormat {
    pub fn range_start(&self) -> u32 {
        self.inner.get("rangeStart").as_::<u32>()
    }

}
impl TextFormat {
    pub fn range_end(&self) -> u32 {
        self.inner.get("rangeEnd").as_::<u32>()
    }

}
impl TextFormat {
    pub fn underline_style(&self) -> UnderlineStyle {
        self.inner.get("underlineStyle").as_::<UnderlineStyle>()
    }

}
impl TextFormat {
    pub fn underline_thickness(&self) -> UnderlineThickness {
        self.inner.get("underlineThickness").as_::<UnderlineThickness>()
    }

}
