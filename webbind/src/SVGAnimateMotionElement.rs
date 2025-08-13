use super::*;




/// The SVGAnimateMotionElement class.
/// [`SVGAnimateMotionElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimateMotionElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimateMotionElement {
    inner: SVGAnimationElement,
}

impl FromVal for SVGAnimateMotionElement {
    fn from_val(v: &Any) -> Self {
        SVGAnimateMotionElement { inner: SVGAnimationElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGAnimateMotionElement {
    type Target = SVGAnimationElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGAnimateMotionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGAnimateMotionElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGAnimateMotionElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGAnimateMotionElement> for Any {
    fn from(s: SVGAnimateMotionElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGAnimateMotionElement> for Any {
    fn from(s: &SVGAnimateMotionElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGAnimateMotionElement);


