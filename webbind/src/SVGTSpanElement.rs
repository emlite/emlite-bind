use super::*;




/// The SVGTSpanElement class.
/// [`SVGTSpanElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTSpanElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGTSpanElement {
    inner: SVGTextPositioningElement,
}

impl FromVal for SVGTSpanElement {
    fn from_val(v: &Any) -> Self {
        SVGTSpanElement { inner: SVGTextPositioningElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGTSpanElement {
    type Target = SVGTextPositioningElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGTSpanElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGTSpanElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGTSpanElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGTSpanElement> for Any {
    fn from(s: SVGTSpanElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGTSpanElement> for Any {
    fn from(s: &SVGTSpanElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGTSpanElement);


