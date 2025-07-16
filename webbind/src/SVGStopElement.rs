use super::*;

/// The SVGStopElement class.
/// [`SVGStopElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStopElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGStopElement {
    inner: SVGElement,
}
impl FromVal for SVGStopElement {
    fn from_val(v: &Any) -> Self {
        SVGStopElement {
            inner: SVGElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGStopElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGStopElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGStopElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGStopElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGStopElement> for Any {
    fn from(s: SVGStopElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGStopElement> for Any {
    fn from(s: &SVGStopElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGStopElement);

impl SVGStopElement {
    /// Getter of the `offset` attribute.
    /// [`SVGStopElement.offset`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStopElement/offset)
    pub fn offset(&self) -> SVGAnimatedNumber {
        self.inner.get("offset").as_::<SVGAnimatedNumber>()
    }
}
