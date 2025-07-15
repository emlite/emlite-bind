use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFrameSetElement {
    inner: HTMLElement,
}
impl FromVal for HTMLFrameSetElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLFrameSetElement {
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
impl core::ops::Deref for HTMLFrameSetElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLFrameSetElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLFrameSetElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLFrameSetElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLFrameSetElement> for emlite::Val {
    fn from(s: HTMLFrameSetElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLFrameSetElement> for emlite::Val {
    fn from(s: &HTMLFrameSetElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLFrameSetElement);

impl HTMLFrameSetElement {
    pub fn new() -> HTMLFrameSetElement {
        Self {
            inner: emlite::Val::global("HTMLFrameSetElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLFrameSetElement {
    pub fn cols(&self) -> DOMString {
        self.inner.get("cols").as_::<DOMString>()
    }

    pub fn set_cols(&mut self, value: DOMString) {
        self.inner.set("cols", value);
    }
}
impl HTMLFrameSetElement {
    pub fn rows(&self) -> DOMString {
        self.inner.get("rows").as_::<DOMString>()
    }

    pub fn set_rows(&mut self, value: DOMString) {
        self.inner.set("rows", value);
    }
}
impl HTMLFrameSetElement {
    pub fn onportalactivate(&self) -> Any {
        self.inner.get("onportalactivate").as_::<Any>()
    }

    pub fn set_onportalactivate(&mut self, value: Any) {
        self.inner.set("onportalactivate", value);
    }
}
