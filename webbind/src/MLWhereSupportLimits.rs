use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLWhereSupportLimits {
    inner: Any,
}
impl FromVal for MLWhereSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLWhereSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLWhereSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLWhereSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLWhereSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLWhereSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLWhereSupportLimits> for Any {
    fn from(s: MLWhereSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLWhereSupportLimits> for Any {
    fn from(s: &MLWhereSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLWhereSupportLimits {
    pub fn condition(&self) -> MLTensorLimits {
        self.inner.get("condition").as_::<MLTensorLimits>()
    }

    pub fn set_condition(&mut self, value: &MLTensorLimits) {
        self.inner.set("condition", value);
    }
}
impl MLWhereSupportLimits {
    pub fn true_value(&self) -> MLTensorLimits {
        self.inner.get("trueValue").as_::<MLTensorLimits>()
    }

    pub fn set_true_value(&mut self, value: &MLTensorLimits) {
        self.inner.set("trueValue", value);
    }
}
impl MLWhereSupportLimits {
    pub fn false_value(&self) -> MLTensorLimits {
        self.inner.get("falseValue").as_::<MLTensorLimits>()
    }

    pub fn set_false_value(&mut self, value: &MLTensorLimits) {
        self.inner.set("falseValue", value);
    }
}
impl MLWhereSupportLimits {
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
