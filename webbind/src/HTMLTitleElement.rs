use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for HTMLTitleElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTitleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLTitleElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLTitleElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLTitleElement> for emlite::Val {
    fn from(s: HTMLTitleElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTitleElement);

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
    pub fn text(&self) -> DOMString {
        self.inner.get("text").as_::<DOMString>()
    }

    pub fn set_text(&mut self, value: DOMString) {
        self.inner.set("text", value);
    }
}
