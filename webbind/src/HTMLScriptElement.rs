use super::*;

/// The HTMLScriptElement class.
/// [`HTMLScriptElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLScriptElement {
    inner: HTMLElement,
}

impl FromVal for HTMLScriptElement {
    fn from_val(v: &Any) -> Self {
        HTMLScriptElement {
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

impl core::ops::Deref for HTMLScriptElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLScriptElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLScriptElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLScriptElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLScriptElement> for Any {
    fn from(s: HTMLScriptElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLScriptElement> for Any {
    fn from(s: &HTMLScriptElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLScriptElement);

impl HTMLScriptElement {
    /// The `new HTMLScriptElement(..)` constructor, creating a new HTMLScriptElement instance
    pub fn new() -> HTMLScriptElement {
        Self {
            inner: Any::global("HTMLScriptElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLScriptElement {
    /// Getter of the `type` attribute.
    /// [`HTMLScriptElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLScriptElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/type)
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `src` attribute.
    /// [`HTMLScriptElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/src)
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

    /// Setter of the `src` attribute.
    /// [`HTMLScriptElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/src)
    pub fn set_src(&mut self, value: &JsString) {
        self.inner.set("src", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `noModule` attribute.
    /// [`HTMLScriptElement.noModule`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/noModule)
    pub fn no_module(&self) -> bool {
        self.inner.get("noModule").as_::<bool>()
    }

    /// Setter of the `noModule` attribute.
    /// [`HTMLScriptElement.noModule`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/noModule)
    pub fn set_no_module(&mut self, value: bool) {
        self.inner.set("noModule", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `async` attribute.
    /// [`HTMLScriptElement.async`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/async)
    pub fn async_(&self) -> bool {
        self.inner.get("async").as_::<bool>()
    }

    /// Setter of the `async` attribute.
    /// [`HTMLScriptElement.async`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/async)
    pub fn set_async_(&mut self, value: bool) {
        self.inner.set("async", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `defer` attribute.
    /// [`HTMLScriptElement.defer`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/defer)
    pub fn defer(&self) -> bool {
        self.inner.get("defer").as_::<bool>()
    }

    /// Setter of the `defer` attribute.
    /// [`HTMLScriptElement.defer`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/defer)
    pub fn set_defer(&mut self, value: bool) {
        self.inner.set("defer", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `blocking` attribute.
    /// [`HTMLScriptElement.blocking`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/blocking)
    pub fn blocking(&self) -> DOMTokenList {
        self.inner.get("blocking").as_::<DOMTokenList>()
    }
}
impl HTMLScriptElement {
    /// Getter of the `crossOrigin` attribute.
    /// [`HTMLScriptElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/crossOrigin)
    pub fn cross_origin(&self) -> JsString {
        self.inner.get("crossOrigin").as_::<JsString>()
    }

    /// Setter of the `crossOrigin` attribute.
    /// [`HTMLScriptElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/crossOrigin)
    pub fn set_cross_origin(&mut self, value: &JsString) {
        self.inner.set("crossOrigin", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `referrerPolicy` attribute.
    /// [`HTMLScriptElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/referrerPolicy)
    pub fn referrer_policy(&self) -> JsString {
        self.inner.get("referrerPolicy").as_::<JsString>()
    }

    /// Setter of the `referrerPolicy` attribute.
    /// [`HTMLScriptElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/referrerPolicy)
    pub fn set_referrer_policy(&mut self, value: &JsString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `integrity` attribute.
    /// [`HTMLScriptElement.integrity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/integrity)
    pub fn integrity(&self) -> JsString {
        self.inner.get("integrity").as_::<JsString>()
    }

    /// Setter of the `integrity` attribute.
    /// [`HTMLScriptElement.integrity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/integrity)
    pub fn set_integrity(&mut self, value: &JsString) {
        self.inner.set("integrity", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `fetchPriority` attribute.
    /// [`HTMLScriptElement.fetchPriority`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/fetchPriority)
    pub fn fetch_priority(&self) -> JsString {
        self.inner.get("fetchPriority").as_::<JsString>()
    }

    /// Setter of the `fetchPriority` attribute.
    /// [`HTMLScriptElement.fetchPriority`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/fetchPriority)
    pub fn set_fetch_priority(&mut self, value: &JsString) {
        self.inner.set("fetchPriority", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `text` attribute.
    /// [`HTMLScriptElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/text)
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }

    /// Setter of the `text` attribute.
    /// [`HTMLScriptElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/text)
    pub fn set_text(&mut self, value: &JsString) {
        self.inner.set("text", value);
    }
}
impl HTMLScriptElement {
    /// The supports method.
    /// [`HTMLScriptElement.supports`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/supports)
    pub fn supports(type_: &JsString) -> bool {
        Any::global("HTMLScriptElement")
            .call("supports", &[type_.into()])
            .as_::<bool>()
    }
}
impl HTMLScriptElement {
    /// Getter of the `charset` attribute.
    /// [`HTMLScriptElement.charset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/charset)
    pub fn charset(&self) -> JsString {
        self.inner.get("charset").as_::<JsString>()
    }

    /// Setter of the `charset` attribute.
    /// [`HTMLScriptElement.charset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/charset)
    pub fn set_charset(&mut self, value: &JsString) {
        self.inner.set("charset", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `event` attribute.
    /// [`HTMLScriptElement.event`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/event)
    pub fn event(&self) -> JsString {
        self.inner.get("event").as_::<JsString>()
    }

    /// Setter of the `event` attribute.
    /// [`HTMLScriptElement.event`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/event)
    pub fn set_event(&mut self, value: &JsString) {
        self.inner.set("event", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `htmlFor` attribute.
    /// [`HTMLScriptElement.htmlFor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/htmlFor)
    pub fn html_for(&self) -> JsString {
        self.inner.get("htmlFor").as_::<JsString>()
    }

    /// Setter of the `htmlFor` attribute.
    /// [`HTMLScriptElement.htmlFor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/htmlFor)
    pub fn set_html_for(&mut self, value: &JsString) {
        self.inner.set("htmlFor", value);
    }
}
impl HTMLScriptElement {
    /// Getter of the `attributionSrc` attribute.
    /// [`HTMLScriptElement.attributionSrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/attributionSrc)
    pub fn attribution_src(&self) -> JsString {
        self.inner.get("attributionSrc").as_::<JsString>()
    }

    /// Setter of the `attributionSrc` attribute.
    /// [`HTMLScriptElement.attributionSrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/attributionSrc)
    pub fn set_attribution_src(&mut self, value: &JsString) {
        self.inner.set("attributionSrc", value);
    }
}
