use super::*;




/// The MediaTrackConstraints dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaTrackConstraints {
    inner: Any,
}

impl FromVal for MediaTrackConstraints {
    fn from_val(v: &Any) -> Self {
        MediaTrackConstraints { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaTrackConstraints {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaTrackConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaTrackConstraints {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaTrackConstraints {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaTrackConstraints> for Any {
    fn from(s: MediaTrackConstraints) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaTrackConstraints> for Any {
    fn from(s: &MediaTrackConstraints) -> Any {
        s.inner.clone()
    }
}

impl MediaTrackConstraints {
    /// Getter of the `advanced` attribute.
    pub fn advanced(&self) -> TypedArray<MediaTrackConstraintSet> {
        self.inner.get("advanced").as_::<TypedArray<MediaTrackConstraintSet>>()
    }

    /// Setter of the `advanced` attribute.
    pub fn set_advanced(&mut self, value: &TypedArray<MediaTrackConstraintSet>) {
        self.inner.set("advanced", value);
    }
}
