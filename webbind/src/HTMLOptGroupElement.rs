use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLOptGroupElement {
    inner: HTMLElement,
}
impl FromVal for HTMLOptGroupElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLOptGroupElement {
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
impl core::ops::Deref for HTMLOptGroupElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLOptGroupElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLOptGroupElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLOptGroupElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLOptGroupElement> for emlite::Val {
    fn from(s: HTMLOptGroupElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLOptGroupElement);

impl HTMLOptGroupElement {
    pub fn new() -> HTMLOptGroupElement {
        Self {
            inner: emlite::Val::global("HTMLOptGroupElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLOptGroupElement {
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLOptGroupElement {
    pub fn label(&self) -> jsbind::DOMString {
        self.inner.get("label").as_::<jsbind::DOMString>()
    }

    pub fn set_label(&mut self, value: jsbind::DOMString) {
        self.inner.set("label", value);
    }
}
