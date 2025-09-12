use super::*;

/// The HTMLObjectElement class.
/// [`HTMLObjectElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLObjectElement {
    inner: HTMLElement,
}

impl FromVal for HTMLObjectElement {
    fn from_val(v: &Any) -> Self {
        HTMLObjectElement {
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

impl core::ops::Deref for HTMLObjectElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLObjectElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLObjectElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLObjectElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLObjectElement> for Any {
    fn from(s: HTMLObjectElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLObjectElement> for Any {
    fn from(s: &HTMLObjectElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLObjectElement);

impl HTMLObjectElement {
    /// Getter of the `data` attribute.
    /// [`HTMLObjectElement.data`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)
    pub fn data(&self) -> JsString {
        self.inner.get("data").as_::<JsString>()
    }

    /// Setter of the `data` attribute.
    /// [`HTMLObjectElement.data`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)
    pub fn set_data(&mut self, value: &JsString) {
        self.inner.set("data", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `type` attribute.
    /// [`HTMLObjectElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLObjectElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `name` attribute.
    /// [`HTMLObjectElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLObjectElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `form` attribute.
    /// [`HTMLObjectElement.form`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/form)
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLObjectElement {
    /// Getter of the `width` attribute.
    /// [`HTMLObjectElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)
    pub fn width(&self) -> JsString {
        self.inner.get("width").as_::<JsString>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLObjectElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)
    pub fn set_width(&mut self, value: &JsString) {
        self.inner.set("width", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `height` attribute.
    /// [`HTMLObjectElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)
    pub fn height(&self) -> JsString {
        self.inner.get("height").as_::<JsString>()
    }

    /// Setter of the `height` attribute.
    /// [`HTMLObjectElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)
    pub fn set_height(&mut self, value: &JsString) {
        self.inner.set("height", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `contentDocument` attribute.
    /// [`HTMLObjectElement.contentDocument`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentDocument)
    pub fn content_document(&self) -> Document {
        self.inner.get("contentDocument").as_::<Document>()
    }
}
impl HTMLObjectElement {
    /// Getter of the `contentWindow` attribute.
    /// [`HTMLObjectElement.contentWindow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentWindow)
    pub fn content_window(&self) -> Any {
        self.inner.get("contentWindow").as_::<Any>()
    }
}
impl HTMLObjectElement {
    /// Getter of the `willValidate` attribute.
    /// [`HTMLObjectElement.willValidate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/willValidate)
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLObjectElement {
    /// Getter of the `validity` attribute.
    /// [`HTMLObjectElement.validity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validity)
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLObjectElement {
    /// Getter of the `validationMessage` attribute.
    /// [`HTMLObjectElement.validationMessage`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validationMessage)
    pub fn validation_message(&self) -> JsString {
        self.inner.get("validationMessage").as_::<JsString>()
    }
}
impl HTMLObjectElement {
    /// Getter of the `align` attribute.
    /// [`HTMLObjectElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)
    pub fn align(&self) -> JsString {
        self.inner.get("align").as_::<JsString>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLObjectElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)
    pub fn set_align(&mut self, value: &JsString) {
        self.inner.set("align", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `archive` attribute.
    /// [`HTMLObjectElement.archive`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)
    pub fn archive(&self) -> JsString {
        self.inner.get("archive").as_::<JsString>()
    }

    /// Setter of the `archive` attribute.
    /// [`HTMLObjectElement.archive`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)
    pub fn set_archive(&mut self, value: &JsString) {
        self.inner.set("archive", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `code` attribute.
    /// [`HTMLObjectElement.code`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)
    pub fn code(&self) -> JsString {
        self.inner.get("code").as_::<JsString>()
    }

    /// Setter of the `code` attribute.
    /// [`HTMLObjectElement.code`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)
    pub fn set_code(&mut self, value: &JsString) {
        self.inner.set("code", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `declare` attribute.
    /// [`HTMLObjectElement.declare`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)
    pub fn declare(&self) -> bool {
        self.inner.get("declare").as_::<bool>()
    }

    /// Setter of the `declare` attribute.
    /// [`HTMLObjectElement.declare`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)
    pub fn set_declare(&mut self, value: bool) {
        self.inner.set("declare", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `hspace` attribute.
    /// [`HTMLObjectElement.hspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)
    pub fn hspace(&self) -> u32 {
        self.inner.get("hspace").as_::<u32>()
    }

    /// Setter of the `hspace` attribute.
    /// [`HTMLObjectElement.hspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)
    pub fn set_hspace(&mut self, value: u32) {
        self.inner.set("hspace", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `standby` attribute.
    /// [`HTMLObjectElement.standby`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)
    pub fn standby(&self) -> JsString {
        self.inner.get("standby").as_::<JsString>()
    }

    /// Setter of the `standby` attribute.
    /// [`HTMLObjectElement.standby`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)
    pub fn set_standby(&mut self, value: &JsString) {
        self.inner.set("standby", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `vspace` attribute.
    /// [`HTMLObjectElement.vspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)
    pub fn vspace(&self) -> u32 {
        self.inner.get("vspace").as_::<u32>()
    }

    /// Setter of the `vspace` attribute.
    /// [`HTMLObjectElement.vspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)
    pub fn set_vspace(&mut self, value: u32) {
        self.inner.set("vspace", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `codeBase` attribute.
    /// [`HTMLObjectElement.codeBase`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)
    pub fn code_base(&self) -> JsString {
        self.inner.get("codeBase").as_::<JsString>()
    }

    /// Setter of the `codeBase` attribute.
    /// [`HTMLObjectElement.codeBase`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)
    pub fn set_code_base(&mut self, value: &JsString) {
        self.inner.set("codeBase", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `codeType` attribute.
    /// [`HTMLObjectElement.codeType`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)
    pub fn code_type(&self) -> JsString {
        self.inner.get("codeType").as_::<JsString>()
    }

    /// Setter of the `codeType` attribute.
    /// [`HTMLObjectElement.codeType`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)
    pub fn set_code_type(&mut self, value: &JsString) {
        self.inner.set("codeType", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `useMap` attribute.
    /// [`HTMLObjectElement.useMap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)
    pub fn use_map(&self) -> JsString {
        self.inner.get("useMap").as_::<JsString>()
    }

    /// Setter of the `useMap` attribute.
    /// [`HTMLObjectElement.useMap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)
    pub fn set_use_map(&mut self, value: &JsString) {
        self.inner.set("useMap", value);
    }
}
impl HTMLObjectElement {
    /// Getter of the `border` attribute.
    /// [`HTMLObjectElement.border`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)
    pub fn border(&self) -> JsString {
        self.inner.get("border").as_::<JsString>()
    }

    /// Setter of the `border` attribute.
    /// [`HTMLObjectElement.border`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)
    pub fn set_border(&mut self, value: &JsString) {
        self.inner.set("border", value);
    }
}

impl HTMLObjectElement {
    /// The `new HTMLObjectElement(..)` constructor, creating a new HTMLObjectElement instance
    pub fn new() -> HTMLObjectElement {
        Self {
            inner: Any::global("HTMLObjectElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}

impl HTMLObjectElement {
    /// The getSVGDocument method.
    /// [`HTMLObjectElement.getSVGDocument`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/getSVGDocument)
    pub fn get_svg_document(&self) -> Document {
        self.inner.call("getSVGDocument", &[]).as_::<Document>()
    }
}
impl HTMLObjectElement {
    /// The checkValidity method.
    /// [`HTMLObjectElement.checkValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/checkValidity)
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLObjectElement {
    /// The reportValidity method.
    /// [`HTMLObjectElement.reportValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/reportValidity)
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLObjectElement {
    /// The setCustomValidity method.
    /// [`HTMLObjectElement.setCustomValidity`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/setCustomValidity)
    pub fn set_custom_validity(&self, error: &JsString) -> Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<Undefined>()
    }
}
