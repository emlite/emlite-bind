use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityProviderAccountList {
    inner: Any,
}
impl FromVal for IdentityProviderAccountList {
    fn from_val(v: &Any) -> Self {
        IdentityProviderAccountList { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityProviderAccountList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityProviderAccountList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityProviderAccountList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityProviderAccountList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityProviderAccountList> for Any {
    fn from(s: IdentityProviderAccountList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityProviderAccountList> for Any {
    fn from(s: &IdentityProviderAccountList) -> Any {
        s.inner.clone()
    }
}

impl IdentityProviderAccountList {
    pub fn accounts(&self) -> TypedArray<IdentityProviderAccount> {
        self.inner
            .get("accounts")
            .as_::<TypedArray<IdentityProviderAccount>>()
    }

    pub fn set_accounts(&mut self, value: &TypedArray<IdentityProviderAccount>) {
        self.inner.set("accounts", value);
    }
}
