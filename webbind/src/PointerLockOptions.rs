use super::*;

/// The PointerLockOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PointerLockOptions {
    inner: Any,
}

impl FromVal for PointerLockOptions {
    fn from_val(v: &Any) -> Self {
        PointerLockOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PointerLockOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PointerLockOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PointerLockOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PointerLockOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PointerLockOptions> for Any {
    fn from(s: PointerLockOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PointerLockOptions> for Any {
    fn from(s: &PointerLockOptions) -> Any {
        s.inner.clone()
    }
}

impl PointerLockOptions {
    /// Getter of the `unadjustedMovement` attribute.
    pub fn unadjusted_movement(&self) -> bool {
        self.inner.get("unadjustedMovement").as_::<bool>()
    }

    /// Setter of the `unadjustedMovement` attribute.
    pub fn set_unadjusted_movement(&mut self, value: bool) {
        self.inner.set("unadjustedMovement", value);
    }
}
