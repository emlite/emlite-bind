use super::*;

/// The Argon2Params dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Argon2Params {
    inner: Any,
}

impl FromVal for Argon2Params {
    fn from_val(v: &Any) -> Self {
        Argon2Params { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Argon2Params {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Argon2Params {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Argon2Params {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Argon2Params {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Argon2Params> for Any {
    fn from(s: Argon2Params) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Argon2Params> for Any {
    fn from(s: &Argon2Params) -> Any {
        s.inner.clone()
    }
}

impl Argon2Params {
    /// Getter of the `nonce` attribute.
    pub fn nonce(&self) -> Any {
        self.inner.get("nonce").as_::<Any>()
    }

    /// Setter of the `nonce` attribute.
    pub fn set_nonce(&mut self, value: &Any) {
        self.inner.set("nonce", value);
    }
}
impl Argon2Params {
    /// Getter of the `parallelism` attribute.
    pub fn parallelism(&self) -> u32 {
        self.inner.get("parallelism").as_::<u32>()
    }

    /// Setter of the `parallelism` attribute.
    pub fn set_parallelism(&mut self, value: u32) {
        self.inner.set("parallelism", value);
    }
}
impl Argon2Params {
    /// Getter of the `memory` attribute.
    pub fn memory(&self) -> u32 {
        self.inner.get("memory").as_::<u32>()
    }

    /// Setter of the `memory` attribute.
    pub fn set_memory(&mut self, value: u32) {
        self.inner.set("memory", value);
    }
}
impl Argon2Params {
    /// Getter of the `passes` attribute.
    pub fn passes(&self) -> u32 {
        self.inner.get("passes").as_::<u32>()
    }

    /// Setter of the `passes` attribute.
    pub fn set_passes(&mut self, value: u32) {
        self.inner.set("passes", value);
    }
}
impl Argon2Params {
    /// Getter of the `version` attribute.
    pub fn version(&self) -> u8 {
        self.inner.get("version").as_::<u8>()
    }

    /// Setter of the `version` attribute.
    pub fn set_version(&mut self, value: u8) {
        self.inner.set("version", value);
    }
}
impl Argon2Params {
    /// Getter of the `secretValue` attribute.
    pub fn secret_value(&self) -> Any {
        self.inner.get("secretValue").as_::<Any>()
    }

    /// Setter of the `secretValue` attribute.
    pub fn set_secret_value(&mut self, value: &Any) {
        self.inner.set("secretValue", value);
    }
}
impl Argon2Params {
    /// Getter of the `associatedData` attribute.
    pub fn associated_data(&self) -> Any {
        self.inner.get("associatedData").as_::<Any>()
    }

    /// Setter of the `associatedData` attribute.
    pub fn set_associated_data(&mut self, value: &Any) {
        self.inner.set("associatedData", value);
    }
}
