use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFontElement {
    inner: HTMLElement,
}
impl FromVal for HTMLFontElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLFontElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLFontElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLFontElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLFontElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLFontElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HTMLFontElement> for emlite::Val {
    fn from(s: HTMLFontElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLFontElement);



impl HTMLFontElement {
    pub fn new() -> HTMLFontElement {
        Self {
            inner: emlite::Val::global("HTMLFontElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLFontElement {
    pub fn color(&self) -> DOMString {
        self.inner.get("color").as_::<DOMString>()
    }

    pub fn set_color(&mut self, value: DOMString) {
        self.inner.set("color", value);
    }

}
impl HTMLFontElement {
    pub fn face(&self) -> DOMString {
        self.inner.get("face").as_::<DOMString>()
    }

    pub fn set_face(&mut self, value: DOMString) {
        self.inner.set("face", value);
    }

}
impl HTMLFontElement {
    pub fn size(&self) -> DOMString {
        self.inner.get("size").as_::<DOMString>()
    }

    pub fn set_size(&mut self, value: DOMString) {
        self.inner.set("size", value);
    }

}
