use super::*;




/// The MathMLElement class.
/// [`MathMLElement`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MathMLElement {
    inner: Element,
}

impl FromVal for MathMLElement {
    fn from_val(v: &Any) -> Self {
        MathMLElement { inner: Element::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MathMLElement {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MathMLElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MathMLElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MathMLElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MathMLElement> for Any {
    fn from(s: MathMLElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MathMLElement> for Any {
    fn from(s: &MathMLElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MathMLElement);


impl MathMLElement {
    /// Getter of the `style` attribute.
    /// [`MathMLElement.style`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/style)
    pub fn style(&self) -> CSSStyleProperties {
        self.inner.get("style").as_::<CSSStyleProperties>()
    }

}
impl MathMLElement {
    /// Getter of the `onbeforexrselect` attribute.
    /// [`MathMLElement.onbeforexrselect`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onbeforexrselect)
    pub fn onbeforexrselect(&self) -> Any {
        self.inner.get("onbeforexrselect").as_::<Any>()
    }

    /// Setter of the `onbeforexrselect` attribute.
    /// [`MathMLElement.onbeforexrselect`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onbeforexrselect)
    pub fn set_onbeforexrselect(&mut self, value: &Any) {
        self.inner.set("onbeforexrselect", value);
    }
}
impl MathMLElement {
    /// Getter of the `dataset` attribute.
    /// [`MathMLElement.dataset`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/dataset)
    pub fn dataset(&self) -> DOMStringMap {
        self.inner.get("dataset").as_::<DOMStringMap>()
    }

}
impl MathMLElement {
    /// Getter of the `nonce` attribute.
    /// [`MathMLElement.nonce`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/nonce)
    pub fn nonce(&self) -> JsString {
        self.inner.get("nonce").as_::<JsString>()
    }

    /// Setter of the `nonce` attribute.
    /// [`MathMLElement.nonce`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/nonce)
    pub fn set_nonce(&mut self, value: &JsString) {
        self.inner.set("nonce", value);
    }
}
impl MathMLElement {
    /// Getter of the `autofocus` attribute.
    /// [`MathMLElement.autofocus`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/autofocus)
    pub fn autofocus(&self) -> bool {
        self.inner.get("autofocus").as_::<bool>()
    }

    /// Setter of the `autofocus` attribute.
    /// [`MathMLElement.autofocus`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/autofocus)
    pub fn set_autofocus(&mut self, value: bool) {
        self.inner.set("autofocus", value);
    }
}
impl MathMLElement {
    /// Getter of the `tabIndex` attribute.
    /// [`MathMLElement.tabIndex`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/tabIndex)
    pub fn tab_index(&self) -> i32 {
        self.inner.get("tabIndex").as_::<i32>()
    }

    /// Setter of the `tabIndex` attribute.
    /// [`MathMLElement.tabIndex`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/tabIndex)
    pub fn set_tab_index(&mut self, value: i32) {
        self.inner.set("tabIndex", value);
    }
}
impl MathMLElement {
    /// The focus method.
    /// [`MathMLElement.focus`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/focus)
    pub fn focus0(&self, ) -> Undefined {
        self.inner.call("focus", &[]).as_::<Undefined>()
    }
    /// The focus method.
    /// [`MathMLElement.focus`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/focus)
    pub fn focus1(&self, options: &FocusOptions) -> Undefined {
        self.inner.call("focus", &[options.into(), ]).as_::<Undefined>()
    }
}
impl MathMLElement {
    /// The blur method.
    /// [`MathMLElement.blur`](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/blur)
    pub fn blur(&self, ) -> Undefined {
        self.inner.call("blur", &[]).as_::<Undefined>()
    }
}
