use super::*;




/// The IIRFilterOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IIRFilterOptions {
    inner: Any,
}

impl FromVal for IIRFilterOptions {
    fn from_val(v: &Any) -> Self {
        IIRFilterOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IIRFilterOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IIRFilterOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IIRFilterOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IIRFilterOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IIRFilterOptions> for Any {
    fn from(s: IIRFilterOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IIRFilterOptions> for Any {
    fn from(s: &IIRFilterOptions) -> Any {
        s.inner.clone()
    }
}

impl IIRFilterOptions {
    /// Getter of the `feedforward` attribute.
    pub fn feedforward(&self) -> TypedArray<f64> {
        self.inner.get("feedforward").as_::<TypedArray<f64>>()
    }

    /// Setter of the `feedforward` attribute.
    pub fn set_feedforward(&mut self, value: TypedArray<f64>) {
        self.inner.set("feedforward", value);
    }
}
impl IIRFilterOptions {
    /// Getter of the `feedback` attribute.
    pub fn feedback(&self) -> TypedArray<f64> {
        self.inner.get("feedback").as_::<TypedArray<f64>>()
    }

    /// Setter of the `feedback` attribute.
    pub fn set_feedback(&mut self, value: TypedArray<f64>) {
        self.inner.set("feedback", value);
    }
}
