use super::*;




/// The ConstrainDoubleRange dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConstrainDoubleRange {
    inner: Any,
}

impl FromVal for ConstrainDoubleRange {
    fn from_val(v: &Any) -> Self {
        ConstrainDoubleRange { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ConstrainDoubleRange {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ConstrainDoubleRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ConstrainDoubleRange {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ConstrainDoubleRange {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ConstrainDoubleRange> for Any {
    fn from(s: ConstrainDoubleRange) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ConstrainDoubleRange> for Any {
    fn from(s: &ConstrainDoubleRange) -> Any {
        s.inner.clone()
    }
}

impl ConstrainDoubleRange {
    /// Getter of the `exact` attribute.
    pub fn exact(&self) -> f64 {
        self.inner.get("exact").as_::<f64>()
    }

    /// Setter of the `exact` attribute.
    pub fn set_exact(&mut self, value: f64) {
        self.inner.set("exact", value);
    }
}
impl ConstrainDoubleRange {
    /// Getter of the `ideal` attribute.
    pub fn ideal(&self) -> f64 {
        self.inner.get("ideal").as_::<f64>()
    }

    /// Setter of the `ideal` attribute.
    pub fn set_ideal(&mut self, value: f64) {
        self.inner.set("ideal", value);
    }
}
