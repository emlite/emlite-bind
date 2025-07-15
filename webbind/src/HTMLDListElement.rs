use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLDListElement {
    inner: HTMLElement,
}
impl FromVal for HTMLDListElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLDListElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLDListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLDListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLDListElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLDListElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HTMLDListElement> for emlite::Val {
    fn from(s: HTMLDListElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLDListElement);



impl HTMLDListElement {
    pub fn new() -> HTMLDListElement {
        Self {
            inner: emlite::Val::global("HTMLDListElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLDListElement {
    pub fn compact(&self) -> bool {
        self.inner.get("compact").as_::<bool>()
    }

    pub fn set_compact(&mut self, value: bool) {
        self.inner.set("compact", value);
    }

}
