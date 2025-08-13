use super::*;




/// The SVGFEDistantLightElement class.
/// [`SVGFEDistantLightElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEDistantLightElement {
    inner: SVGElement,
}

impl FromVal for SVGFEDistantLightElement {
    fn from_val(v: &Any) -> Self {
        SVGFEDistantLightElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGFEDistantLightElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEDistantLightElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEDistantLightElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEDistantLightElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGFEDistantLightElement> for Any {
    fn from(s: SVGFEDistantLightElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEDistantLightElement> for Any {
    fn from(s: &SVGFEDistantLightElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEDistantLightElement);


impl SVGFEDistantLightElement {
    /// Getter of the `azimuth` attribute.
    /// [`SVGFEDistantLightElement.azimuth`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement/azimuth)
    pub fn azimuth(&self) -> SVGAnimatedNumber {
        self.inner.get("azimuth").as_::<SVGAnimatedNumber>()
    }

}
impl SVGFEDistantLightElement {
    /// Getter of the `elevation` attribute.
    /// [`SVGFEDistantLightElement.elevation`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement/elevation)
    pub fn elevation(&self) -> SVGAnimatedNumber {
        self.inner.get("elevation").as_::<SVGAnimatedNumber>()
    }

}
