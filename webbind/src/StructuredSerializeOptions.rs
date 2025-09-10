use super::*;

/// The StructuredSerializeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StructuredSerializeOptions {
    inner: Any,
}

impl FromVal for StructuredSerializeOptions {
    fn from_val(v: &Any) -> Self {
        StructuredSerializeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for StructuredSerializeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StructuredSerializeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StructuredSerializeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StructuredSerializeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<StructuredSerializeOptions> for Any {
    fn from(s: StructuredSerializeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StructuredSerializeOptions> for Any {
    fn from(s: &StructuredSerializeOptions) -> Any {
        s.inner.clone()
    }
}

impl StructuredSerializeOptions {
    /// Getter of the `transfer` attribute.
    pub fn transfer(&self) -> TypedArray<Object> {
        self.inner.get("transfer").as_::<TypedArray<Object>>()
    }

    /// Setter of the `transfer` attribute.
    pub fn set_transfer(&mut self, value: &TypedArray<Object>) {
        self.inner.set("transfer", value);
    }
}
