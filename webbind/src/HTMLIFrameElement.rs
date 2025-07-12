use super::*;

#[derive(Clone, Debug)]
pub struct HTMLIFrameElement {
    inner: HTMLElement,
}
impl FromVal for HTMLIFrameElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLIFrameElement {
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
impl std::ops::Deref for HTMLIFrameElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLIFrameElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLIFrameElement> for emlite::Val {
    fn from(s: HTMLIFrameElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLIFrameElement {
    pub fn new() -> HTMLIFrameElement {
        Self {
            inner: emlite::Val::global("HTMLIFrameElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLIFrameElement {
    pub fn src(&self) -> jsbind::USVString {
        self.inner.get("src").as_::<jsbind::USVString>()
    }

    pub fn set_src(&mut self, value: jsbind::USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLIFrameElement {
    pub fn srcdoc(&self) -> jsbind::Any {
        self.inner.get("srcdoc").as_::<jsbind::Any>()
    }

    pub fn set_srcdoc(&mut self, value: jsbind::Any) {
        self.inner.set("srcdoc", value);
    }
}
impl HTMLIFrameElement {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLIFrameElement {
    pub fn sandbox(&self) -> DOMTokenList {
        self.inner.get("sandbox").as_::<DOMTokenList>()
    }
}
impl HTMLIFrameElement {
    pub fn allow(&self) -> jsbind::DOMString {
        self.inner.get("allow").as_::<jsbind::DOMString>()
    }

    pub fn set_allow(&mut self, value: jsbind::DOMString) {
        self.inner.set("allow", value);
    }
}
impl HTMLIFrameElement {
    pub fn allow_fullscreen(&self) -> bool {
        self.inner.get("allowFullscreen").as_::<bool>()
    }

    pub fn set_allow_fullscreen(&mut self, value: bool) {
        self.inner.set("allowFullscreen", value);
    }
}
impl HTMLIFrameElement {
    pub fn width(&self) -> jsbind::DOMString {
        self.inner.get("width").as_::<jsbind::DOMString>()
    }

    pub fn set_width(&mut self, value: jsbind::DOMString) {
        self.inner.set("width", value);
    }
}
impl HTMLIFrameElement {
    pub fn height(&self) -> jsbind::DOMString {
        self.inner.get("height").as_::<jsbind::DOMString>()
    }

    pub fn set_height(&mut self, value: jsbind::DOMString) {
        self.inner.set("height", value);
    }
}
impl HTMLIFrameElement {
    pub fn referrer_policy(&self) -> jsbind::DOMString {
        self.inner.get("referrerPolicy").as_::<jsbind::DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: jsbind::DOMString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLIFrameElement {
    pub fn loading(&self) -> jsbind::DOMString {
        self.inner.get("loading").as_::<jsbind::DOMString>()
    }

    pub fn set_loading(&mut self, value: jsbind::DOMString) {
        self.inner.set("loading", value);
    }
}
impl HTMLIFrameElement {
    pub fn content_document(&self) -> Document {
        self.inner.get("contentDocument").as_::<Document>()
    }
}
impl HTMLIFrameElement {
    pub fn content_window(&self) -> jsbind::Any {
        self.inner.get("contentWindow").as_::<jsbind::Any>()
    }
}
impl HTMLIFrameElement {
    pub fn get_svg_document(&self) -> Document {
        self.inner.call("getSVGDocument", &[]).as_::<Document>()
    }
}
impl HTMLIFrameElement {
    pub fn credentialless(&self) -> bool {
        self.inner.get("credentialless").as_::<bool>()
    }

    pub fn set_credentialless(&mut self, value: bool) {
        self.inner.set("credentialless", value);
    }
}
impl HTMLIFrameElement {
    pub fn csp(&self) -> jsbind::DOMString {
        self.inner.get("csp").as_::<jsbind::DOMString>()
    }

    pub fn set_csp(&mut self, value: jsbind::DOMString) {
        self.inner.set("csp", value);
    }
}
impl HTMLIFrameElement {
    pub fn align(&self) -> jsbind::DOMString {
        self.inner.get("align").as_::<jsbind::DOMString>()
    }

    pub fn set_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLIFrameElement {
    pub fn scrolling(&self) -> jsbind::DOMString {
        self.inner.get("scrolling").as_::<jsbind::DOMString>()
    }

    pub fn set_scrolling(&mut self, value: jsbind::DOMString) {
        self.inner.set("scrolling", value);
    }
}
impl HTMLIFrameElement {
    pub fn frame_border(&self) -> jsbind::DOMString {
        self.inner.get("frameBorder").as_::<jsbind::DOMString>()
    }

    pub fn set_frame_border(&mut self, value: jsbind::DOMString) {
        self.inner.set("frameBorder", value);
    }
}
impl HTMLIFrameElement {
    pub fn long_desc(&self) -> jsbind::USVString {
        self.inner.get("longDesc").as_::<jsbind::USVString>()
    }

    pub fn set_long_desc(&mut self, value: jsbind::USVString) {
        self.inner.set("longDesc", value);
    }
}
impl HTMLIFrameElement {
    pub fn margin_height(&self) -> jsbind::DOMString {
        self.inner.get("marginHeight").as_::<jsbind::DOMString>()
    }

    pub fn set_margin_height(&mut self, value: jsbind::DOMString) {
        self.inner.set("marginHeight", value);
    }
}
impl HTMLIFrameElement {
    pub fn margin_width(&self) -> jsbind::DOMString {
        self.inner.get("marginWidth").as_::<jsbind::DOMString>()
    }

    pub fn set_margin_width(&mut self, value: jsbind::DOMString) {
        self.inner.set("marginWidth", value);
    }
}
impl HTMLIFrameElement {
    pub fn permissions_policy(&self) -> PermissionsPolicy {
        self.inner
            .get("permissionsPolicy")
            .as_::<PermissionsPolicy>()
    }
}
impl HTMLIFrameElement {
    pub fn private_token(&self) -> jsbind::DOMString {
        self.inner.get("privateToken").as_::<jsbind::DOMString>()
    }

    pub fn set_private_token(&mut self, value: jsbind::DOMString) {
        self.inner.set("privateToken", value);
    }
}
impl HTMLIFrameElement {
    pub fn ad_auction_headers(&self) -> bool {
        self.inner.get("adAuctionHeaders").as_::<bool>()
    }

    pub fn set_ad_auction_headers(&mut self, value: bool) {
        self.inner.set("adAuctionHeaders", value);
    }
}
impl HTMLIFrameElement {
    pub fn shared_storage_writable(&self) -> bool {
        self.inner.get("sharedStorageWritable").as_::<bool>()
    }

    pub fn set_shared_storage_writable(&mut self, value: bool) {
        self.inner.set("sharedStorageWritable", value);
    }
}
