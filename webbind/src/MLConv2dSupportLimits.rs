use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLConv2dSupportLimits {
    inner: Any,
}
impl FromVal for MLConv2dSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLConv2dSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLConv2dSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLConv2dSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLConv2dSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLConv2dSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLConv2dSupportLimits> for Any {
    fn from(s: MLConv2dSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLConv2dSupportLimits> for Any {
    fn from(s: &MLConv2dSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLConv2dSupportLimits {
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLConv2dSupportLimits {
    pub fn filter(&self) -> MLTensorLimits {
        self.inner.get("filter").as_::<MLTensorLimits>()
    }

    pub fn set_filter(&mut self, value: &MLTensorLimits) {
        self.inner.set("filter", value);
    }
}
impl MLConv2dSupportLimits {
    pub fn bias(&self) -> MLTensorLimits {
        self.inner.get("bias").as_::<MLTensorLimits>()
    }

    pub fn set_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("bias", value);
    }
}
impl MLConv2dSupportLimits {
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
