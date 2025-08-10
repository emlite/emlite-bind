use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SanitizerElementNamespaceWithAttributes {
    inner: Any,
}
impl FromVal for SanitizerElementNamespaceWithAttributes {
    fn from_val(v: &Any) -> Self {
        SanitizerElementNamespaceWithAttributes { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SanitizerElementNamespaceWithAttributes {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SanitizerElementNamespaceWithAttributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SanitizerElementNamespaceWithAttributes {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SanitizerElementNamespaceWithAttributes {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SanitizerElementNamespaceWithAttributes> for Any {
    fn from(s: SanitizerElementNamespaceWithAttributes) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SanitizerElementNamespaceWithAttributes> for Any {
    fn from(s: &SanitizerElementNamespaceWithAttributes) -> Any {
        s.inner.clone()
    }
}

impl SanitizerElementNamespaceWithAttributes {
    pub fn attributes(&self) -> TypedArray<Any> {
        self.inner.get("attributes").as_::<TypedArray<Any>>()
    }

    pub fn set_attributes(&mut self, value: &TypedArray<Any>) {
        self.inner.set("attributes", value);
    }
}
impl SanitizerElementNamespaceWithAttributes {
    pub fn remove_attributes(&self) -> TypedArray<Any> {
        self.inner.get("removeAttributes").as_::<TypedArray<Any>>()
    }

    pub fn set_remove_attributes(&mut self, value: &TypedArray<Any>) {
        self.inner.set("removeAttributes", value);
    }
}
