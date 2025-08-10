use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLBinarySupportLimits {
    inner: Any,
}
impl FromVal for MLBinarySupportLimits {
    fn from_val(v: &Any) -> Self {
        MLBinarySupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLBinarySupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLBinarySupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLBinarySupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLBinarySupportLimits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLBinarySupportLimits> for Any {
    fn from(s: MLBinarySupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLBinarySupportLimits> for Any {
    fn from(s: &MLBinarySupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLBinarySupportLimits {
    pub fn a(&self) -> MLTensorLimits {
        self.inner.get("a").as_::<MLTensorLimits>()
    }

    pub fn set_a(&mut self, value: &MLTensorLimits) {
        self.inner.set("a", value);
    }
}
impl MLBinarySupportLimits {
    pub fn b(&self) -> MLTensorLimits {
        self.inner.get("b").as_::<MLTensorLimits>()
    }

    pub fn set_b(&mut self, value: &MLTensorLimits) {
        self.inner.set("b", value);
    }
}
impl MLBinarySupportLimits {
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
