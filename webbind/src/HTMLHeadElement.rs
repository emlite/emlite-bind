use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLHeadElement {
    inner: HTMLElement,
}
impl FromVal for HTMLHeadElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLHeadElement {
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
impl core::ops::Deref for HTMLHeadElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLHeadElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLHeadElement> for emlite::Val {
    fn from(s: HTMLHeadElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLHeadElement {
    pub fn new() -> HTMLHeadElement {
        Self {
            inner: emlite::Val::global("HTMLHeadElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
