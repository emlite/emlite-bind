use super::*;




/// The GPUFragmentState dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUFragmentState {
    inner: Any,
}

impl FromVal for GPUFragmentState {
    fn from_val(v: &Any) -> Self {
        GPUFragmentState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUFragmentState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUFragmentState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUFragmentState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUFragmentState {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUFragmentState> for Any {
    fn from(s: GPUFragmentState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUFragmentState> for Any {
    fn from(s: &GPUFragmentState) -> Any {
        s.inner.clone()
    }
}

impl GPUFragmentState {
    /// Getter of the `targets` attribute.
    pub fn targets(&self) -> TypedArray<GPUColorTargetState> {
        self.inner.get("targets").as_::<TypedArray<GPUColorTargetState>>()
    }

    /// Setter of the `targets` attribute.
    pub fn set_targets(&mut self, value: &TypedArray<GPUColorTargetState>) {
        self.inner.set("targets", value);
    }
}
