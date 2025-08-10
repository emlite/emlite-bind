use super::*;

/// The MLLstmCellOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLLstmCellOptions {
    inner: Any,
}

impl FromVal for MLLstmCellOptions {
    fn from_val(v: &Any) -> Self {
        MLLstmCellOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLLstmCellOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLLstmCellOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLLstmCellOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLLstmCellOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLLstmCellOptions> for Any {
    fn from(s: MLLstmCellOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLLstmCellOptions> for Any {
    fn from(s: &MLLstmCellOptions) -> Any {
        s.inner.clone()
    }
}

impl MLLstmCellOptions {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLLstmCellOptions {
    /// Getter of the `recurrentBias` attribute.
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    /// Setter of the `recurrentBias` attribute.
    pub fn set_recurrent_bias(&mut self, value: &MLOperand) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLLstmCellOptions {
    /// Getter of the `peepholeWeight` attribute.
    pub fn peephole_weight(&self) -> MLOperand {
        self.inner.get("peepholeWeight").as_::<MLOperand>()
    }

    /// Setter of the `peepholeWeight` attribute.
    pub fn set_peephole_weight(&mut self, value: &MLOperand) {
        self.inner.set("peepholeWeight", value);
    }
}
impl MLLstmCellOptions {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> MLLstmWeightLayout {
        self.inner.get("layout").as_::<MLLstmWeightLayout>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &MLLstmWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLLstmCellOptions {
    /// Getter of the `activations` attribute.
    pub fn activations(&self) -> TypedArray<MLRecurrentNetworkActivation> {
        self.inner
            .get("activations")
            .as_::<TypedArray<MLRecurrentNetworkActivation>>()
    }

    /// Setter of the `activations` attribute.
    pub fn set_activations(&mut self, value: &TypedArray<MLRecurrentNetworkActivation>) {
        self.inner.set("activations", value);
    }
}
