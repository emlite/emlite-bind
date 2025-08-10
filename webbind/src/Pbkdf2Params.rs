use super::*;

/// The Pbkdf2Params dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Pbkdf2Params {
    inner: Any,
}

impl FromVal for Pbkdf2Params {
    fn from_val(v: &Any) -> Self {
        Pbkdf2Params { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Pbkdf2Params {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Pbkdf2Params {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Pbkdf2Params {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Pbkdf2Params {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Pbkdf2Params> for Any {
    fn from(s: Pbkdf2Params) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Pbkdf2Params> for Any {
    fn from(s: &Pbkdf2Params) -> Any {
        s.inner.clone()
    }
}

impl Pbkdf2Params {
    /// Getter of the `salt` attribute.
    pub fn salt(&self) -> Any {
        self.inner.get("salt").as_::<Any>()
    }

    /// Setter of the `salt` attribute.
    pub fn set_salt(&mut self, value: &Any) {
        self.inner.set("salt", value);
    }
}
impl Pbkdf2Params {
    /// Getter of the `iterations` attribute.
    pub fn iterations(&self) -> u32 {
        self.inner.get("iterations").as_::<u32>()
    }

    /// Setter of the `iterations` attribute.
    pub fn set_iterations(&mut self, value: u32) {
        self.inner.set("iterations", value);
    }
}
impl Pbkdf2Params {
    /// Getter of the `hash` attribute.
    pub fn hash(&self) -> Any {
        self.inner.get("hash").as_::<Any>()
    }

    /// Setter of the `hash` attribute.
    pub fn set_hash(&mut self, value: &Any) {
        self.inner.set("hash", value);
    }
}
