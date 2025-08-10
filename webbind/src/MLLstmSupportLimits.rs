use super::*;

/// The MLLstmSupportLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLLstmSupportLimits {
    inner: Any,
}

impl FromVal for MLLstmSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLLstmSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLLstmSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLLstmSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLLstmSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLLstmSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLLstmSupportLimits> for Any {
    fn from(s: MLLstmSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLLstmSupportLimits> for Any {
    fn from(s: &MLLstmSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLLstmSupportLimits {
    /// Getter of the `input` attribute.
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    /// Setter of the `input` attribute.
    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLLstmSupportLimits {
    /// Getter of the `weight` attribute.
    pub fn weight(&self) -> MLTensorLimits {
        self.inner.get("weight").as_::<MLTensorLimits>()
    }

    /// Setter of the `weight` attribute.
    pub fn set_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("weight", value);
    }
}
impl MLLstmSupportLimits {
    /// Getter of the `recurrentWeight` attribute.
    pub fn recurrent_weight(&self) -> MLTensorLimits {
        self.inner.get("recurrentWeight").as_::<MLTensorLimits>()
    }

    /// Setter of the `recurrentWeight` attribute.
    pub fn set_recurrent_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("recurrentWeight", value);
    }
}
impl MLLstmSupportLimits {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLTensorLimits {
        self.inner.get("bias").as_::<MLTensorLimits>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("bias", value);
    }
}
impl MLLstmSupportLimits {
    /// Getter of the `recurrentBias` attribute.
    pub fn recurrent_bias(&self) -> MLTensorLimits {
        self.inner.get("recurrentBias").as_::<MLTensorLimits>()
    }

    /// Setter of the `recurrentBias` attribute.
    pub fn set_recurrent_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLLstmSupportLimits {
    /// Getter of the `peepholeWeight` attribute.
    pub fn peephole_weight(&self) -> MLTensorLimits {
        self.inner.get("peepholeWeight").as_::<MLTensorLimits>()
    }

    /// Setter of the `peepholeWeight` attribute.
    pub fn set_peephole_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("peepholeWeight", value);
    }
}
impl MLLstmSupportLimits {
    /// Getter of the `initialHiddenState` attribute.
    pub fn initial_hidden_state(&self) -> MLTensorLimits {
        self.inner.get("initialHiddenState").as_::<MLTensorLimits>()
    }

    /// Setter of the `initialHiddenState` attribute.
    pub fn set_initial_hidden_state(&mut self, value: &MLTensorLimits) {
        self.inner.set("initialHiddenState", value);
    }
}
impl MLLstmSupportLimits {
    /// Getter of the `initialCellState` attribute.
    pub fn initial_cell_state(&self) -> MLTensorLimits {
        self.inner.get("initialCellState").as_::<MLTensorLimits>()
    }

    /// Setter of the `initialCellState` attribute.
    pub fn set_initial_cell_state(&mut self, value: &MLTensorLimits) {
        self.inner.set("initialCellState", value);
    }
}
impl MLLstmSupportLimits {
    /// Getter of the `outputs` attribute.
    pub fn outputs(&self) -> MLDataTypeLimits {
        self.inner.get("outputs").as_::<MLDataTypeLimits>()
    }

    /// Setter of the `outputs` attribute.
    pub fn set_outputs(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("outputs", value);
    }
}
