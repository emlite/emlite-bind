use super::*;

#[derive(Clone, Debug)]
pub struct HTMLFrameSetElement {
    inner: HTMLElement,
}
impl FromVal for HTMLFrameSetElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLFrameSetElement {
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
impl std::ops::Deref for HTMLFrameSetElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLFrameSetElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLFrameSetElement> for emlite::Val {
    fn from(s: HTMLFrameSetElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLFrameSetElement {
    pub fn new() -> HTMLFrameSetElement {
        Self {
            inner: emlite::Val::global("HTMLFrameSetElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLFrameSetElement {
    pub fn cols(&self) -> jsbind::DOMString {
        self.inner.get("cols").as_::<jsbind::DOMString>()
    }

    pub fn set_cols(&mut self, value: jsbind::DOMString) {
        self.inner.set("cols", value);
    }
}
impl HTMLFrameSetElement {
    pub fn rows(&self) -> jsbind::DOMString {
        self.inner.get("rows").as_::<jsbind::DOMString>()
    }

    pub fn set_rows(&mut self, value: jsbind::DOMString) {
        self.inner.set("rows", value);
    }
}
impl HTMLFrameSetElement {
    pub fn onportalactivate(&self) -> jsbind::Any {
        self.inner.get("onportalactivate").as_::<jsbind::Any>()
    }

    pub fn set_onportalactivate(&mut self, value: jsbind::Any) {
        self.inner.set("onportalactivate", value);
    }
}
