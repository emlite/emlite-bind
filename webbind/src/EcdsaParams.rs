use super::*;

/// The EcdsaParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EcdsaParams {
    inner: Any,
}

impl FromVal for EcdsaParams {
    fn from_val(v: &Any) -> Self {
        EcdsaParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EcdsaParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EcdsaParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EcdsaParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EcdsaParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EcdsaParams> for Any {
    fn from(s: EcdsaParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EcdsaParams> for Any {
    fn from(s: &EcdsaParams) -> Any {
        s.inner.clone()
    }
}

impl EcdsaParams {
    /// Getter of the `hash` attribute.
    pub fn hash(&self) -> Any {
        self.inner.get("hash").as_::<Any>()
    }

    /// Setter of the `hash` attribute.
    pub fn set_hash(&mut self, value: &Any) {
        self.inner.set("hash", value);
    }
}
