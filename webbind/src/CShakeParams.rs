use super::*;

/// The CShakeParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CShakeParams {
    inner: Any,
}

impl FromVal for CShakeParams {
    fn from_val(v: &Any) -> Self {
        CShakeParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CShakeParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CShakeParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CShakeParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CShakeParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CShakeParams> for Any {
    fn from(s: CShakeParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CShakeParams> for Any {
    fn from(s: &CShakeParams) -> Any {
        s.inner.clone()
    }
}

impl CShakeParams {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}
impl CShakeParams {
    /// Getter of the `functionName` attribute.
    pub fn function_name(&self) -> Any {
        self.inner.get("functionName").as_::<Any>()
    }

    /// Setter of the `functionName` attribute.
    pub fn set_function_name(&mut self, value: &Any) {
        self.inner.set("functionName", value);
    }
}
impl CShakeParams {
    /// Getter of the `customization` attribute.
    pub fn customization(&self) -> Any {
        self.inner.get("customization").as_::<Any>()
    }

    /// Setter of the `customization` attribute.
    pub fn set_customization(&mut self, value: &Any) {
        self.inner.set("customization", value);
    }
}
