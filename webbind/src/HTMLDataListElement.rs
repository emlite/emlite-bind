use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for HTMLDataListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLDataListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLDataListElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLDataListElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLDataListElement> for emlite::Val {
    fn from(s: HTMLDataListElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLDataListElement);

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
