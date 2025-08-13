use super::*;




/// The DetectedFace dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DetectedFace {
    inner: Any,
}

impl FromVal for DetectedFace {
    fn from_val(v: &Any) -> Self {
        DetectedFace { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DetectedFace {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DetectedFace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DetectedFace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DetectedFace {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DetectedFace> for Any {
    fn from(s: DetectedFace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DetectedFace> for Any {
    fn from(s: &DetectedFace) -> Any {
        s.inner.clone()
    }
}

impl DetectedFace {
    /// Getter of the `boundingBox` attribute.
    pub fn bounding_box(&self) -> DOMRectReadOnly {
        self.inner.get("boundingBox").as_::<DOMRectReadOnly>()
    }

    /// Setter of the `boundingBox` attribute.
    pub fn set_bounding_box(&mut self, value: &DOMRectReadOnly) {
        self.inner.set("boundingBox", value);
    }
}
impl DetectedFace {
    /// Getter of the `landmarks` attribute.
    pub fn landmarks(&self) -> TypedArray<Landmark> {
        self.inner.get("landmarks").as_::<TypedArray<Landmark>>()
    }

    /// Setter of the `landmarks` attribute.
    pub fn set_landmarks(&mut self, value: &TypedArray<Landmark>) {
        self.inner.set("landmarks", value);
    }
}
