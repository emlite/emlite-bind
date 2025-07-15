use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLHtmlElement {
    inner: HTMLElement,
}
impl FromVal for HTMLHtmlElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLHtmlElement {
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
impl core::ops::Deref for HTMLHtmlElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLHtmlElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLHtmlElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLHtmlElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLHtmlElement> for emlite::Val {
    fn from(s: HTMLHtmlElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLHtmlElement);

impl HTMLHtmlElement {
    pub fn new() -> HTMLHtmlElement {
        Self {
            inner: emlite::Val::global("HTMLHtmlElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLHtmlElement {
    pub fn version(&self) -> DOMString {
        self.inner.get("version").as_::<DOMString>()
    }

    pub fn set_version(&mut self, value: DOMString) {
        self.inner.set("version", value);
    }
}
