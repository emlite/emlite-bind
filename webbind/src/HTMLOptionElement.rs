use super::*;

/// The HTMLOptionElement class.
/// [`HTMLOptionElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLOptionElement {
    inner: HTMLElement,
}

impl FromVal for HTMLOptionElement {
    fn from_val(v: &Any) -> Self {
        HTMLOptionElement {
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

impl core::ops::Deref for HTMLOptionElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLOptionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLOptionElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLOptionElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLOptionElement> for Any {
    fn from(s: HTMLOptionElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLOptionElement> for Any {
    fn from(s: &HTMLOptionElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLOptionElement);

impl HTMLOptionElement {
    /// Getter of the `disabled` attribute.
    /// [`HTMLOptionElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/disabled)
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    /// Setter of the `disabled` attribute.
    /// [`HTMLOptionElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/disabled)
    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLOptionElement {
    /// Getter of the `form` attribute.
    /// [`HTMLOptionElement.form`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/form)
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLOptionElement {
    /// Getter of the `label` attribute.
    /// [`HTMLOptionElement.label`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`HTMLOptionElement.label`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl HTMLOptionElement {
    /// Getter of the `defaultSelected` attribute.
    /// [`HTMLOptionElement.defaultSelected`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/defaultSelected)
    pub fn default_selected(&self) -> bool {
        self.inner.get("defaultSelected").as_::<bool>()
    }

    /// Setter of the `defaultSelected` attribute.
    /// [`HTMLOptionElement.defaultSelected`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/defaultSelected)
    pub fn set_default_selected(&mut self, value: bool) {
        self.inner.set("defaultSelected", value);
    }
}
impl HTMLOptionElement {
    /// Getter of the `selected` attribute.
    /// [`HTMLOptionElement.selected`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/selected)
    pub fn selected(&self) -> bool {
        self.inner.get("selected").as_::<bool>()
    }

    /// Setter of the `selected` attribute.
    /// [`HTMLOptionElement.selected`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/selected)
    pub fn set_selected(&mut self, value: bool) {
        self.inner.set("selected", value);
    }
}
impl HTMLOptionElement {
    /// Getter of the `value` attribute.
    /// [`HTMLOptionElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/value)
    pub fn value(&self) -> JsString {
        self.inner.get("value").as_::<JsString>()
    }

    /// Setter of the `value` attribute.
    /// [`HTMLOptionElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/value)
    pub fn set_value(&mut self, value: &JsString) {
        self.inner.set("value", value);
    }
}
impl HTMLOptionElement {
    /// Getter of the `text` attribute.
    /// [`HTMLOptionElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/text)
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }

    /// Setter of the `text` attribute.
    /// [`HTMLOptionElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/text)
    pub fn set_text(&mut self, value: &JsString) {
        self.inner.set("text", value);
    }
}
impl HTMLOptionElement {
    /// Getter of the `index` attribute.
    /// [`HTMLOptionElement.index`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/index)
    pub fn index(&self) -> i32 {
        self.inner.get("index").as_::<i32>()
    }
}

impl HTMLOptionElement {
    /// The `new HTMLOptionElement(..)` constructor, creating a new HTMLOptionElement instance
    pub fn new() -> HTMLOptionElement {
        Self {
            inner: Any::global("HTMLOptionElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
