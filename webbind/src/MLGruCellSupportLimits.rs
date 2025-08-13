use super::*;




/// The MLGruCellSupportLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLGruCellSupportLimits {
    inner: Any,
}

impl FromVal for MLGruCellSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLGruCellSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLGruCellSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLGruCellSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLGruCellSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLGruCellSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLGruCellSupportLimits> for Any {
    fn from(s: MLGruCellSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLGruCellSupportLimits> for Any {
    fn from(s: &MLGruCellSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLGruCellSupportLimits {
    /// Getter of the `input` attribute.
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    /// Setter of the `input` attribute.
    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLGruCellSupportLimits {
    /// Getter of the `weight` attribute.
    pub fn weight(&self) -> MLTensorLimits {
        self.inner.get("weight").as_::<MLTensorLimits>()
    }

    /// Setter of the `weight` attribute.
    pub fn set_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("weight", value);
    }
}
impl MLGruCellSupportLimits {
    /// Getter of the `recurrentWeight` attribute.
    pub fn recurrent_weight(&self) -> MLTensorLimits {
        self.inner.get("recurrentWeight").as_::<MLTensorLimits>()
    }

    /// Setter of the `recurrentWeight` attribute.
    pub fn set_recurrent_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("recurrentWeight", value);
    }
}
impl MLGruCellSupportLimits {
    /// Getter of the `hiddenState` attribute.
    pub fn hidden_state(&self) -> MLTensorLimits {
        self.inner.get("hiddenState").as_::<MLTensorLimits>()
    }

    /// Setter of the `hiddenState` attribute.
    pub fn set_hidden_state(&mut self, value: &MLTensorLimits) {
        self.inner.set("hiddenState", value);
    }
}
impl MLGruCellSupportLimits {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLTensorLimits {
        self.inner.get("bias").as_::<MLTensorLimits>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("bias", value);
    }
}
impl MLGruCellSupportLimits {
    /// Getter of the `recurrentBias` attribute.
    pub fn recurrent_bias(&self) -> MLTensorLimits {
        self.inner.get("recurrentBias").as_::<MLTensorLimits>()
    }

    /// Setter of the `recurrentBias` attribute.
    pub fn set_recurrent_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLGruCellSupportLimits {
    /// Getter of the `output` attribute.
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    /// Setter of the `output` attribute.
    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
