use super::*;

/// The HTMLTemplateElement class.
/// [`HTMLTemplateElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTemplateElement {
    inner: HTMLElement,
}

impl FromVal for HTMLTemplateElement {
    fn from_val(v: &Any) -> Self {
        HTMLTemplateElement {
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

impl core::ops::Deref for HTMLTemplateElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLTemplateElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLTemplateElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLTemplateElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLTemplateElement> for Any {
    fn from(s: HTMLTemplateElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLTemplateElement> for Any {
    fn from(s: &HTMLTemplateElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLTemplateElement);

impl HTMLTemplateElement {
    /// The `new HTMLTemplateElement(..)` constructor, creating a new HTMLTemplateElement instance
    pub fn new() -> HTMLTemplateElement {
        Self {
            inner: Any::global("HTMLTemplateElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTemplateElement {
    /// Getter of the `content` attribute.
    /// [`HTMLTemplateElement.content`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/content)
    pub fn content(&self) -> DocumentFragment {
        self.inner.get("content").as_::<DocumentFragment>()
    }
}
impl HTMLTemplateElement {
    /// Getter of the `shadowRootMode` attribute.
    /// [`HTMLTemplateElement.shadowRootMode`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/shadowRootMode)
    pub fn shadow_root_mode(&self) -> JsString {
        self.inner.get("shadowRootMode").as_::<JsString>()
    }

    /// Setter of the `shadowRootMode` attribute.
    /// [`HTMLTemplateElement.shadowRootMode`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/shadowRootMode)
    pub fn set_shadow_root_mode(&mut self, value: &JsString) {
        self.inner.set("shadowRootMode", value);
    }
}
impl HTMLTemplateElement {
    /// Getter of the `shadowRootDelegatesFocus` attribute.
    /// [`HTMLTemplateElement.shadowRootDelegatesFocus`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/shadowRootDelegatesFocus)
    pub fn shadow_root_delegates_focus(&self) -> bool {
        self.inner.get("shadowRootDelegatesFocus").as_::<bool>()
    }

    /// Setter of the `shadowRootDelegatesFocus` attribute.
    /// [`HTMLTemplateElement.shadowRootDelegatesFocus`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/shadowRootDelegatesFocus)
    pub fn set_shadow_root_delegates_focus(&mut self, value: bool) {
        self.inner.set("shadowRootDelegatesFocus", value);
    }
}
impl HTMLTemplateElement {
    /// Getter of the `shadowRootClonable` attribute.
    /// [`HTMLTemplateElement.shadowRootClonable`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/shadowRootClonable)
    pub fn shadow_root_clonable(&self) -> bool {
        self.inner.get("shadowRootClonable").as_::<bool>()
    }

    /// Setter of the `shadowRootClonable` attribute.
    /// [`HTMLTemplateElement.shadowRootClonable`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/shadowRootClonable)
    pub fn set_shadow_root_clonable(&mut self, value: bool) {
        self.inner.set("shadowRootClonable", value);
    }
}
impl HTMLTemplateElement {
    /// Getter of the `shadowRootSerializable` attribute.
    /// [`HTMLTemplateElement.shadowRootSerializable`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/shadowRootSerializable)
    pub fn shadow_root_serializable(&self) -> bool {
        self.inner.get("shadowRootSerializable").as_::<bool>()
    }

    /// Setter of the `shadowRootSerializable` attribute.
    /// [`HTMLTemplateElement.shadowRootSerializable`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/shadowRootSerializable)
    pub fn set_shadow_root_serializable(&mut self, value: bool) {
        self.inner.set("shadowRootSerializable", value);
    }
}
impl HTMLTemplateElement {
    /// Getter of the `shadowRootCustomElementRegistry` attribute.
    /// [`HTMLTemplateElement.shadowRootCustomElementRegistry`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/shadowRootCustomElementRegistry)
    pub fn shadow_root_custom_element_registry(&self) -> JsString {
        self.inner
            .get("shadowRootCustomElementRegistry")
            .as_::<JsString>()
    }

    /// Setter of the `shadowRootCustomElementRegistry` attribute.
    /// [`HTMLTemplateElement.shadowRootCustomElementRegistry`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/shadowRootCustomElementRegistry)
    pub fn set_shadow_root_custom_element_registry(&mut self, value: &JsString) {
        self.inner.set("shadowRootCustomElementRegistry", value);
    }
}
