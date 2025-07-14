use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLHeadingElement {
    inner: HTMLElement,
}
impl FromVal for HTMLHeadingElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLHeadingElement {
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
impl core::ops::Deref for HTMLHeadingElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLHeadingElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLHeadingElement> for emlite::Val {
    fn from(s: HTMLHeadingElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLHeadingElement {
    pub fn new() -> HTMLHeadingElement {
        Self {
            inner: emlite::Val::global("HTMLHeadingElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLHeadingElement {
    pub fn align(&self) -> jsbind::DOMString {
        self.inner.get("align").as_::<jsbind::DOMString>()
    }

    pub fn set_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("align", value);
    }
}
