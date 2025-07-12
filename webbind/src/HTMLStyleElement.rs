use super::*;

#[derive(Clone, Debug)]
pub struct HTMLStyleElement {
    inner: HTMLElement,
}
impl FromVal for HTMLStyleElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLStyleElement {
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
impl std::ops::Deref for HTMLStyleElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLStyleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLStyleElement> for emlite::Val {
    fn from(s: HTMLStyleElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLStyleElement {
    pub fn new() -> HTMLStyleElement {
        Self {
            inner: emlite::Val::global("HTMLStyleElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLStyleElement {
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLStyleElement {
    pub fn media(&self) -> jsbind::DOMString {
        self.inner.get("media").as_::<jsbind::DOMString>()
    }

    pub fn set_media(&mut self, value: jsbind::DOMString) {
        self.inner.set("media", value);
    }
}
impl HTMLStyleElement {
    pub fn blocking(&self) -> DOMTokenList {
        self.inner.get("blocking").as_::<DOMTokenList>()
    }
}
impl HTMLStyleElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLStyleElement {
    pub fn sheet(&self) -> CSSStyleSheet {
        self.inner.get("sheet").as_::<CSSStyleSheet>()
    }
}
