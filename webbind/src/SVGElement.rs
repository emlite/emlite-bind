use super::*;




/// The SVGElement class.
/// [`SVGElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGElement {
    inner: Element,
}

impl FromVal for SVGElement {
    fn from_val(v: &Any) -> Self {
        SVGElement { inner: Element::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGElement {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGElement> for Any {
    fn from(s: SVGElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGElement> for Any {
    fn from(s: &SVGElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGElement);


impl SVGElement {
    /// Getter of the `className` attribute.
    /// [`SVGElement.className`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/className)
    pub fn class_name(&self) -> SVGAnimatedString {
        self.inner.get("className").as_::<SVGAnimatedString>()
    }

}
impl SVGElement {
    /// Getter of the `ownerSVGElement` attribute.
    /// [`SVGElement.ownerSVGElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ownerSVGElement)
    pub fn owner_svg_element(&self) -> SVGSVGElement {
        self.inner.get("ownerSVGElement").as_::<SVGSVGElement>()
    }

}
impl SVGElement {
    /// Getter of the `viewportElement` attribute.
    /// [`SVGElement.viewportElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/viewportElement)
    pub fn viewport_element(&self) -> SVGElement {
        self.inner.get("viewportElement").as_::<SVGElement>()
    }

}
impl SVGElement {
    /// Getter of the `onbeforexrselect` attribute.
    /// [`SVGElement.onbeforexrselect`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onbeforexrselect)
    pub fn onbeforexrselect(&self) -> Any {
        self.inner.get("onbeforexrselect").as_::<Any>()
    }

    /// Setter of the `onbeforexrselect` attribute.
    /// [`SVGElement.onbeforexrselect`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onbeforexrselect)
    pub fn set_onbeforexrselect(&mut self, value: &Any) {
        self.inner.set("onbeforexrselect", value);
    }
}
impl SVGElement {
    /// Getter of the `correspondingElement` attribute.
    /// [`SVGElement.correspondingElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/correspondingElement)
    pub fn corresponding_element(&self) -> SVGElement {
        self.inner.get("correspondingElement").as_::<SVGElement>()
    }

}
impl SVGElement {
    /// Getter of the `correspondingUseElement` attribute.
    /// [`SVGElement.correspondingUseElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/correspondingUseElement)
    pub fn corresponding_use_element(&self) -> SVGUseElement {
        self.inner.get("correspondingUseElement").as_::<SVGUseElement>()
    }

}
impl SVGElement {
    /// Getter of the `dataset` attribute.
    /// [`SVGElement.dataset`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/dataset)
    pub fn dataset(&self) -> DOMStringMap {
        self.inner.get("dataset").as_::<DOMStringMap>()
    }

}
impl SVGElement {
    /// Getter of the `nonce` attribute.
    /// [`SVGElement.nonce`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/nonce)
    pub fn nonce(&self) -> JsString {
        self.inner.get("nonce").as_::<JsString>()
    }

    /// Setter of the `nonce` attribute.
    /// [`SVGElement.nonce`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/nonce)
    pub fn set_nonce(&mut self, value: &JsString) {
        self.inner.set("nonce", value);
    }
}
impl SVGElement {
    /// Getter of the `autofocus` attribute.
    /// [`SVGElement.autofocus`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/autofocus)
    pub fn autofocus(&self) -> bool {
        self.inner.get("autofocus").as_::<bool>()
    }

    /// Setter of the `autofocus` attribute.
    /// [`SVGElement.autofocus`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/autofocus)
    pub fn set_autofocus(&mut self, value: bool) {
        self.inner.set("autofocus", value);
    }
}
impl SVGElement {
    /// Getter of the `tabIndex` attribute.
    /// [`SVGElement.tabIndex`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)
    pub fn tab_index(&self) -> i32 {
        self.inner.get("tabIndex").as_::<i32>()
    }

    /// Setter of the `tabIndex` attribute.
    /// [`SVGElement.tabIndex`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)
    pub fn set_tab_index(&mut self, value: i32) {
        self.inner.set("tabIndex", value);
    }
}
impl SVGElement {
    /// The focus method.
    /// [`SVGElement.focus`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/focus)
    pub fn focus0(&self, ) -> Undefined {
        self.inner.call("focus", &[]).as_::<Undefined>()
    }
    /// The focus method.
    /// [`SVGElement.focus`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/focus)
    pub fn focus1(&self, options: &FocusOptions) -> Undefined {
        self.inner.call("focus", &[options.into(), ]).as_::<Undefined>()
    }
}
impl SVGElement {
    /// The blur method.
    /// [`SVGElement.blur`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/blur)
    pub fn blur(&self, ) -> Undefined {
        self.inner.call("blur", &[]).as_::<Undefined>()
    }
}
impl SVGElement {
    /// Getter of the `style` attribute.
    /// [`SVGElement.style`](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/style)
    pub fn style(&self) -> CSSStyleDeclaration {
        self.inner.get("style").as_::<CSSStyleDeclaration>()
    }

}
