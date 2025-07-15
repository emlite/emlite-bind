use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for HTMLIFrameElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLIFrameElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLIFrameElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLIFrameElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLIFrameElement> for emlite::Val {
    fn from(s: HTMLIFrameElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLIFrameElement);

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
    pub fn src(&self) -> USVString {
        self.inner.get("src").as_::<USVString>()
    }

    pub fn set_src(&mut self, value: USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLIFrameElement {
    pub fn srcdoc(&self) -> Any {
        self.inner.get("srcdoc").as_::<Any>()
    }

    pub fn set_srcdoc(&mut self, value: Any) {
        self.inner.set("srcdoc", value);
    }
}
impl HTMLIFrameElement {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    pub fn set_name(&mut self, value: DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLIFrameElement {
    pub fn sandbox(&self) -> DOMTokenList {
        self.inner.get("sandbox").as_::<DOMTokenList>()
    }
}
impl HTMLIFrameElement {
    pub fn allow(&self) -> DOMString {
        self.inner.get("allow").as_::<DOMString>()
    }

    pub fn set_allow(&mut self, value: DOMString) {
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
    pub fn width(&self) -> DOMString {
        self.inner.get("width").as_::<DOMString>()
    }

    pub fn set_width(&mut self, value: DOMString) {
        self.inner.set("width", value);
    }
}
impl HTMLIFrameElement {
    pub fn height(&self) -> DOMString {
        self.inner.get("height").as_::<DOMString>()
    }

    pub fn set_height(&mut self, value: DOMString) {
        self.inner.set("height", value);
    }
}
impl HTMLIFrameElement {
    pub fn referrer_policy(&self) -> DOMString {
        self.inner.get("referrerPolicy").as_::<DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: DOMString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLIFrameElement {
    pub fn loading(&self) -> DOMString {
        self.inner.get("loading").as_::<DOMString>()
    }

    pub fn set_loading(&mut self, value: DOMString) {
        self.inner.set("loading", value);
    }
}
impl HTMLIFrameElement {
    pub fn content_document(&self) -> Document {
        self.inner.get("contentDocument").as_::<Document>()
    }
}
impl HTMLIFrameElement {
    pub fn content_window(&self) -> Any {
        self.inner.get("contentWindow").as_::<Any>()
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
    pub fn csp(&self) -> DOMString {
        self.inner.get("csp").as_::<DOMString>()
    }

    pub fn set_csp(&mut self, value: DOMString) {
        self.inner.set("csp", value);
    }
}
impl HTMLIFrameElement {
    pub fn align(&self) -> DOMString {
        self.inner.get("align").as_::<DOMString>()
    }

    pub fn set_align(&mut self, value: DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLIFrameElement {
    pub fn scrolling(&self) -> DOMString {
        self.inner.get("scrolling").as_::<DOMString>()
    }

    pub fn set_scrolling(&mut self, value: DOMString) {
        self.inner.set("scrolling", value);
    }
}
impl HTMLIFrameElement {
    pub fn frame_border(&self) -> DOMString {
        self.inner.get("frameBorder").as_::<DOMString>()
    }

    pub fn set_frame_border(&mut self, value: DOMString) {
        self.inner.set("frameBorder", value);
    }
}
impl HTMLIFrameElement {
    pub fn long_desc(&self) -> USVString {
        self.inner.get("longDesc").as_::<USVString>()
    }

    pub fn set_long_desc(&mut self, value: USVString) {
        self.inner.set("longDesc", value);
    }
}
impl HTMLIFrameElement {
    pub fn margin_height(&self) -> DOMString {
        self.inner.get("marginHeight").as_::<DOMString>()
    }

    pub fn set_margin_height(&mut self, value: DOMString) {
        self.inner.set("marginHeight", value);
    }
}
impl HTMLIFrameElement {
    pub fn margin_width(&self) -> DOMString {
        self.inner.get("marginWidth").as_::<DOMString>()
    }

    pub fn set_margin_width(&mut self, value: DOMString) {
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
    pub fn private_token(&self) -> DOMString {
        self.inner.get("privateToken").as_::<DOMString>()
    }

    pub fn set_private_token(&mut self, value: DOMString) {
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
