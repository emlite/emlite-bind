use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGStyleElement {
    inner: SVGElement,
}
impl FromVal for SVGStyleElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGStyleElement {
            inner: SVGElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGStyleElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGStyleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGStyleElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGStyleElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGStyleElement> for emlite::Val {
    fn from(s: SVGStyleElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGStyleElement);

impl SVGStyleElement {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    pub fn set_type_(&mut self, value: DOMString) {
        self.inner.set("type", value);
    }
}
impl SVGStyleElement {
    pub fn media(&self) -> DOMString {
        self.inner.get("media").as_::<DOMString>()
    }

    pub fn set_media(&mut self, value: DOMString) {
        self.inner.set("media", value);
    }
}
impl SVGStyleElement {
    pub fn title(&self) -> DOMString {
        self.inner.get("title").as_::<DOMString>()
    }

    pub fn set_title(&mut self, value: DOMString) {
        self.inner.set("title", value);
    }
}
impl SVGStyleElement {
    pub fn sheet(&self) -> CSSStyleSheet {
        self.inner.get("sheet").as_::<CSSStyleSheet>()
    }
}
