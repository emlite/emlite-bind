use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLDivElement {
    inner: HTMLElement,
}
impl FromVal for HTMLDivElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLDivElement {
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
impl core::ops::Deref for HTMLDivElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLDivElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLDivElement> for emlite::Val {
    fn from(s: HTMLDivElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLDivElement {
    pub fn new() -> HTMLDivElement {
        Self {
            inner: emlite::Val::global("HTMLDivElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLDivElement {
    pub fn align(&self) -> jsbind::DOMString {
        self.inner.get("align").as_::<jsbind::DOMString>()
    }

    pub fn set_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("align", value);
    }
}
