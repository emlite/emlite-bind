use super::*;

/// The SVGFEMergeNodeElement class.
/// [`SVGFEMergeNodeElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeNodeElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEMergeNodeElement {
    inner: SVGElement,
}

impl FromVal for SVGFEMergeNodeElement {
    fn from_val(v: &Any) -> Self {
        SVGFEMergeNodeElement {
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

impl core::ops::Deref for SVGFEMergeNodeElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEMergeNodeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEMergeNodeElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEMergeNodeElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGFEMergeNodeElement> for Any {
    fn from(s: SVGFEMergeNodeElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEMergeNodeElement> for Any {
    fn from(s: &SVGFEMergeNodeElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEMergeNodeElement);

impl SVGFEMergeNodeElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFEMergeNodeElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeNodeElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
