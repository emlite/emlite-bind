use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLPictureElement {
    inner: HTMLElement,
}
impl FromVal for HTMLPictureElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLPictureElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLPictureElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLPictureElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLPictureElement> for emlite::Val {
    fn from(s: HTMLPictureElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLPictureElement {
    pub fn new() -> HTMLPictureElement {
        Self {
            inner: emlite::Val::global("HTMLPictureElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
