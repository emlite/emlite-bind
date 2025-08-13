use super::*;




/// The ProfilerTrace dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProfilerTrace {
    inner: Any,
}

impl FromVal for ProfilerTrace {
    fn from_val(v: &Any) -> Self {
        ProfilerTrace { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ProfilerTrace {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ProfilerTrace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ProfilerTrace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ProfilerTrace {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ProfilerTrace> for Any {
    fn from(s: ProfilerTrace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ProfilerTrace> for Any {
    fn from(s: &ProfilerTrace) -> Any {
        s.inner.clone()
    }
}

impl ProfilerTrace {
    /// Getter of the `resources` attribute.
    pub fn resources(&self) -> TypedArray<Any> {
        self.inner.get("resources").as_::<TypedArray<Any>>()
    }

    /// Setter of the `resources` attribute.
    pub fn set_resources(&mut self, value: &TypedArray<Any>) {
        self.inner.set("resources", value);
    }
}
impl ProfilerTrace {
    /// Getter of the `frames` attribute.
    pub fn frames(&self) -> TypedArray<ProfilerFrame> {
        self.inner.get("frames").as_::<TypedArray<ProfilerFrame>>()
    }

    /// Setter of the `frames` attribute.
    pub fn set_frames(&mut self, value: &TypedArray<ProfilerFrame>) {
        self.inner.set("frames", value);
    }
}
impl ProfilerTrace {
    /// Getter of the `stacks` attribute.
    pub fn stacks(&self) -> TypedArray<ProfilerStack> {
        self.inner.get("stacks").as_::<TypedArray<ProfilerStack>>()
    }

    /// Setter of the `stacks` attribute.
    pub fn set_stacks(&mut self, value: &TypedArray<ProfilerStack>) {
        self.inner.set("stacks", value);
    }
}
impl ProfilerTrace {
    /// Getter of the `samples` attribute.
    pub fn samples(&self) -> TypedArray<ProfilerSample> {
        self.inner.get("samples").as_::<TypedArray<ProfilerSample>>()
    }

    /// Setter of the `samples` attribute.
    pub fn set_samples(&mut self, value: &TypedArray<ProfilerSample>) {
        self.inner.set("samples", value);
    }
}
