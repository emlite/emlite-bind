use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLConcatSupportLimits {
    inner: Any,
}
impl FromVal for MLConcatSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLConcatSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLConcatSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLConcatSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLConcatSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLConcatSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLConcatSupportLimits> for Any {
    fn from(s: MLConcatSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLConcatSupportLimits> for Any {
    fn from(s: &MLConcatSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLConcatSupportLimits {
    pub fn inputs(&self) -> MLTensorLimits {
        self.inner.get("inputs").as_::<MLTensorLimits>()
    }

    pub fn set_inputs(&mut self, value: &MLTensorLimits) {
        self.inner.set("inputs", value);
    }
}
impl MLConcatSupportLimits {
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
