use super::*;

#[derive(Clone, Debug)]
pub struct HTMLHtmlElement {
    inner: HTMLElement,
}
impl FromVal for HTMLHtmlElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLHtmlElement {
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
impl std::ops::Deref for HTMLHtmlElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLHtmlElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLHtmlElement> for emlite::Val {
    fn from(s: HTMLHtmlElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLHtmlElement {
    pub fn new() -> HTMLHtmlElement {
        Self {
            inner: emlite::Val::global("HTMLHtmlElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLHtmlElement {
    pub fn version(&self) -> jsbind::DOMString {
        self.inner.get("version").as_::<jsbind::DOMString>()
    }

    pub fn set_version(&mut self, value: jsbind::DOMString) {
        self.inner.set("version", value);
    }
}
