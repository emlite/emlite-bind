use super::*;




/// The SVGClipPathElement class.
/// [`SVGClipPathElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGClipPathElement {
    inner: SVGElement,
}

impl FromVal for SVGClipPathElement {
    fn from_val(v: &Any) -> Self {
        SVGClipPathElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGClipPathElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGClipPathElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGClipPathElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGClipPathElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGClipPathElement> for Any {
    fn from(s: SVGClipPathElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGClipPathElement> for Any {
    fn from(s: &SVGClipPathElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGClipPathElement);


impl SVGClipPathElement {
    /// Getter of the `clipPathUnits` attribute.
    /// [`SVGClipPathElement.clipPathUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/clipPathUnits)
    pub fn clip_path_units(&self) -> SVGAnimatedEnumeration {
        self.inner.get("clipPathUnits").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGClipPathElement {
    /// Getter of the `transform` attribute.
    /// [`SVGClipPathElement.transform`](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/transform)
    pub fn transform(&self) -> SVGAnimatedTransformList {
        self.inner.get("transform").as_::<SVGAnimatedTransformList>()
    }

}
