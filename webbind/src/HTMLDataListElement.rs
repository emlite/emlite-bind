use super::*;

#[derive(Clone, Debug)]
pub struct HTMLDataListElement {
    inner: HTMLElement,
}
impl FromVal for HTMLDataListElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLDataListElement {
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
impl std::ops::Deref for HTMLDataListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLDataListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLDataListElement> for emlite::Val {
    fn from(s: HTMLDataListElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLDataListElement {
    pub fn new() -> HTMLDataListElement {
        Self {
            inner: emlite::Val::global("HTMLDataListElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLDataListElement {
    pub fn options(&self) -> HTMLCollection {
        self.inner.get("options").as_::<HTMLCollection>()
    }
}
