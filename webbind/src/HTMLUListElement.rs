use super::*;

#[derive(Clone, Debug)]
pub struct HTMLUListElement {
    inner: HTMLElement,
}
impl FromVal for HTMLUListElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLUListElement {
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
impl std::ops::Deref for HTMLUListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLUListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLUListElement> for emlite::Val {
    fn from(s: HTMLUListElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLUListElement {
    pub fn new() -> HTMLUListElement {
        Self {
            inner: emlite::Val::global("HTMLUListElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLUListElement {
    pub fn compact(&self) -> bool {
        self.inner.get("compact").as_::<bool>()
    }

    pub fn set_compact(&mut self, value: bool) {
        self.inner.set("compact", value);
    }
}
impl HTMLUListElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
