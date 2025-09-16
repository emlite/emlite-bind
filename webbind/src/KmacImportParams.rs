use super::*;

/// The KmacImportParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KmacImportParams {
    inner: Any,
}

impl FromVal for KmacImportParams {
    fn from_val(v: &Any) -> Self {
        KmacImportParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for KmacImportParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for KmacImportParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for KmacImportParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for KmacImportParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<KmacImportParams> for Any {
    fn from(s: KmacImportParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&KmacImportParams> for Any {
    fn from(s: &KmacImportParams) -> Any {
        s.inner.clone()
    }
}

impl KmacImportParams {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}
