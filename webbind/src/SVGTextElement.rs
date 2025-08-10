use super::*;

/// The SVGTextElement class.
/// [`SVGTextElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGTextElement {
    inner: SVGTextPositioningElement,
}

impl FromVal for SVGTextElement {
    fn from_val(v: &Any) -> Self {
        SVGTextElement {
            inner: SVGTextPositioningElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGTextElement {
    type Target = SVGTextPositioningElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGTextElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGTextElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGTextElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGTextElement> for Any {
    fn from(s: SVGTextElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGTextElement> for Any {
    fn from(s: &SVGTextElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGTextElement);
