use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DoubleRange {
    inner: Any,
}
impl FromVal for DoubleRange {
    fn from_val(v: &Any) -> Self {
        DoubleRange { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DoubleRange {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DoubleRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DoubleRange {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DoubleRange {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DoubleRange> for Any {
    fn from(s: DoubleRange) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DoubleRange> for Any {
    fn from(s: &DoubleRange) -> Any {
        s.inner.clone()
    }
}

impl DoubleRange {
    pub fn max(&self) -> f64 {
        self.inner.get("max").as_::<f64>()
    }

    pub fn set_max(&mut self, value: f64) {
        self.inner.set("max", value);
    }
}
impl DoubleRange {
    pub fn min(&self) -> f64 {
        self.inner.get("min").as_::<f64>()
    }

    pub fn set_min(&mut self, value: f64) {
        self.inner.set("min", value);
    }
}
