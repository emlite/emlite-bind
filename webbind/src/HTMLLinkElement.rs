use super::*;

#[derive(Clone, Debug)]
pub struct HTMLLinkElement {
    inner: HTMLElement,
}
impl FromVal for HTMLLinkElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLLinkElement {
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
impl std::ops::Deref for HTMLLinkElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLLinkElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLLinkElement> for emlite::Val {
    fn from(s: HTMLLinkElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLLinkElement {
    pub fn new() -> HTMLLinkElement {
        Self {
            inner: emlite::Val::global("HTMLLinkElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLLinkElement {
    pub fn href(&self) -> jsbind::USVString {
        self.inner.get("href").as_::<jsbind::USVString>()
    }

    pub fn set_href(&mut self, value: jsbind::USVString) {
        self.inner.set("href", value);
    }
}
impl HTMLLinkElement {
    pub fn cross_origin(&self) -> jsbind::DOMString {
        self.inner.get("crossOrigin").as_::<jsbind::DOMString>()
    }

    pub fn set_cross_origin(&mut self, value: jsbind::DOMString) {
        self.inner.set("crossOrigin", value);
    }
}
impl HTMLLinkElement {
    pub fn rel(&self) -> jsbind::DOMString {
        self.inner.get("rel").as_::<jsbind::DOMString>()
    }

    pub fn set_rel(&mut self, value: jsbind::DOMString) {
        self.inner.set("rel", value);
    }
}
impl HTMLLinkElement {
    pub fn as_(&self) -> jsbind::DOMString {
        self.inner.get("as").as_::<jsbind::DOMString>()
    }

    pub fn set_as_(&mut self, value: jsbind::DOMString) {
        self.inner.set("as", value);
    }
}
impl HTMLLinkElement {
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl HTMLLinkElement {
    pub fn media(&self) -> jsbind::DOMString {
        self.inner.get("media").as_::<jsbind::DOMString>()
    }

    pub fn set_media(&mut self, value: jsbind::DOMString) {
        self.inner.set("media", value);
    }
}
impl HTMLLinkElement {
    pub fn integrity(&self) -> jsbind::DOMString {
        self.inner.get("integrity").as_::<jsbind::DOMString>()
    }

    pub fn set_integrity(&mut self, value: jsbind::DOMString) {
        self.inner.set("integrity", value);
    }
}
impl HTMLLinkElement {
    pub fn hreflang(&self) -> jsbind::DOMString {
        self.inner.get("hreflang").as_::<jsbind::DOMString>()
    }

    pub fn set_hreflang(&mut self, value: jsbind::DOMString) {
        self.inner.set("hreflang", value);
    }
}
impl HTMLLinkElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLLinkElement {
    pub fn sizes(&self) -> DOMTokenList {
        self.inner.get("sizes").as_::<DOMTokenList>()
    }
}
impl HTMLLinkElement {
    pub fn image_srcset(&self) -> jsbind::USVString {
        self.inner.get("imageSrcset").as_::<jsbind::USVString>()
    }

    pub fn set_image_srcset(&mut self, value: jsbind::USVString) {
        self.inner.set("imageSrcset", value);
    }
}
impl HTMLLinkElement {
    pub fn image_sizes(&self) -> jsbind::DOMString {
        self.inner.get("imageSizes").as_::<jsbind::DOMString>()
    }

    pub fn set_image_sizes(&mut self, value: jsbind::DOMString) {
        self.inner.set("imageSizes", value);
    }
}
impl HTMLLinkElement {
    pub fn referrer_policy(&self) -> jsbind::DOMString {
        self.inner.get("referrerPolicy").as_::<jsbind::DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: jsbind::DOMString) {
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
    pub fn fetch_priority(&self) -> jsbind::DOMString {
        self.inner.get("fetchPriority").as_::<jsbind::DOMString>()
    }

    pub fn set_fetch_priority(&mut self, value: jsbind::DOMString) {
        self.inner.set("fetchPriority", value);
    }
}
impl HTMLLinkElement {
    pub fn charset(&self) -> jsbind::DOMString {
        self.inner.get("charset").as_::<jsbind::DOMString>()
    }

    pub fn set_charset(&mut self, value: jsbind::DOMString) {
        self.inner.set("charset", value);
    }
}
impl HTMLLinkElement {
    pub fn rev(&self) -> jsbind::DOMString {
        self.inner.get("rev").as_::<jsbind::DOMString>()
    }

    pub fn set_rev(&mut self, value: jsbind::DOMString) {
        self.inner.set("rev", value);
    }
}
impl HTMLLinkElement {
    pub fn target(&self) -> jsbind::DOMString {
        self.inner.get("target").as_::<jsbind::DOMString>()
    }

    pub fn set_target(&mut self, value: jsbind::DOMString) {
        self.inner.set("target", value);
    }
}
impl HTMLLinkElement {
    pub fn sheet(&self) -> CSSStyleSheet {
        self.inner.get("sheet").as_::<CSSStyleSheet>()
    }
}
