use super::*;




/// The SVGSwitchElement class.
/// [`SVGSwitchElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSwitchElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGSwitchElement {
    inner: SVGGraphicsElement,
}

impl FromVal for SVGSwitchElement {
    fn from_val(v: &Any) -> Self {
        SVGSwitchElement { inner: SVGGraphicsElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGSwitchElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGSwitchElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGSwitchElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGSwitchElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGSwitchElement> for Any {
    fn from(s: SVGSwitchElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGSwitchElement> for Any {
    fn from(s: &SVGSwitchElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGSwitchElement);


