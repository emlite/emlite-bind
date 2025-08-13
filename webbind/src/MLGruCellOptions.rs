use super::*;




/// The MLGruCellOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLGruCellOptions {
    inner: Any,
}

impl FromVal for MLGruCellOptions {
    fn from_val(v: &Any) -> Self {
        MLGruCellOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLGruCellOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLGruCellOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLGruCellOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLGruCellOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLGruCellOptions> for Any {
    fn from(s: MLGruCellOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLGruCellOptions> for Any {
    fn from(s: &MLGruCellOptions) -> Any {
        s.inner.clone()
    }
}

impl MLGruCellOptions {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLGruCellOptions {
    /// Getter of the `recurrentBias` attribute.
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    /// Setter of the `recurrentBias` attribute.
    pub fn set_recurrent_bias(&mut self, value: &MLOperand) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLGruCellOptions {
    /// Getter of the `resetAfter` attribute.
    pub fn reset_after(&self) -> bool {
        self.inner.get("resetAfter").as_::<bool>()
    }

    /// Setter of the `resetAfter` attribute.
    pub fn set_reset_after(&mut self, value: bool) {
        self.inner.set("resetAfter", value);
    }
}
impl MLGruCellOptions {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> MLGruWeightLayout {
        self.inner.get("layout").as_::<MLGruWeightLayout>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &MLGruWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLGruCellOptions {
    /// Getter of the `activations` attribute.
    pub fn activations(&self) -> TypedArray<MLRecurrentNetworkActivation> {
        self.inner.get("activations").as_::<TypedArray<MLRecurrentNetworkActivation>>()
    }

    /// Setter of the `activations` attribute.
    pub fn set_activations(&mut self, value: &TypedArray<MLRecurrentNetworkActivation>) {
        self.inner.set("activations", value);
    }
}
