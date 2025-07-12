use super::*;

#[derive(Clone, Debug)]
pub struct HTMLTitleElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTitleElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLTitleElement {
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
impl std::ops::Deref for HTMLTitleElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLTitleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLTitleElement> for emlite::Val {
    fn from(s: HTMLTitleElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLTitleElement {
    pub fn new() -> HTMLTitleElement {
        Self {
            inner: emlite::Val::global("HTMLTitleElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTitleElement {
    pub fn text(&self) -> jsbind::DOMString {
        self.inner.get("text").as_::<jsbind::DOMString>()
    }

    pub fn set_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("text", value);
    }
}
