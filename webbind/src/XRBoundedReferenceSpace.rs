use super::*;




/// The XRBoundedReferenceSpace class.
/// [`XRBoundedReferenceSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRBoundedReferenceSpace)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRBoundedReferenceSpace {
    inner: XRReferenceSpace,
}

impl FromVal for XRBoundedReferenceSpace {
    fn from_val(v: &Any) -> Self {
        XRBoundedReferenceSpace { inner: XRReferenceSpace::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRBoundedReferenceSpace {
    type Target = XRReferenceSpace;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRBoundedReferenceSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRBoundedReferenceSpace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRBoundedReferenceSpace {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRBoundedReferenceSpace> for Any {
    fn from(s: XRBoundedReferenceSpace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRBoundedReferenceSpace> for Any {
    fn from(s: &XRBoundedReferenceSpace) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRBoundedReferenceSpace);


impl XRBoundedReferenceSpace {
    /// Getter of the `boundsGeometry` attribute.
    /// [`XRBoundedReferenceSpace.boundsGeometry`](https://developer.mozilla.org/en-US/docs/Web/API/XRBoundedReferenceSpace/boundsGeometry)
    pub fn bounds_geometry(&self) -> TypedArray<DOMPointReadOnly> {
        self.inner.get("boundsGeometry").as_::<TypedArray<DOMPointReadOnly>>()
    }

}
