use super::*;

/// The HTMLLinkElement class.
/// [`HTMLLinkElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLLinkElement {
    inner: HTMLElement,
}
impl FromVal for HTMLLinkElement {
    fn from_val(v: &Any) -> Self {
        HTMLLinkElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for HTMLLinkElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLLinkElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLLinkElement> for Any {
    fn from(s: HTMLLinkElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLLinkElement> for Any {
    fn from(s: &HTMLLinkElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLLinkElement);

impl HTMLLinkElement {
    /// The `new HTMLLinkElement(..)` constructor, creating a new HTMLLinkElement instance
    pub fn new() -> HTMLLinkElement {
        Self {
            inner: Any::global("HTMLLinkElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLLinkElement {
    /// Getter of the `href` attribute.
    /// [`HTMLLinkElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)
    pub fn href(&self) -> JsString {
        self.inner.get("href").as_::<JsString>()
    }

    /// Setter of the `href` attribute.
    /// [`HTMLLinkElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)
    pub fn set_href(&mut self, value: &JsString) {
        self.inner.set("href", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `crossOrigin` attribute.
    /// [`HTMLLinkElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)
    pub fn cross_origin(&self) -> JsString {
        self.inner.get("crossOrigin").as_::<JsString>()
    }

    /// Setter of the `crossOrigin` attribute.
    /// [`HTMLLinkElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)
    pub fn set_cross_origin(&mut self, value: &JsString) {
        self.inner.set("crossOrigin", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `rel` attribute.
    /// [`HTMLLinkElement.rel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)
    pub fn rel(&self) -> JsString {
        self.inner.get("rel").as_::<JsString>()
    }

    /// Setter of the `rel` attribute.
    /// [`HTMLLinkElement.rel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)
    pub fn set_rel(&mut self, value: &JsString) {
        self.inner.set("rel", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `as` attribute.
    /// [`HTMLLinkElement.as`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)
    pub fn as_(&self) -> JsString {
        self.inner.get("as").as_::<JsString>()
    }

    /// Setter of the `as` attribute.
    /// [`HTMLLinkElement.as`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)
    pub fn set_as_(&mut self, value: &JsString) {
        self.inner.set("as", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `relList` attribute.
    /// [`HTMLLinkElement.relList`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/relList)
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl HTMLLinkElement {
    /// Getter of the `media` attribute.
    /// [`HTMLLinkElement.media`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)
    pub fn media(&self) -> JsString {
        self.inner.get("media").as_::<JsString>()
    }

    /// Setter of the `media` attribute.
    /// [`HTMLLinkElement.media`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)
    pub fn set_media(&mut self, value: &JsString) {
        self.inner.set("media", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `integrity` attribute.
    /// [`HTMLLinkElement.integrity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)
    pub fn integrity(&self) -> JsString {
        self.inner.get("integrity").as_::<JsString>()
    }

    /// Setter of the `integrity` attribute.
    /// [`HTMLLinkElement.integrity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)
    pub fn set_integrity(&mut self, value: &JsString) {
        self.inner.set("integrity", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `hreflang` attribute.
    /// [`HTMLLinkElement.hreflang`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)
    pub fn hreflang(&self) -> JsString {
        self.inner.get("hreflang").as_::<JsString>()
    }

    /// Setter of the `hreflang` attribute.
    /// [`HTMLLinkElement.hreflang`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)
    pub fn set_hreflang(&mut self, value: &JsString) {
        self.inner.set("hreflang", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `type` attribute.
    /// [`HTMLLinkElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLLinkElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `sizes` attribute.
    /// [`HTMLLinkElement.sizes`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sizes)
    pub fn sizes(&self) -> DOMTokenList {
        self.inner.get("sizes").as_::<DOMTokenList>()
    }
}
impl HTMLLinkElement {
    /// Getter of the `imageSrcset` attribute.
    /// [`HTMLLinkElement.imageSrcset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/imageSrcset)
    pub fn image_srcset(&self) -> JsString {
        self.inner.get("imageSrcset").as_::<JsString>()
    }

    /// Setter of the `imageSrcset` attribute.
    /// [`HTMLLinkElement.imageSrcset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/imageSrcset)
    pub fn set_image_srcset(&mut self, value: &JsString) {
        self.inner.set("imageSrcset", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `imageSizes` attribute.
    /// [`HTMLLinkElement.imageSizes`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/imageSizes)
    pub fn image_sizes(&self) -> JsString {
        self.inner.get("imageSizes").as_::<JsString>()
    }

    /// Setter of the `imageSizes` attribute.
    /// [`HTMLLinkElement.imageSizes`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/imageSizes)
    pub fn set_image_sizes(&mut self, value: &JsString) {
        self.inner.set("imageSizes", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `referrerPolicy` attribute.
    /// [`HTMLLinkElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)
    pub fn referrer_policy(&self) -> JsString {
        self.inner.get("referrerPolicy").as_::<JsString>()
    }

    /// Setter of the `referrerPolicy` attribute.
    /// [`HTMLLinkElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)
    pub fn set_referrer_policy(&mut self, value: &JsString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `blocking` attribute.
    /// [`HTMLLinkElement.blocking`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/blocking)
    pub fn blocking(&self) -> DOMTokenList {
        self.inner.get("blocking").as_::<DOMTokenList>()
    }
}
impl HTMLLinkElement {
    /// Getter of the `disabled` attribute.
    /// [`HTMLLinkElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    /// Setter of the `disabled` attribute.
    /// [`HTMLLinkElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)
    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `fetchPriority` attribute.
    /// [`HTMLLinkElement.fetchPriority`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/fetchPriority)
    pub fn fetch_priority(&self) -> JsString {
        self.inner.get("fetchPriority").as_::<JsString>()
    }

    /// Setter of the `fetchPriority` attribute.
    /// [`HTMLLinkElement.fetchPriority`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/fetchPriority)
    pub fn set_fetch_priority(&mut self, value: &JsString) {
        self.inner.set("fetchPriority", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `charset` attribute.
    /// [`HTMLLinkElement.charset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)
    pub fn charset(&self) -> JsString {
        self.inner.get("charset").as_::<JsString>()
    }

    /// Setter of the `charset` attribute.
    /// [`HTMLLinkElement.charset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)
    pub fn set_charset(&mut self, value: &JsString) {
        self.inner.set("charset", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `rev` attribute.
    /// [`HTMLLinkElement.rev`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)
    pub fn rev(&self) -> JsString {
        self.inner.get("rev").as_::<JsString>()
    }

    /// Setter of the `rev` attribute.
    /// [`HTMLLinkElement.rev`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)
    pub fn set_rev(&mut self, value: &JsString) {
        self.inner.set("rev", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `target` attribute.
    /// [`HTMLLinkElement.target`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)
    pub fn target(&self) -> JsString {
        self.inner.get("target").as_::<JsString>()
    }

    /// Setter of the `target` attribute.
    /// [`HTMLLinkElement.target`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)
    pub fn set_target(&mut self, value: &JsString) {
        self.inner.set("target", value);
    }
}
impl HTMLLinkElement {
    /// Getter of the `sheet` attribute.
    /// [`HTMLLinkElement.sheet`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sheet)
    pub fn sheet(&self) -> CSSStyleSheet {
        self.inner.get("sheet").as_::<CSSStyleSheet>()
    }
}
