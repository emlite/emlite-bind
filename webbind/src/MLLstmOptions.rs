use super::*;

/// The MLLstmOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLLstmOptions {
    inner: Any,
}

impl FromVal for MLLstmOptions {
    fn from_val(v: &Any) -> Self {
        MLLstmOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLLstmOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLLstmOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLLstmOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLLstmOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLLstmOptions> for Any {
    fn from(s: MLLstmOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLLstmOptions> for Any {
    fn from(s: &MLLstmOptions) -> Any {
        s.inner.clone()
    }
}

impl MLLstmOptions {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLLstmOptions {
    /// Getter of the `recurrentBias` attribute.
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    /// Setter of the `recurrentBias` attribute.
    pub fn set_recurrent_bias(&mut self, value: &MLOperand) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLLstmOptions {
    /// Getter of the `peepholeWeight` attribute.
    pub fn peephole_weight(&self) -> MLOperand {
        self.inner.get("peepholeWeight").as_::<MLOperand>()
    }

    /// Setter of the `peepholeWeight` attribute.
    pub fn set_peephole_weight(&mut self, value: &MLOperand) {
        self.inner.set("peepholeWeight", value);
    }
}
impl MLLstmOptions {
    /// Getter of the `initialHiddenState` attribute.
    pub fn initial_hidden_state(&self) -> MLOperand {
        self.inner.get("initialHiddenState").as_::<MLOperand>()
    }

    /// Setter of the `initialHiddenState` attribute.
    pub fn set_initial_hidden_state(&mut self, value: &MLOperand) {
        self.inner.set("initialHiddenState", value);
    }
}
impl MLLstmOptions {
    /// Getter of the `initialCellState` attribute.
    pub fn initial_cell_state(&self) -> MLOperand {
        self.inner.get("initialCellState").as_::<MLOperand>()
    }

    /// Setter of the `initialCellState` attribute.
    pub fn set_initial_cell_state(&mut self, value: &MLOperand) {
        self.inner.set("initialCellState", value);
    }
}
impl MLLstmOptions {
    /// Getter of the `returnSequence` attribute.
    pub fn return_sequence(&self) -> bool {
        self.inner.get("returnSequence").as_::<bool>()
    }

    /// Setter of the `returnSequence` attribute.
    pub fn set_return_sequence(&mut self, value: bool) {
        self.inner.set("returnSequence", value);
    }
}
impl MLLstmOptions {
    /// Getter of the `direction` attribute.
    pub fn direction(&self) -> MLRecurrentNetworkDirection {
        self.inner
            .get("direction")
            .as_::<MLRecurrentNetworkDirection>()
    }

    /// Setter of the `direction` attribute.
    pub fn set_direction(&mut self, value: &MLRecurrentNetworkDirection) {
        self.inner.set("direction", value);
    }
}
impl MLLstmOptions {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> MLLstmWeightLayout {
        self.inner.get("layout").as_::<MLLstmWeightLayout>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &MLLstmWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLLstmOptions {
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
