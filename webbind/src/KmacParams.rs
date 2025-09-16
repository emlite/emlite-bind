use super::*;

/// The KmacParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KmacParams {
    inner: Any,
}

impl FromVal for KmacParams {
    fn from_val(v: &Any) -> Self {
        KmacParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for KmacParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for KmacParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for KmacParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for KmacParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<KmacParams> for Any {
    fn from(s: KmacParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&KmacParams> for Any {
    fn from(s: &KmacParams) -> Any {
        s.inner.clone()
    }
}

impl KmacParams {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}
impl KmacParams {
    /// Getter of the `customization` attribute.
    pub fn customization(&self) -> Any {
        self.inner.get("customization").as_::<Any>()
    }

    /// Setter of the `customization` attribute.
    pub fn set_customization(&mut self, value: &Any) {
        self.inner.set("customization", value);
    }
}
