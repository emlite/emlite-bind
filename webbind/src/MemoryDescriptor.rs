use super::*;




/// The MemoryDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MemoryDescriptor {
    inner: Any,
}

impl FromVal for MemoryDescriptor {
    fn from_val(v: &Any) -> Self {
        MemoryDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MemoryDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MemoryDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MemoryDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MemoryDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MemoryDescriptor> for Any {
    fn from(s: MemoryDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MemoryDescriptor> for Any {
    fn from(s: &MemoryDescriptor) -> Any {
        s.inner.clone()
    }
}

impl MemoryDescriptor {
    /// Getter of the `initial` attribute.
    pub fn initial(&self) -> u32 {
        self.inner.get("initial").as_::<u32>()
    }

    /// Setter of the `initial` attribute.
    pub fn set_initial(&mut self, value: u32) {
        self.inner.set("initial", value);
    }
}
impl MemoryDescriptor {
    /// Getter of the `maximum` attribute.
    pub fn maximum(&self) -> u32 {
        self.inner.get("maximum").as_::<u32>()
    }

    /// Setter of the `maximum` attribute.
    pub fn set_maximum(&mut self, value: u32) {
        self.inner.set("maximum", value);
    }
}
