use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLGruSupportLimits {
    inner: Any,
}
impl FromVal for MLGruSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLGruSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLGruSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLGruSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLGruSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLGruSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLGruSupportLimits> for Any {
    fn from(s: MLGruSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLGruSupportLimits> for Any {
    fn from(s: &MLGruSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLGruSupportLimits {
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLGruSupportLimits {
    pub fn weight(&self) -> MLTensorLimits {
        self.inner.get("weight").as_::<MLTensorLimits>()
    }

    pub fn set_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("weight", value);
    }
}
impl MLGruSupportLimits {
    pub fn recurrent_weight(&self) -> MLTensorLimits {
        self.inner.get("recurrentWeight").as_::<MLTensorLimits>()
    }

    pub fn set_recurrent_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("recurrentWeight", value);
    }
}
impl MLGruSupportLimits {
    pub fn bias(&self) -> MLTensorLimits {
        self.inner.get("bias").as_::<MLTensorLimits>()
    }

    pub fn set_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("bias", value);
    }
}
impl MLGruSupportLimits {
    pub fn recurrent_bias(&self) -> MLTensorLimits {
        self.inner.get("recurrentBias").as_::<MLTensorLimits>()
    }

    pub fn set_recurrent_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLGruSupportLimits {
    pub fn initial_hidden_state(&self) -> MLTensorLimits {
        self.inner.get("initialHiddenState").as_::<MLTensorLimits>()
    }

    pub fn set_initial_hidden_state(&mut self, value: &MLTensorLimits) {
        self.inner.set("initialHiddenState", value);
    }
}
impl MLGruSupportLimits {
    pub fn outputs(&self) -> MLDataTypeLimits {
        self.inner.get("outputs").as_::<MLDataTypeLimits>()
    }

    pub fn set_outputs(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("outputs", value);
    }
}
