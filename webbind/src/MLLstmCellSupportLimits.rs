use super::*;




/// The MLLstmCellSupportLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLLstmCellSupportLimits {
    inner: Any,
}

impl FromVal for MLLstmCellSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLLstmCellSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLLstmCellSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLLstmCellSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLLstmCellSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLLstmCellSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLLstmCellSupportLimits> for Any {
    fn from(s: MLLstmCellSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLLstmCellSupportLimits> for Any {
    fn from(s: &MLLstmCellSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLLstmCellSupportLimits {
    /// Getter of the `input` attribute.
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    /// Setter of the `input` attribute.
    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLLstmCellSupportLimits {
    /// Getter of the `weight` attribute.
    pub fn weight(&self) -> MLTensorLimits {
        self.inner.get("weight").as_::<MLTensorLimits>()
    }

    /// Setter of the `weight` attribute.
    pub fn set_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("weight", value);
    }
}
impl MLLstmCellSupportLimits {
    /// Getter of the `recurrentWeight` attribute.
    pub fn recurrent_weight(&self) -> MLTensorLimits {
        self.inner.get("recurrentWeight").as_::<MLTensorLimits>()
    }

    /// Setter of the `recurrentWeight` attribute.
    pub fn set_recurrent_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("recurrentWeight", value);
    }
}
impl MLLstmCellSupportLimits {
    /// Getter of the `hiddenState` attribute.
    pub fn hidden_state(&self) -> MLTensorLimits {
        self.inner.get("hiddenState").as_::<MLTensorLimits>()
    }

    /// Setter of the `hiddenState` attribute.
    pub fn set_hidden_state(&mut self, value: &MLTensorLimits) {
        self.inner.set("hiddenState", value);
    }
}
impl MLLstmCellSupportLimits {
    /// Getter of the `cellState` attribute.
    pub fn cell_state(&self) -> MLTensorLimits {
        self.inner.get("cellState").as_::<MLTensorLimits>()
    }

    /// Setter of the `cellState` attribute.
    pub fn set_cell_state(&mut self, value: &MLTensorLimits) {
        self.inner.set("cellState", value);
    }
}
impl MLLstmCellSupportLimits {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLTensorLimits {
        self.inner.get("bias").as_::<MLTensorLimits>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("bias", value);
    }
}
impl MLLstmCellSupportLimits {
    /// Getter of the `recurrentBias` attribute.
    pub fn recurrent_bias(&self) -> MLTensorLimits {
        self.inner.get("recurrentBias").as_::<MLTensorLimits>()
    }

    /// Setter of the `recurrentBias` attribute.
    pub fn set_recurrent_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLLstmCellSupportLimits {
    /// Getter of the `peepholeWeight` attribute.
    pub fn peephole_weight(&self) -> MLTensorLimits {
        self.inner.get("peepholeWeight").as_::<MLTensorLimits>()
    }

    /// Setter of the `peepholeWeight` attribute.
    pub fn set_peephole_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("peepholeWeight", value);
    }
}
impl MLLstmCellSupportLimits {
    /// Getter of the `outputs` attribute.
    pub fn outputs(&self) -> MLDataTypeLimits {
        self.inner.get("outputs").as_::<MLDataTypeLimits>()
    }

    /// Setter of the `outputs` attribute.
    pub fn set_outputs(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("outputs", value);
    }
}
