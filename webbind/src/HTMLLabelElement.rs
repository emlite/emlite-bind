use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLLabelElement {
    inner: HTMLElement,
}
impl FromVal for HTMLLabelElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLLabelElement {
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
impl core::ops::Deref for HTMLLabelElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLLabelElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLLabelElement> for emlite::Val {
    fn from(s: HTMLLabelElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLLabelElement {
    pub fn new() -> HTMLLabelElement {
        Self {
            inner: emlite::Val::global("HTMLLabelElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLLabelElement {
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLLabelElement {
    pub fn html_for(&self) -> jsbind::DOMString {
        self.inner.get("htmlFor").as_::<jsbind::DOMString>()
    }

    pub fn set_html_for(&mut self, value: jsbind::DOMString) {
        self.inner.set("htmlFor", value);
    }
}
impl HTMLLabelElement {
    pub fn control(&self) -> HTMLElement {
        self.inner.get("control").as_::<HTMLElement>()
    }
}
