use super::*;

/// The MLGruOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLGruOptions {
    inner: Any,
}

impl FromVal for MLGruOptions {
    fn from_val(v: &Any) -> Self {
        MLGruOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLGruOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLGruOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLGruOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLGruOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLGruOptions> for Any {
    fn from(s: MLGruOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLGruOptions> for Any {
    fn from(s: &MLGruOptions) -> Any {
        s.inner.clone()
    }
}

impl MLGruOptions {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLGruOptions {
    /// Getter of the `recurrentBias` attribute.
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    /// Setter of the `recurrentBias` attribute.
    pub fn set_recurrent_bias(&mut self, value: &MLOperand) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLGruOptions {
    /// Getter of the `initialHiddenState` attribute.
    pub fn initial_hidden_state(&self) -> MLOperand {
        self.inner.get("initialHiddenState").as_::<MLOperand>()
    }

    /// Setter of the `initialHiddenState` attribute.
    pub fn set_initial_hidden_state(&mut self, value: &MLOperand) {
        self.inner.set("initialHiddenState", value);
    }
}
impl MLGruOptions {
    /// Getter of the `resetAfter` attribute.
    pub fn reset_after(&self) -> bool {
        self.inner.get("resetAfter").as_::<bool>()
    }

    /// Setter of the `resetAfter` attribute.
    pub fn set_reset_after(&mut self, value: bool) {
        self.inner.set("resetAfter", value);
    }
}
impl MLGruOptions {
    /// Getter of the `returnSequence` attribute.
    pub fn return_sequence(&self) -> bool {
        self.inner.get("returnSequence").as_::<bool>()
    }

    /// Setter of the `returnSequence` attribute.
    pub fn set_return_sequence(&mut self, value: bool) {
        self.inner.set("returnSequence", value);
    }
}
impl MLGruOptions {
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
impl MLGruOptions {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> MLGruWeightLayout {
        self.inner.get("layout").as_::<MLGruWeightLayout>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &MLGruWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLGruOptions {
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
