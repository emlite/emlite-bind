use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLQuoteElement {
    inner: HTMLElement,
}
impl FromVal for HTMLQuoteElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLQuoteElement {
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
impl core::ops::Deref for HTMLQuoteElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLQuoteElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLQuoteElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLQuoteElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLQuoteElement> for emlite::Val {
    fn from(s: HTMLQuoteElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLQuoteElement);

impl HTMLQuoteElement {
    pub fn new() -> HTMLQuoteElement {
        Self {
            inner: emlite::Val::global("HTMLQuoteElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLQuoteElement {
    pub fn cite(&self) -> jsbind::USVString {
        self.inner.get("cite").as_::<jsbind::USVString>()
    }

    pub fn set_cite(&mut self, value: jsbind::USVString) {
        self.inner.set("cite", value);
    }
}
