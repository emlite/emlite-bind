use super::*;




/// The XRRigidTransform class.
/// [`XRRigidTransform`](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRRigidTransform {
    inner: Any,
}

impl FromVal for XRRigidTransform {
    fn from_val(v: &Any) -> Self {
        XRRigidTransform { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRRigidTransform {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRRigidTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRRigidTransform {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRRigidTransform {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRRigidTransform> for Any {
    fn from(s: XRRigidTransform) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRRigidTransform> for Any {
    fn from(s: &XRRigidTransform) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRRigidTransform);



impl XRRigidTransform {
    /// The `new XRRigidTransform(..)` constructor, creating a new XRRigidTransform instance
    pub fn new0() -> XRRigidTransform {
        Self {
            inner: Any::global("XRRigidTransform").new(&[]).as_::<Any>(),
        }
    }

    /// The `new XRRigidTransform(..)` constructor, creating a new XRRigidTransform instance
    pub fn new1(position: &DOMPointInit) -> XRRigidTransform {
        Self {
            inner: Any::global("XRRigidTransform").new(&[position.into()]).as_::<Any>(),
        }
    }

    /// The `new XRRigidTransform(..)` constructor, creating a new XRRigidTransform instance
    pub fn new2(position: &DOMPointInit, orientation: &DOMPointInit) -> XRRigidTransform {
        Self {
            inner: Any::global("XRRigidTransform").new(&[position.into(), orientation.into()]).as_::<Any>(),
        }
    }

}
impl XRRigidTransform {
    /// Getter of the `position` attribute.
    /// [`XRRigidTransform.position`](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform/position)
    pub fn position(&self) -> DOMPointReadOnly {
        self.inner.get("position").as_::<DOMPointReadOnly>()
    }

}
impl XRRigidTransform {
    /// Getter of the `orientation` attribute.
    /// [`XRRigidTransform.orientation`](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform/orientation)
    pub fn orientation(&self) -> DOMPointReadOnly {
        self.inner.get("orientation").as_::<DOMPointReadOnly>()
    }

}
impl XRRigidTransform {
    /// Getter of the `matrix` attribute.
    /// [`XRRigidTransform.matrix`](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform/matrix)
    pub fn matrix(&self) -> Float32Array {
        self.inner.get("matrix").as_::<Float32Array>()
    }

}
impl XRRigidTransform {
    /// Getter of the `inverse` attribute.
    /// [`XRRigidTransform.inverse`](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform/inverse)
    pub fn inverse(&self) -> XRRigidTransform {
        self.inner.get("inverse").as_::<XRRigidTransform>()
    }

}
