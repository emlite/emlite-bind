use super::*;




/// The HTMLIFrameElement class.
/// [`HTMLIFrameElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLIFrameElement {
    inner: HTMLElement,
}

impl FromVal for HTMLIFrameElement {
    fn from_val(v: &Any) -> Self {
        HTMLIFrameElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for HTMLIFrameElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLIFrameElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLIFrameElement> for Any {
    fn from(s: HTMLIFrameElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLIFrameElement> for Any {
    fn from(s: &HTMLIFrameElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLIFrameElement);



impl HTMLIFrameElement {
    /// The `new HTMLIFrameElement(..)` constructor, creating a new HTMLIFrameElement instance
    pub fn new() -> HTMLIFrameElement {
        Self {
            inner: Any::global("HTMLIFrameElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLIFrameElement {
    /// Getter of the `src` attribute.
    /// [`HTMLIFrameElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

    /// Setter of the `src` attribute.
    /// [`HTMLIFrameElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)
    pub fn set_src(&mut self, value: &JsString) {
        self.inner.set("src", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `srcdoc` attribute.
    /// [`HTMLIFrameElement.srcdoc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)
    pub fn srcdoc(&self) -> Any {
        self.inner.get("srcdoc").as_::<Any>()
    }

    /// Setter of the `srcdoc` attribute.
    /// [`HTMLIFrameElement.srcdoc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)
    pub fn set_srcdoc(&mut self, value: &Any) {
        self.inner.set("srcdoc", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `name` attribute.
    /// [`HTMLIFrameElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLIFrameElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `sandbox` attribute.
    /// [`HTMLIFrameElement.sandbox`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/sandbox)
    pub fn sandbox(&self) -> DOMTokenList {
        self.inner.get("sandbox").as_::<DOMTokenList>()
    }

}
impl HTMLIFrameElement {
    /// Getter of the `allow` attribute.
    /// [`HTMLIFrameElement.allow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allow)
    pub fn allow(&self) -> JsString {
        self.inner.get("allow").as_::<JsString>()
    }

    /// Setter of the `allow` attribute.
    /// [`HTMLIFrameElement.allow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allow)
    pub fn set_allow(&mut self, value: &JsString) {
        self.inner.set("allow", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `allowFullscreen` attribute.
    /// [`HTMLIFrameElement.allowFullscreen`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)
    pub fn allow_fullscreen(&self) -> bool {
        self.inner.get("allowFullscreen").as_::<bool>()
    }

    /// Setter of the `allowFullscreen` attribute.
    /// [`HTMLIFrameElement.allowFullscreen`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)
    pub fn set_allow_fullscreen(&mut self, value: bool) {
        self.inner.set("allowFullscreen", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `width` attribute.
    /// [`HTMLIFrameElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)
    pub fn width(&self) -> JsString {
        self.inner.get("width").as_::<JsString>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLIFrameElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)
    pub fn set_width(&mut self, value: &JsString) {
        self.inner.set("width", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `height` attribute.
    /// [`HTMLIFrameElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)
    pub fn height(&self) -> JsString {
        self.inner.get("height").as_::<JsString>()
    }

    /// Setter of the `height` attribute.
    /// [`HTMLIFrameElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)
    pub fn set_height(&mut self, value: &JsString) {
        self.inner.set("height", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `referrerPolicy` attribute.
    /// [`HTMLIFrameElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)
    pub fn referrer_policy(&self) -> JsString {
        self.inner.get("referrerPolicy").as_::<JsString>()
    }

    /// Setter of the `referrerPolicy` attribute.
    /// [`HTMLIFrameElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)
    pub fn set_referrer_policy(&mut self, value: &JsString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `loading` attribute.
    /// [`HTMLIFrameElement.loading`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/loading)
    pub fn loading(&self) -> JsString {
        self.inner.get("loading").as_::<JsString>()
    }

    /// Setter of the `loading` attribute.
    /// [`HTMLIFrameElement.loading`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/loading)
    pub fn set_loading(&mut self, value: &JsString) {
        self.inner.set("loading", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `contentDocument` attribute.
    /// [`HTMLIFrameElement.contentDocument`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentDocument)
    pub fn content_document(&self) -> Document {
        self.inner.get("contentDocument").as_::<Document>()
    }

}
impl HTMLIFrameElement {
    /// Getter of the `contentWindow` attribute.
    /// [`HTMLIFrameElement.contentWindow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentWindow)
    pub fn content_window(&self) -> Any {
        self.inner.get("contentWindow").as_::<Any>()
    }

}
impl HTMLIFrameElement {
    /// The getSVGDocument method.
    /// [`HTMLIFrameElement.getSVGDocument`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/getSVGDocument)
    pub fn get_svg_document(&self, ) -> Document {
        self.inner.call("getSVGDocument", &[]).as_::<Document>()
    }
}
impl HTMLIFrameElement {
    /// Getter of the `credentialless` attribute.
    /// [`HTMLIFrameElement.credentialless`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/credentialless)
    pub fn credentialless(&self) -> bool {
        self.inner.get("credentialless").as_::<bool>()
    }

    /// Setter of the `credentialless` attribute.
    /// [`HTMLIFrameElement.credentialless`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/credentialless)
    pub fn set_credentialless(&mut self, value: bool) {
        self.inner.set("credentialless", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `csp` attribute.
    /// [`HTMLIFrameElement.csp`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/csp)
    pub fn csp(&self) -> JsString {
        self.inner.get("csp").as_::<JsString>()
    }

    /// Setter of the `csp` attribute.
    /// [`HTMLIFrameElement.csp`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/csp)
    pub fn set_csp(&mut self, value: &JsString) {
        self.inner.set("csp", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `align` attribute.
    /// [`HTMLIFrameElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)
    pub fn align(&self) -> JsString {
        self.inner.get("align").as_::<JsString>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLIFrameElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)
    pub fn set_align(&mut self, value: &JsString) {
        self.inner.set("align", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `scrolling` attribute.
    /// [`HTMLIFrameElement.scrolling`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)
    pub fn scrolling(&self) -> JsString {
        self.inner.get("scrolling").as_::<JsString>()
    }

    /// Setter of the `scrolling` attribute.
    /// [`HTMLIFrameElement.scrolling`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)
    pub fn set_scrolling(&mut self, value: &JsString) {
        self.inner.set("scrolling", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `frameBorder` attribute.
    /// [`HTMLIFrameElement.frameBorder`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)
    pub fn frame_border(&self) -> JsString {
        self.inner.get("frameBorder").as_::<JsString>()
    }

    /// Setter of the `frameBorder` attribute.
    /// [`HTMLIFrameElement.frameBorder`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)
    pub fn set_frame_border(&mut self, value: &JsString) {
        self.inner.set("frameBorder", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `longDesc` attribute.
    /// [`HTMLIFrameElement.longDesc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)
    pub fn long_desc(&self) -> JsString {
        self.inner.get("longDesc").as_::<JsString>()
    }

    /// Setter of the `longDesc` attribute.
    /// [`HTMLIFrameElement.longDesc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)
    pub fn set_long_desc(&mut self, value: &JsString) {
        self.inner.set("longDesc", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `marginHeight` attribute.
    /// [`HTMLIFrameElement.marginHeight`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)
    pub fn margin_height(&self) -> JsString {
        self.inner.get("marginHeight").as_::<JsString>()
    }

    /// Setter of the `marginHeight` attribute.
    /// [`HTMLIFrameElement.marginHeight`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)
    pub fn set_margin_height(&mut self, value: &JsString) {
        self.inner.set("marginHeight", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `marginWidth` attribute.
    /// [`HTMLIFrameElement.marginWidth`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)
    pub fn margin_width(&self) -> JsString {
        self.inner.get("marginWidth").as_::<JsString>()
    }

    /// Setter of the `marginWidth` attribute.
    /// [`HTMLIFrameElement.marginWidth`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)
    pub fn set_margin_width(&mut self, value: &JsString) {
        self.inner.set("marginWidth", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `permissionsPolicy` attribute.
    /// [`HTMLIFrameElement.permissionsPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/permissionsPolicy)
    pub fn permissions_policy(&self) -> PermissionsPolicy {
        self.inner.get("permissionsPolicy").as_::<PermissionsPolicy>()
    }

}
impl HTMLIFrameElement {
    /// Getter of the `privateToken` attribute.
    /// [`HTMLIFrameElement.privateToken`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/privateToken)
    pub fn private_token(&self) -> JsString {
        self.inner.get("privateToken").as_::<JsString>()
    }

    /// Setter of the `privateToken` attribute.
    /// [`HTMLIFrameElement.privateToken`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/privateToken)
    pub fn set_private_token(&mut self, value: &JsString) {
        self.inner.set("privateToken", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `adAuctionHeaders` attribute.
    /// [`HTMLIFrameElement.adAuctionHeaders`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/adAuctionHeaders)
    pub fn ad_auction_headers(&self) -> bool {
        self.inner.get("adAuctionHeaders").as_::<bool>()
    }

    /// Setter of the `adAuctionHeaders` attribute.
    /// [`HTMLIFrameElement.adAuctionHeaders`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/adAuctionHeaders)
    pub fn set_ad_auction_headers(&mut self, value: bool) {
        self.inner.set("adAuctionHeaders", value);
    }
}
impl HTMLIFrameElement {
    /// Getter of the `sharedStorageWritable` attribute.
    /// [`HTMLIFrameElement.sharedStorageWritable`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/sharedStorageWritable)
    pub fn shared_storage_writable(&self) -> bool {
        self.inner.get("sharedStorageWritable").as_::<bool>()
    }

    /// Setter of the `sharedStorageWritable` attribute.
    /// [`HTMLIFrameElement.sharedStorageWritable`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/sharedStorageWritable)
    pub fn set_shared_storage_writable(&mut self, value: bool) {
        self.inner.set("sharedStorageWritable", value);
    }
}
