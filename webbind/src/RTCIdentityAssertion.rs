use super::*;




/// The RTCIdentityAssertion class.
/// [`RTCIdentityAssertion`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityAssertion)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIdentityAssertion {
    inner: Any,
}

impl FromVal for RTCIdentityAssertion {
    fn from_val(v: &Any) -> Self {
        RTCIdentityAssertion { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIdentityAssertion {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIdentityAssertion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIdentityAssertion {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIdentityAssertion {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCIdentityAssertion> for Any {
    fn from(s: RTCIdentityAssertion) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIdentityAssertion> for Any {
    fn from(s: &RTCIdentityAssertion) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCIdentityAssertion);



impl RTCIdentityAssertion {
    /// The `new RTCIdentityAssertion(..)` constructor, creating a new RTCIdentityAssertion instance
    pub fn new(idp: &JsString, name: &JsString) -> RTCIdentityAssertion {
        Self {
            inner: Any::global("RTCIdentityAssertion").new(&[idp.into(), name.into()]).as_::<Any>(),
        }
    }

}
impl RTCIdentityAssertion {
    /// Getter of the `idp` attribute.
    /// [`RTCIdentityAssertion.idp`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityAssertion/idp)
    pub fn idp(&self) -> JsString {
        self.inner.get("idp").as_::<JsString>()
    }

    /// Setter of the `idp` attribute.
    /// [`RTCIdentityAssertion.idp`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityAssertion/idp)
    pub fn set_idp(&mut self, value: &JsString) {
        self.inner.set("idp", value);
    }
}
impl RTCIdentityAssertion {
    /// Getter of the `name` attribute.
    /// [`RTCIdentityAssertion.name`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityAssertion/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`RTCIdentityAssertion.name`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityAssertion/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
