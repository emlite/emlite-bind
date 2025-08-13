use super::*;




/// The SVGTextPathElement class.
/// [`SVGTextPathElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGTextPathElement {
    inner: SVGTextContentElement,
}

impl FromVal for SVGTextPathElement {
    fn from_val(v: &Any) -> Self {
        SVGTextPathElement { inner: SVGTextContentElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGTextPathElement {
    type Target = SVGTextContentElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGTextPathElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGTextPathElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGTextPathElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGTextPathElement> for Any {
    fn from(s: SVGTextPathElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGTextPathElement> for Any {
    fn from(s: &SVGTextPathElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGTextPathElement);


impl SVGTextPathElement {
    /// Getter of the `startOffset` attribute.
    /// [`SVGTextPathElement.startOffset`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/startOffset)
    pub fn start_offset(&self) -> SVGAnimatedLength {
        self.inner.get("startOffset").as_::<SVGAnimatedLength>()
    }

}
impl SVGTextPathElement {
    /// Getter of the `method` attribute.
    /// [`SVGTextPathElement.method`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/method)
    pub fn method(&self) -> SVGAnimatedEnumeration {
        self.inner.get("method").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGTextPathElement {
    /// Getter of the `spacing` attribute.
    /// [`SVGTextPathElement.spacing`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/spacing)
    pub fn spacing(&self) -> SVGAnimatedEnumeration {
        self.inner.get("spacing").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGTextPathElement {
    /// Getter of the `href` attribute.
    /// [`SVGTextPathElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/href)
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }

}
