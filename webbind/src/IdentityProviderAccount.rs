use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityProviderAccount {
    inner: Any,
}
impl FromVal for IdentityProviderAccount {
    fn from_val(v: &Any) -> Self {
        IdentityProviderAccount { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityProviderAccount {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityProviderAccount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityProviderAccount {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityProviderAccount {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityProviderAccount> for Any {
    fn from(s: IdentityProviderAccount) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityProviderAccount> for Any {
    fn from(s: &IdentityProviderAccount) -> Any {
        s.inner.clone()
    }
}

impl IdentityProviderAccount {
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl IdentityProviderAccount {
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl IdentityProviderAccount {
    pub fn email(&self) -> JsString {
        self.inner.get("email").as_::<JsString>()
    }

    pub fn set_email(&mut self, value: &JsString) {
        self.inner.set("email", value);
    }
}
impl IdentityProviderAccount {
    pub fn tel(&self) -> JsString {
        self.inner.get("tel").as_::<JsString>()
    }

    pub fn set_tel(&mut self, value: &JsString) {
        self.inner.set("tel", value);
    }
}
impl IdentityProviderAccount {
    pub fn username(&self) -> JsString {
        self.inner.get("username").as_::<JsString>()
    }

    pub fn set_username(&mut self, value: &JsString) {
        self.inner.set("username", value);
    }
}
impl IdentityProviderAccount {
    pub fn given_name(&self) -> JsString {
        self.inner.get("given_name").as_::<JsString>()
    }

    pub fn set_given_name(&mut self, value: &JsString) {
        self.inner.set("given_name", value);
    }
}
impl IdentityProviderAccount {
    pub fn picture(&self) -> JsString {
        self.inner.get("picture").as_::<JsString>()
    }

    pub fn set_picture(&mut self, value: &JsString) {
        self.inner.set("picture", value);
    }
}
impl IdentityProviderAccount {
    pub fn approved_clients(&self) -> TypedArray<JsString> {
        self.inner
            .get("approved_clients")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_approved_clients(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("approved_clients", value);
    }
}
impl IdentityProviderAccount {
    pub fn login_hints(&self) -> TypedArray<JsString> {
        self.inner.get("login_hints").as_::<TypedArray<JsString>>()
    }

    pub fn set_login_hints(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("login_hints", value);
    }
}
impl IdentityProviderAccount {
    pub fn domain_hints(&self) -> TypedArray<JsString> {
        self.inner.get("domain_hints").as_::<TypedArray<JsString>>()
    }

    pub fn set_domain_hints(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("domain_hints", value);
    }
}
impl IdentityProviderAccount {
    pub fn label_hints(&self) -> TypedArray<JsString> {
        self.inner.get("label_hints").as_::<TypedArray<JsString>>()
    }

    pub fn set_label_hints(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("label_hints", value);
    }
}
