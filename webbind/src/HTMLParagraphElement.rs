use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLParagraphElement {
    inner: HTMLElement,
}
impl FromVal for HTMLParagraphElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLParagraphElement {
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
impl core::ops::Deref for HTMLParagraphElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLParagraphElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLParagraphElement> for emlite::Val {
    fn from(s: HTMLParagraphElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLParagraphElement {
    pub fn new() -> HTMLParagraphElement {
        Self {
            inner: emlite::Val::global("HTMLParagraphElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLParagraphElement {
    pub fn align(&self) -> jsbind::DOMString {
        self.inner.get("align").as_::<jsbind::DOMString>()
    }

    pub fn set_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("align", value);
    }
}
