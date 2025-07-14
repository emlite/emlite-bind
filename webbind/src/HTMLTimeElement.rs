use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTimeElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTimeElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLTimeElement {
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
impl core::ops::Deref for HTMLTimeElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTimeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLTimeElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLTimeElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLTimeElement> for emlite::Val {
    fn from(s: HTMLTimeElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTimeElement);

impl HTMLTimeElement {
    pub fn new() -> HTMLTimeElement {
        Self {
            inner: emlite::Val::global("HTMLTimeElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTimeElement {
    pub fn date_time(&self) -> jsbind::DOMString {
        self.inner.get("dateTime").as_::<jsbind::DOMString>()
    }

    pub fn set_date_time(&mut self, value: jsbind::DOMString) {
        self.inner.set("dateTime", value);
    }
}
