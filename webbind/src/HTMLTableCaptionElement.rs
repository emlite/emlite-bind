use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLTableCaptionElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableCaptionElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLTableCaptionElement {
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
impl core::ops::Deref for HTMLTableCaptionElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTableCaptionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLTableCaptionElement> for emlite::Val {
    fn from(s: HTMLTableCaptionElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLTableCaptionElement {
    pub fn new() -> HTMLTableCaptionElement {
        Self {
            inner: emlite::Val::global("HTMLTableCaptionElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTableCaptionElement {
    pub fn align(&self) -> jsbind::DOMString {
        self.inner.get("align").as_::<jsbind::DOMString>()
    }

    pub fn set_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("align", value);
    }
}
