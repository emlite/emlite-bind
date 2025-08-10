use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLScatterSupportLimits {
    inner: Any,
}
impl FromVal for MLScatterSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLScatterSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLScatterSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLScatterSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLScatterSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLScatterSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLScatterSupportLimits> for Any {
    fn from(s: MLScatterSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLScatterSupportLimits> for Any {
    fn from(s: &MLScatterSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLScatterSupportLimits {
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLScatterSupportLimits {
    pub fn indices(&self) -> MLTensorLimits {
        self.inner.get("indices").as_::<MLTensorLimits>()
    }

    pub fn set_indices(&mut self, value: &MLTensorLimits) {
        self.inner.set("indices", value);
    }
}
impl MLScatterSupportLimits {
    pub fn updates(&self) -> MLTensorLimits {
        self.inner.get("updates").as_::<MLTensorLimits>()
    }

    pub fn set_updates(&mut self, value: &MLTensorLimits) {
        self.inner.set("updates", value);
    }
}
impl MLScatterSupportLimits {
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
