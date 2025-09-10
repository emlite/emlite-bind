use super::*;

/// The TrustedTypePolicyOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrustedTypePolicyOptions {
    inner: Any,
}

impl FromVal for TrustedTypePolicyOptions {
    fn from_val(v: &Any) -> Self {
        TrustedTypePolicyOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TrustedTypePolicyOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TrustedTypePolicyOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TrustedTypePolicyOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TrustedTypePolicyOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TrustedTypePolicyOptions> for Any {
    fn from(s: TrustedTypePolicyOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TrustedTypePolicyOptions> for Any {
    fn from(s: &TrustedTypePolicyOptions) -> Any {
        s.inner.clone()
    }
}

impl TrustedTypePolicyOptions {
    /// Getter of the `createHTML` attribute.
    pub fn create_html(&self) -> Function {
        self.inner.get("createHTML").as_::<Function>()
    }

    /// Setter of the `createHTML` attribute.
    pub fn set_create_html(&mut self, value: &Function) {
        self.inner.set("createHTML", value);
    }
}
impl TrustedTypePolicyOptions {
    /// Getter of the `createScript` attribute.
    pub fn create_script(&self) -> Function {
        self.inner.get("createScript").as_::<Function>()
    }

    /// Setter of the `createScript` attribute.
    pub fn set_create_script(&mut self, value: &Function) {
        self.inner.set("createScript", value);
    }
}
impl TrustedTypePolicyOptions {
    /// Getter of the `createScriptURL` attribute.
    pub fn create_script_url(&self) -> Function {
        self.inner.get("createScriptURL").as_::<Function>()
    }

    /// Setter of the `createScriptURL` attribute.
    pub fn set_create_script_url(&mut self, value: &Function) {
        self.inner.set("createScriptURL", value);
    }
}
