use super::*;




/// The LockManagerSnapshot dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LockManagerSnapshot {
    inner: Any,
}

impl FromVal for LockManagerSnapshot {
    fn from_val(v: &Any) -> Self {
        LockManagerSnapshot { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for LockManagerSnapshot {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LockManagerSnapshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LockManagerSnapshot {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LockManagerSnapshot {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<LockManagerSnapshot> for Any {
    fn from(s: LockManagerSnapshot) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LockManagerSnapshot> for Any {
    fn from(s: &LockManagerSnapshot) -> Any {
        s.inner.clone()
    }
}

impl LockManagerSnapshot {
    /// Getter of the `held` attribute.
    pub fn held(&self) -> TypedArray<LockInfo> {
        self.inner.get("held").as_::<TypedArray<LockInfo>>()
    }

    /// Setter of the `held` attribute.
    pub fn set_held(&mut self, value: &TypedArray<LockInfo>) {
        self.inner.set("held", value);
    }
}
impl LockManagerSnapshot {
    /// Getter of the `pending` attribute.
    pub fn pending(&self) -> TypedArray<LockInfo> {
        self.inner.get("pending").as_::<TypedArray<LockInfo>>()
    }

    /// Setter of the `pending` attribute.
    pub fn set_pending(&mut self, value: &TypedArray<LockInfo>) {
        self.inner.set("pending", value);
    }
}
