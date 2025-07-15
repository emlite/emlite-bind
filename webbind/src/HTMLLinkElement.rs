use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLLinkElement {
    inner: HTMLElement,
}
impl FromVal for HTMLLinkElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLLinkElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLLinkElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLLinkElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLLinkElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLLinkElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HTMLLinkElement> for emlite::Val {
    fn from(s: HTMLLinkElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLLinkElement);



impl HTMLLinkElement {
    pub fn new() -> HTMLLinkElement {
        Self {
            inner: emlite::Val::global("HTMLLinkElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLLinkElement {
    pub fn href(&self) -> USVString {
        self.inner.get("href").as_::<USVString>()
    }

    pub fn set_href(&mut self, value: USVString) {
        self.inner.set("href", value);
    }

}
impl HTMLLinkElement {
    pub fn cross_origin(&self) -> DOMString {
        self.inner.get("crossOrigin").as_::<DOMString>()
    }

    pub fn set_cross_origin(&mut self, value: DOMString) {
        self.inner.set("crossOrigin", value);
    }

}
impl HTMLLinkElement {
    pub fn rel(&self) -> DOMString {
        self.inner.get("rel").as_::<DOMString>()
    }

    pub fn set_rel(&mut self, value: DOMString) {
        self.inner.set("rel", value);
    }

}
impl HTMLLinkElement {
    pub fn as_(&self) -> DOMString {
        self.inner.get("as").as_::<DOMString>()
    }

    pub fn set_as_(&mut self, value: DOMString) {
        self.inner.set("as", value);
    }

}
impl HTMLLinkElement {
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }

}
impl HTMLLinkElement {
    pub fn media(&self) -> DOMString {
        self.inner.get("media").as_::<DOMString>()
    }

    pub fn set_media(&mut self, value: DOMString) {
        self.inner.set("media", value);
    }

}
impl HTMLLinkElement {
    pub fn integrity(&self) -> DOMString {
        self.inner.get("integrity").as_::<DOMString>()
    }

    pub fn set_integrity(&mut self, value: DOMString) {
        self.inner.set("integrity", value);
    }

}
impl HTMLLinkElement {
    pub fn hreflang(&self) -> DOMString {
        self.inner.get("hreflang").as_::<DOMString>()
    }

    pub fn set_hreflang(&mut self, value: DOMString) {
        self.inner.set("hreflang", value);
    }

}
impl HTMLLinkElement {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    pub fn set_type_(&mut self, value: DOMString) {
        self.inner.set("type", value);
    }

}
impl HTMLLinkElement {
    pub fn sizes(&self) -> DOMTokenList {
        self.inner.get("sizes").as_::<DOMTokenList>()
    }

}
impl HTMLLinkElement {
    pub fn image_srcset(&self) -> USVString {
        self.inner.get("imageSrcset").as_::<USVString>()
    }

    pub fn set_image_srcset(&mut self, value: USVString) {
        self.inner.set("imageSrcset", value);
    }

}
impl HTMLLinkElement {
    pub fn image_sizes(&self) -> DOMString {
        self.inner.get("imageSizes").as_::<DOMString>()
    }

    pub fn set_image_sizes(&mut self, value: DOMString) {
        self.inner.set("imageSizes", value);
    }

}
impl HTMLLinkElement {
    pub fn referrer_policy(&self) -> DOMString {
        self.inner.get("referrerPolicy").as_::<DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: DOMString) {
        self.inner.set("referrerPolicy", value);
    }

}
impl HTMLLinkElement {
    pub fn blocking(&self) -> DOMTokenList {
        self.inner.get("blocking").as_::<DOMTokenList>()
    }

}
impl HTMLLinkElement {
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }

}
impl HTMLLinkElement {
    pub fn fetch_priority(&self) -> DOMString {
        self.inner.get("fetchPriority").as_::<DOMString>()
    }

    pub fn set_fetch_priority(&mut self, value: DOMString) {
        self.inner.set("fetchPriority", value);
    }

}
impl HTMLLinkElement {
    pub fn charset(&self) -> DOMString {
        self.inner.get("charset").as_::<DOMString>()
    }

    pub fn set_charset(&mut self, value: DOMString) {
        self.inner.set("charset", value);
    }

}
impl HTMLLinkElement {
    pub fn rev(&self) -> DOMString {
        self.inner.get("rev").as_::<DOMString>()
    }

    pub fn set_rev(&mut self, value: DOMString) {
        self.inner.set("rev", value);
    }

}
impl HTMLLinkElement {
    pub fn target(&self) -> DOMString {
        self.inner.get("target").as_::<DOMString>()
    }

    pub fn set_target(&mut self, value: DOMString) {
        self.inner.set("target", value);
    }

}
impl HTMLLinkElement {
    pub fn sheet(&self) -> CSSStyleSheet {
        self.inner.get("sheet").as_::<CSSStyleSheet>()
    }

}
