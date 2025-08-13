use super::*;




/// The SVGScriptElement class.
/// [`SVGScriptElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGScriptElement {
    inner: SVGElement,
}

impl FromVal for SVGScriptElement {
    fn from_val(v: &Any) -> Self {
        SVGScriptElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGScriptElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGScriptElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGScriptElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGScriptElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGScriptElement> for Any {
    fn from(s: SVGScriptElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGScriptElement> for Any {
    fn from(s: &SVGScriptElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGScriptElement);


impl SVGScriptElement {
    /// Getter of the `type` attribute.
    /// [`SVGScriptElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    /// [`SVGScriptElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/type)
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl SVGScriptElement {
    /// Getter of the `crossOrigin` attribute.
    /// [`SVGScriptElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/crossOrigin)
    pub fn cross_origin(&self) -> JsString {
        self.inner.get("crossOrigin").as_::<JsString>()
    }

    /// Setter of the `crossOrigin` attribute.
    /// [`SVGScriptElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/crossOrigin)
    pub fn set_cross_origin(&mut self, value: &JsString) {
        self.inner.set("crossOrigin", value);
    }
}
impl SVGScriptElement {
    /// Getter of the `href` attribute.
    /// [`SVGScriptElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/href)
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }

}
