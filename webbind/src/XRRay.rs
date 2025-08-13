use super::*;




/// The XRRay class.
/// [`XRRay`](https://developer.mozilla.org/en-US/docs/Web/API/XRRay)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRRay {
    inner: Any,
}

impl FromVal for XRRay {
    fn from_val(v: &Any) -> Self {
        XRRay { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRRay {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRRay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRRay {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRRay {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRRay> for Any {
    fn from(s: XRRay) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRRay> for Any {
    fn from(s: &XRRay) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRRay);



impl XRRay {
    /// The `new XRRay(..)` constructor, creating a new XRRay instance
    pub fn new(transform: &XRRigidTransform) -> XRRay {
        Self {
            inner: Any::global("XRRay").new(&[transform.into()]).as_::<Any>(),
        }
    }

}
impl XRRay {
    /// Getter of the `origin` attribute.
    /// [`XRRay.origin`](https://developer.mozilla.org/en-US/docs/Web/API/XRRay/origin)
    pub fn origin(&self) -> DOMPointReadOnly {
        self.inner.get("origin").as_::<DOMPointReadOnly>()
    }

}
impl XRRay {
    /// Getter of the `direction` attribute.
    /// [`XRRay.direction`](https://developer.mozilla.org/en-US/docs/Web/API/XRRay/direction)
    pub fn direction(&self) -> DOMPointReadOnly {
        self.inner.get("direction").as_::<DOMPointReadOnly>()
    }

}
impl XRRay {
    /// Getter of the `matrix` attribute.
    /// [`XRRay.matrix`](https://developer.mozilla.org/en-US/docs/Web/API/XRRay/matrix)
    pub fn matrix(&self) -> Float32Array {
        self.inner.get("matrix").as_::<Float32Array>()
    }

}
