use super::*;

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
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLLstmCellSupportLimits {
    pub fn weight(&self) -> MLTensorLimits {
        self.inner.get("weight").as_::<MLTensorLimits>()
    }

    pub fn set_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("weight", value);
    }
}
impl MLLstmCellSupportLimits {
    pub fn recurrent_weight(&self) -> MLTensorLimits {
        self.inner.get("recurrentWeight").as_::<MLTensorLimits>()
    }

    pub fn set_recurrent_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("recurrentWeight", value);
    }
}
impl MLLstmCellSupportLimits {
    pub fn hidden_state(&self) -> MLTensorLimits {
        self.inner.get("hiddenState").as_::<MLTensorLimits>()
    }

    pub fn set_hidden_state(&mut self, value: &MLTensorLimits) {
        self.inner.set("hiddenState", value);
    }
}
impl MLLstmCellSupportLimits {
    pub fn cell_state(&self) -> MLTensorLimits {
        self.inner.get("cellState").as_::<MLTensorLimits>()
    }

    pub fn set_cell_state(&mut self, value: &MLTensorLimits) {
        self.inner.set("cellState", value);
    }
}
impl MLLstmCellSupportLimits {
    pub fn bias(&self) -> MLTensorLimits {
        self.inner.get("bias").as_::<MLTensorLimits>()
    }

    pub fn set_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("bias", value);
    }
}
impl MLLstmCellSupportLimits {
    pub fn recurrent_bias(&self) -> MLTensorLimits {
        self.inner.get("recurrentBias").as_::<MLTensorLimits>()
    }

    pub fn set_recurrent_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLLstmCellSupportLimits {
    pub fn peephole_weight(&self) -> MLTensorLimits {
        self.inner.get("peepholeWeight").as_::<MLTensorLimits>()
    }

    pub fn set_peephole_weight(&mut self, value: &MLTensorLimits) {
        self.inner.set("peepholeWeight", value);
    }
}
impl MLLstmCellSupportLimits {
    pub fn outputs(&self) -> MLDataTypeLimits {
        self.inner.get("outputs").as_::<MLDataTypeLimits>()
    }

    pub fn set_outputs(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("outputs", value);
    }
}
