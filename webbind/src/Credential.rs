use super::*;




/// The Credential class.
/// [`Credential`](https://developer.mozilla.org/en-US/docs/Web/API/Credential)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Credential {
    inner: Any,
}

impl FromVal for Credential {
    fn from_val(v: &Any) -> Self {
        Credential { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Credential {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Credential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Credential {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Credential {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Credential> for Any {
    fn from(s: Credential) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Credential> for Any {
    fn from(s: &Credential) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Credential);


impl Credential {
    /// Getter of the `id` attribute.
    /// [`Credential.id`](https://developer.mozilla.org/en-US/docs/Web/API/Credential/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

}
impl Credential {
    /// Getter of the `type` attribute.
    /// [`Credential.type`](https://developer.mozilla.org/en-US/docs/Web/API/Credential/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

}
impl Credential {
    /// The isConditionalMediationAvailable method.
    /// [`Credential.isConditionalMediationAvailable`](https://developer.mozilla.org/en-US/docs/Web/API/Credential/isConditionalMediationAvailable)
    pub fn is_conditional_mediation_available() -> Promise<bool> {
        Any::global("Credential").call("isConditionalMediationAvailable", &[]).as_::<Promise<bool>>()
    }
}
impl Credential {
    /// The willRequestConditionalCreation method.
    /// [`Credential.willRequestConditionalCreation`](https://developer.mozilla.org/en-US/docs/Web/API/Credential/willRequestConditionalCreation)
    pub fn will_request_conditional_creation() -> Promise<Undefined> {
        Any::global("Credential").call("willRequestConditionalCreation", &[]).as_::<Promise<Undefined>>()
    }
}
