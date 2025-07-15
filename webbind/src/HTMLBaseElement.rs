use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLBaseElement {
    inner: HTMLElement,
}
impl FromVal for HTMLBaseElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLBaseElement {
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
impl core::ops::Deref for HTMLBaseElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLBaseElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLBaseElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLBaseElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLBaseElement> for emlite::Val {
    fn from(s: HTMLBaseElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLBaseElement);

impl HTMLBaseElement {
    pub fn new() -> HTMLBaseElement {
        Self {
            inner: emlite::Val::global("HTMLBaseElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLBaseElement {
    pub fn href(&self) -> USVString {
        self.inner.get("href").as_::<USVString>()
    }

    pub fn set_href(&mut self, value: USVString) {
        self.inner.set("href", value);
    }
}
impl HTMLBaseElement {
    pub fn target(&self) -> DOMString {
        self.inner.get("target").as_::<DOMString>()
    }

    pub fn set_target(&mut self, value: DOMString) {
        self.inner.set("target", value);
    }
}
