use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLOListElement {
    inner: HTMLElement,
}
impl FromVal for HTMLOListElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLOListElement {
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
impl core::ops::Deref for HTMLOListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLOListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLOListElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLOListElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLOListElement> for emlite::Val {
    fn from(s: HTMLOListElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLOListElement);

impl HTMLOListElement {
    pub fn new() -> HTMLOListElement {
        Self {
            inner: emlite::Val::global("HTMLOListElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLOListElement {
    pub fn reversed(&self) -> bool {
        self.inner.get("reversed").as_::<bool>()
    }

    pub fn set_reversed(&mut self, value: bool) {
        self.inner.set("reversed", value);
    }
}
impl HTMLOListElement {
    pub fn start(&self) -> i32 {
        self.inner.get("start").as_::<i32>()
    }

    pub fn set_start(&mut self, value: i32) {
        self.inner.set("start", value);
    }
}
impl HTMLOListElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLOListElement {
    pub fn compact(&self) -> bool {
        self.inner.get("compact").as_::<bool>()
    }

    pub fn set_compact(&mut self, value: bool) {
        self.inner.set("compact", value);
    }
}
