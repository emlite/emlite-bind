use super::*;

#[derive(Clone, Debug)]
pub struct HTMLDirectoryElement {
    inner: HTMLElement,
}
impl FromVal for HTMLDirectoryElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLDirectoryElement {
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
impl std::ops::Deref for HTMLDirectoryElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLDirectoryElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLDirectoryElement> for emlite::Val {
    fn from(s: HTMLDirectoryElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLDirectoryElement {
    pub fn new() -> HTMLDirectoryElement {
        Self {
            inner: emlite::Val::global("HTMLDirectoryElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLDirectoryElement {
    pub fn compact(&self) -> bool {
        self.inner.get("compact").as_::<bool>()
    }

    pub fn set_compact(&mut self, value: bool) {
        self.inner.set("compact", value);
    }
}
