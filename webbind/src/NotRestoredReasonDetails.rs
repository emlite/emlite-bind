use super::*;




/// The NotRestoredReasonDetails class.
/// [`NotRestoredReasonDetails`](https://developer.mozilla.org/en-US/docs/Web/API/NotRestoredReasonDetails)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NotRestoredReasonDetails {
    inner: Any,
}

impl FromVal for NotRestoredReasonDetails {
    fn from_val(v: &Any) -> Self {
        NotRestoredReasonDetails { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NotRestoredReasonDetails {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NotRestoredReasonDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NotRestoredReasonDetails {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NotRestoredReasonDetails {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NotRestoredReasonDetails> for Any {
    fn from(s: NotRestoredReasonDetails) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NotRestoredReasonDetails> for Any {
    fn from(s: &NotRestoredReasonDetails) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NotRestoredReasonDetails);


impl NotRestoredReasonDetails {
    /// Getter of the `reason` attribute.
    /// [`NotRestoredReasonDetails.reason`](https://developer.mozilla.org/en-US/docs/Web/API/NotRestoredReasonDetails/reason)
    pub fn reason(&self) -> JsString {
        self.inner.get("reason").as_::<JsString>()
    }

}
impl NotRestoredReasonDetails {
    /// The toJSON method.
    /// [`NotRestoredReasonDetails.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/NotRestoredReasonDetails/toJSON)
    pub fn to_json(&self, ) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
