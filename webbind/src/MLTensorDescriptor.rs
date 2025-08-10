use super::*;

/// The MLTensorDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLTensorDescriptor {
    inner: Any,
}

impl FromVal for MLTensorDescriptor {
    fn from_val(v: &Any) -> Self {
        MLTensorDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLTensorDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLTensorDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLTensorDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLTensorDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLTensorDescriptor> for Any {
    fn from(s: MLTensorDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLTensorDescriptor> for Any {
    fn from(s: &MLTensorDescriptor) -> Any {
        s.inner.clone()
    }
}

impl MLTensorDescriptor {
    /// Getter of the `readable` attribute.
    pub fn readable(&self) -> bool {
        self.inner.get("readable").as_::<bool>()
    }

    /// Setter of the `readable` attribute.
    pub fn set_readable(&mut self, value: bool) {
        self.inner.set("readable", value);
    }
}
impl MLTensorDescriptor {
    /// Getter of the `writable` attribute.
    pub fn writable(&self) -> bool {
        self.inner.get("writable").as_::<bool>()
    }

    /// Setter of the `writable` attribute.
    pub fn set_writable(&mut self, value: bool) {
        self.inner.set("writable", value);
    }
}
