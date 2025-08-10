use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityProviderIcon {
    inner: Any,
}
impl FromVal for IdentityProviderIcon {
    fn from_val(v: &Any) -> Self {
        IdentityProviderIcon { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityProviderIcon {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityProviderIcon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityProviderIcon {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityProviderIcon {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityProviderIcon> for Any {
    fn from(s: IdentityProviderIcon) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityProviderIcon> for Any {
    fn from(s: &IdentityProviderIcon) -> Any {
        s.inner.clone()
    }
}

impl IdentityProviderIcon {
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl IdentityProviderIcon {
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }

    pub fn set_size(&mut self, value: u32) {
        self.inner.set("size", value);
    }
}
