use super::*;

/// The HTMLFormElement class.
/// [`HTMLFormElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFormElement {
    inner: HTMLElement,
}

impl FromVal for HTMLFormElement {
    fn from_val(v: &Any) -> Self {
        HTMLFormElement {
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

impl core::ops::Deref for HTMLFormElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLFormElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLFormElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLFormElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLFormElement> for Any {
    fn from(s: HTMLFormElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLFormElement> for Any {
    fn from(s: &HTMLFormElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLFormElement);

impl HTMLFormElement {
    /// Getter of the `acceptCharset` attribute.
    /// [`HTMLFormElement.acceptCharset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/acceptCharset)
    pub fn accept_charset(&self) -> JsString {
        self.inner.get("acceptCharset").as_::<JsString>()
    }

    /// Setter of the `acceptCharset` attribute.
    /// [`HTMLFormElement.acceptCharset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/acceptCharset)
    pub fn set_accept_charset(&mut self, value: &JsString) {
        self.inner.set("acceptCharset", value);
    }
}
impl HTMLFormElement {
    /// Getter of the `action` attribute.
    /// [`HTMLFormElement.action`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/action)
    pub fn action(&self) -> JsString {
        self.inner.get("action").as_::<JsString>()
    }

    /// Setter of the `action` attribute.
    /// [`HTMLFormElement.action`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/action)
    pub fn set_action(&mut self, value: &JsString) {
        self.inner.set("action", value);
    }
}
impl HTMLFormElement {
    /// Getter of the `autocomplete` attribute.
    /// [`HTMLFormElement.autocomplete`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/autocomplete)
    pub fn autocomplete(&self) -> JsString {
        self.inner.get("autocomplete").as_::<JsString>()
    }

    /// Setter of the `autocomplete` attribute.
    /// [`HTMLFormElement.autocomplete`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/autocomplete)
    pub fn set_autocomplete(&mut self, value: &JsString) {
        self.inner.set("autocomplete", value);
    }
}
impl HTMLFormElement {
    /// Getter of the `enctype` attribute.
    /// [`HTMLFormElement.enctype`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype)
    pub fn enctype(&self) -> JsString {
        self.inner.get("enctype").as_::<JsString>()
    }

    /// Setter of the `enctype` attribute.
    /// [`HTMLFormElement.enctype`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype)
    pub fn set_enctype(&mut self, value: &JsString) {
        self.inner.set("enctype", value);
    }
}
impl HTMLFormElement {
    /// Getter of the `encoding` attribute.
    /// [`HTMLFormElement.encoding`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/encoding)
    pub fn encoding(&self) -> JsString {
        self.inner.get("encoding").as_::<JsString>()
    }

    /// Setter of the `encoding` attribute.
    /// [`HTMLFormElement.encoding`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/encoding)
    pub fn set_encoding(&mut self, value: &JsString) {
        self.inner.set("encoding", value);
    }
}
impl HTMLFormElement {
    /// Getter of the `method` attribute.
    /// [`HTMLFormElement.method`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/method)
    pub fn method(&self) -> JsString {
        self.inner.get("method").as_::<JsString>()
    }

    /// Setter of the `method` attribute.
    /// [`HTMLFormElement.method`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/method)
    pub fn set_method(&mut self, value: &JsString) {
        self.inner.set("method", value);
    }
}
impl HTMLFormElement {
    /// Getter of the `name` attribute.
    /// [`HTMLFormElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLFormElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl HTMLFormElement {
    /// Getter of the `noValidate` attribute.
    /// [`HTMLFormElement.noValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/noValidate)
    pub fn no_validate(&self) -> bool {
        self.inner.get("noValidate").as_::<bool>()
    }

    /// Setter of the `noValidate` attribute.
    /// [`HTMLFormElement.noValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/noValidate)
    pub fn set_no_validate(&mut self, value: bool) {
        self.inner.set("noValidate", value);
    }
}
impl HTMLFormElement {
    /// Getter of the `target` attribute.
    /// [`HTMLFormElement.target`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/target)
    pub fn target(&self) -> JsString {
        self.inner.get("target").as_::<JsString>()
    }

    /// Setter of the `target` attribute.
    /// [`HTMLFormElement.target`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/target)
    pub fn set_target(&mut self, value: &JsString) {
        self.inner.set("target", value);
    }
}
impl HTMLFormElement {
    /// Getter of the `rel` attribute.
    /// [`HTMLFormElement.rel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/rel)
    pub fn rel(&self) -> JsString {
        self.inner.get("rel").as_::<JsString>()
    }

    /// Setter of the `rel` attribute.
    /// [`HTMLFormElement.rel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/rel)
    pub fn set_rel(&mut self, value: &JsString) {
        self.inner.set("rel", value);
    }
}
impl HTMLFormElement {
    /// Getter of the `relList` attribute.
    /// [`HTMLFormElement.relList`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/relList)
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl HTMLFormElement {
    /// Getter of the `elements` attribute.
    /// [`HTMLFormElement.elements`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/elements)
    pub fn elements(&self) -> HTMLFormControlsCollection {
        self.inner
            .get("elements")
            .as_::<HTMLFormControlsCollection>()
    }
}
impl HTMLFormElement {
    /// Getter of the `length` attribute.
    /// [`HTMLFormElement.length`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}

impl HTMLFormElement {
    /// The `new HTMLFormElement(..)` constructor, creating a new HTMLFormElement instance
    pub fn new() -> HTMLFormElement {
        Self {
            inner: Any::global("HTMLFormElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}

impl HTMLFormElement {
    /// The submit method.
    /// [`HTMLFormElement.submit`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit)
    pub fn submit(&self) -> Undefined {
        self.inner.call("submit", &[]).as_::<Undefined>()
    }
}
impl HTMLFormElement {
    /// The requestSubmit method.
    /// [`HTMLFormElement.requestSubmit`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/requestSubmit)
    pub fn request_submit(&self) -> Undefined {
        self.inner.call("requestSubmit", &[]).as_::<Undefined>()
    }
}
impl HTMLFormElement {
    /// The requestSubmit method.
    /// [`HTMLFormElement.requestSubmit`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/requestSubmit)
    pub fn request_submit_with_submitter(&self, submitter: &HTMLElement) -> Undefined {
        self.inner
            .call("requestSubmit", &[submitter.into()])
            .as_::<Undefined>()
    }
}
impl HTMLFormElement {
    /// The reset method.
    /// [`HTMLFormElement.reset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset)
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl HTMLFormElement {
    /// The checkValidity method.
    /// [`HTMLFormElement.checkValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/checkValidity)
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLFormElement {
    /// The reportValidity method.
    /// [`HTMLFormElement.reportValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reportValidity)
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
