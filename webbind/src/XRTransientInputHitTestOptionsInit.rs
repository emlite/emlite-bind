use super::*;




/// The XRTransientInputHitTestOptionsInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRTransientInputHitTestOptionsInit {
    inner: Any,
}

impl FromVal for XRTransientInputHitTestOptionsInit {
    fn from_val(v: &Any) -> Self {
        XRTransientInputHitTestOptionsInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRTransientInputHitTestOptionsInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRTransientInputHitTestOptionsInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRTransientInputHitTestOptionsInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRTransientInputHitTestOptionsInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRTransientInputHitTestOptionsInit> for Any {
    fn from(s: XRTransientInputHitTestOptionsInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRTransientInputHitTestOptionsInit> for Any {
    fn from(s: &XRTransientInputHitTestOptionsInit) -> Any {
        s.inner.clone()
    }
}

impl XRTransientInputHitTestOptionsInit {
    /// Getter of the `profile` attribute.
    pub fn profile(&self) -> JsString {
        self.inner.get("profile").as_::<JsString>()
    }

    /// Setter of the `profile` attribute.
    pub fn set_profile(&mut self, value: &JsString) {
        self.inner.set("profile", value);
    }
}
impl XRTransientInputHitTestOptionsInit {
    /// Getter of the `entityTypes` attribute.
    pub fn entity_types(&self) -> TypedArray<XRHitTestTrackableType> {
        self.inner.get("entityTypes").as_::<TypedArray<XRHitTestTrackableType>>()
    }

    /// Setter of the `entityTypes` attribute.
    pub fn set_entity_types(&mut self, value: &TypedArray<XRHitTestTrackableType>) {
        self.inner.set("entityTypes", value);
    }
}
impl XRTransientInputHitTestOptionsInit {
    /// Getter of the `offsetRay` attribute.
    pub fn offset_ray(&self) -> XRRay {
        self.inner.get("offsetRay").as_::<XRRay>()
    }

    /// Setter of the `offsetRay` attribute.
    pub fn set_offset_ray(&mut self, value: &XRRay) {
        self.inner.set("offsetRay", value);
    }
}
