use super::*;

#[derive(Clone, Debug)]
pub struct HTMLDataElement {
    inner: HTMLElement,
}
impl FromVal for HTMLDataElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLDataElement {
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
impl std::ops::Deref for HTMLDataElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLDataElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLDataElement> for emlite::Val {
    fn from(s: HTMLDataElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLDataElement {
    pub fn new() -> HTMLDataElement {
        Self {
            inner: emlite::Val::global("HTMLDataElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLDataElement {
    pub fn value(&self) -> jsbind::DOMString {
        self.inner.get("value").as_::<jsbind::DOMString>()
    }

    pub fn set_value(&mut self, value: jsbind::DOMString) {
        self.inner.set("value", value);
    }
}
