use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLSpanElement {
    inner: HTMLElement,
}
impl FromVal for HTMLSpanElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLSpanElement {
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
impl core::ops::Deref for HTMLSpanElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLSpanElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLSpanElement> for emlite::Val {
    fn from(s: HTMLSpanElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLSpanElement {
    pub fn new() -> HTMLSpanElement {
        Self {
            inner: emlite::Val::global("HTMLSpanElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
