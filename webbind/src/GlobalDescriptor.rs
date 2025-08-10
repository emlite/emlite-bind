use super::*;

/// The GlobalDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GlobalDescriptor {
    inner: Any,
}

impl FromVal for GlobalDescriptor {
    fn from_val(v: &Any) -> Self {
        GlobalDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GlobalDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GlobalDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GlobalDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GlobalDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GlobalDescriptor> for Any {
    fn from(s: GlobalDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GlobalDescriptor> for Any {
    fn from(s: &GlobalDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GlobalDescriptor {
    /// Getter of the `value` attribute.
    pub fn value(&self) -> ValueType {
        self.inner.get("value").as_::<ValueType>()
    }

    /// Setter of the `value` attribute.
    pub fn set_value(&mut self, value: &ValueType) {
        self.inner.set("value", value);
    }
}
impl GlobalDescriptor {
    /// Getter of the `mutable` attribute.
    pub fn mutable(&self) -> bool {
        self.inner.get("mutable").as_::<bool>()
    }

    /// Setter of the `mutable` attribute.
    pub fn set_mutable(&mut self, value: bool) {
        self.inner.set("mutable", value);
    }
}
