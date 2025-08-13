use super::*;




/// The DisconnectedAccount dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DisconnectedAccount {
    inner: Any,
}

impl FromVal for DisconnectedAccount {
    fn from_val(v: &Any) -> Self {
        DisconnectedAccount { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DisconnectedAccount {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DisconnectedAccount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DisconnectedAccount {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DisconnectedAccount {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DisconnectedAccount> for Any {
    fn from(s: DisconnectedAccount) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DisconnectedAccount> for Any {
    fn from(s: &DisconnectedAccount) -> Any {
        s.inner.clone()
    }
}

impl DisconnectedAccount {
    /// Getter of the `account_id` attribute.
    pub fn account_id(&self) -> JsString {
        self.inner.get("account_id").as_::<JsString>()
    }

    /// Setter of the `account_id` attribute.
    pub fn set_account_id(&mut self, value: &JsString) {
        self.inner.set("account_id", value);
    }
}
