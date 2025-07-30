use super::*;

/// The HTMLStyleElement class.
/// [`HTMLStyleElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLStyleElement {
    inner: HTMLElement,
}
impl FromVal for HTMLStyleElement {
    fn from_val(v: &Any) -> Self {
        HTMLStyleElement {
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
impl core::ops::Deref for HTMLStyleElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLStyleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLStyleElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLStyleElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLStyleElement> for Any {
    fn from(s: HTMLStyleElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLStyleElement> for Any {
    fn from(s: &HTMLStyleElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLStyleElement);

impl HTMLStyleElement {
    /// The `new HTMLStyleElement(..)` constructor, creating a new HTMLStyleElement instance
    pub fn new() -> HTMLStyleElement {
        Self {
            inner: Any::global("HTMLStyleElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLStyleElement {
    /// Getter of the `disabled` attribute.
    /// [`HTMLStyleElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/disabled)
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    /// Setter of the `disabled` attribute.
    /// [`HTMLStyleElement.disabled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/disabled)
    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLStyleElement {
    /// Getter of the `media` attribute.
    /// [`HTMLStyleElement.media`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/media)
    pub fn media(&self) -> JsString {
        self.inner.get("media").as_::<JsString>()
    }

    /// Setter of the `media` attribute.
    /// [`HTMLStyleElement.media`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/media)
    pub fn set_media(&mut self, value: &JsString) {
        self.inner.set("media", value);
    }
}
impl HTMLStyleElement {
    /// Getter of the `blocking` attribute.
    /// [`HTMLStyleElement.blocking`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/blocking)
    pub fn blocking(&self) -> DOMTokenList {
        self.inner.get("blocking").as_::<DOMTokenList>()
    }
}
impl HTMLStyleElement {
    /// Getter of the `type` attribute.
    /// [`HTMLStyleElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLStyleElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/type)
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl HTMLStyleElement {
    /// Getter of the `sheet` attribute.
    /// [`HTMLStyleElement.sheet`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/sheet)
    pub fn sheet(&self) -> CSSStyleSheet {
        self.inner.get("sheet").as_::<CSSStyleSheet>()
    }
}
