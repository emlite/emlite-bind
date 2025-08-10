use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLBatchNormalizationSupportLimits {
    inner: Any,
}
impl FromVal for MLBatchNormalizationSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLBatchNormalizationSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLBatchNormalizationSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLBatchNormalizationSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLBatchNormalizationSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLBatchNormalizationSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLBatchNormalizationSupportLimits> for Any {
    fn from(s: MLBatchNormalizationSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLBatchNormalizationSupportLimits> for Any {
    fn from(s: &MLBatchNormalizationSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLBatchNormalizationSupportLimits {
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLBatchNormalizationSupportLimits {
    pub fn mean(&self) -> MLTensorLimits {
        self.inner.get("mean").as_::<MLTensorLimits>()
    }

    pub fn set_mean(&mut self, value: &MLTensorLimits) {
        self.inner.set("mean", value);
    }
}
impl MLBatchNormalizationSupportLimits {
    pub fn variance(&self) -> MLTensorLimits {
        self.inner.get("variance").as_::<MLTensorLimits>()
    }

    pub fn set_variance(&mut self, value: &MLTensorLimits) {
        self.inner.set("variance", value);
    }
}
impl MLBatchNormalizationSupportLimits {
    pub fn scale(&self) -> MLTensorLimits {
        self.inner.get("scale").as_::<MLTensorLimits>()
    }

    pub fn set_scale(&mut self, value: &MLTensorLimits) {
        self.inner.set("scale", value);
    }
}
impl MLBatchNormalizationSupportLimits {
    pub fn bias(&self) -> MLTensorLimits {
        self.inner.get("bias").as_::<MLTensorLimits>()
    }

    pub fn set_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("bias", value);
    }
}
impl MLBatchNormalizationSupportLimits {
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
