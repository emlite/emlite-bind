use super::*;

/// The PasswordCredential class.
/// [`PasswordCredential`](https://developer.mozilla.org/en-US/docs/Web/API/PasswordCredential)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PasswordCredential {
    inner: Credential,
}

impl FromVal for PasswordCredential {
    fn from_val(v: &Any) -> Self {
        PasswordCredential {
            inner: Credential::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PasswordCredential {
    type Target = Credential;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PasswordCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PasswordCredential {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PasswordCredential {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PasswordCredential> for Any {
    fn from(s: PasswordCredential) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PasswordCredential> for Any {
    fn from(s: &PasswordCredential) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PasswordCredential);

impl PasswordCredential {
    /// Getter of the `password` attribute.
    /// [`PasswordCredential.password`](https://developer.mozilla.org/en-US/docs/Web/API/PasswordCredential/password)
    pub fn password(&self) -> JsString {
        self.inner.get("password").as_::<JsString>()
    }
}
impl PasswordCredential {
    /// Getter of the `name` attribute.
    /// [`PasswordCredential.name`](https://developer.mozilla.org/en-US/docs/Web/API/PasswordCredential/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl PasswordCredential {
    /// Getter of the `iconURL` attribute.
    /// [`PasswordCredential.iconURL`](https://developer.mozilla.org/en-US/docs/Web/API/PasswordCredential/iconURL)
    pub fn icon_url(&self) -> JsString {
        self.inner.get("iconURL").as_::<JsString>()
    }
}

impl PasswordCredential {
    /// The `new PasswordCredential(..)` constructor, creating a new PasswordCredential instance
    pub fn new(form: &HTMLFormElement) -> PasswordCredential {
        Self {
            inner: Any::global("PasswordCredential")
                .new(&[form.into()])
                .as_::<Credential>(),
        }
    }
}

impl PasswordCredential {
    /// The `new PasswordCredential(..)` constructor, creating a new PasswordCredential instance
    pub fn new_with_data(data: &PasswordCredentialData) -> PasswordCredential {
        Self {
            inner: Any::global("PasswordCredential")
                .new(&[data.into()])
                .as_::<Credential>(),
        }
    }
}
