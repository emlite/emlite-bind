use super::*;

#[derive(Clone, Debug)]
pub struct HTMLMenuElement {
    inner: HTMLElement,
}
impl FromVal for HTMLMenuElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLMenuElement {
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
impl std::ops::Deref for HTMLMenuElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLMenuElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLMenuElement> for emlite::Val {
    fn from(s: HTMLMenuElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLMenuElement {
    pub fn new() -> HTMLMenuElement {
        Self {
            inner: emlite::Val::global("HTMLMenuElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLMenuElement {
    pub fn compact(&self) -> bool {
        self.inner.get("compact").as_::<bool>()
    }

    pub fn set_compact(&mut self, value: bool) {
        self.inner.set("compact", value);
    }
}
