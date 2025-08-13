use super::*;




/// The PresentationConnectionList class.
/// [`PresentationConnectionList`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationConnectionList {
    inner: EventTarget,
}

impl FromVal for PresentationConnectionList {
    fn from_val(v: &Any) -> Self {
        PresentationConnectionList { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PresentationConnectionList {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PresentationConnectionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PresentationConnectionList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PresentationConnectionList {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PresentationConnectionList> for Any {
    fn from(s: PresentationConnectionList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PresentationConnectionList> for Any {
    fn from(s: &PresentationConnectionList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PresentationConnectionList);


impl PresentationConnectionList {
    /// Getter of the `connections` attribute.
    /// [`PresentationConnectionList.connections`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/connections)
    pub fn connections(&self) -> TypedArray<PresentationConnection> {
        self.inner.get("connections").as_::<TypedArray<PresentationConnection>>()
    }

}
impl PresentationConnectionList {
    /// Getter of the `onconnectionavailable` attribute.
    /// [`PresentationConnectionList.onconnectionavailable`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/onconnectionavailable)
    pub fn onconnectionavailable(&self) -> Any {
        self.inner.get("onconnectionavailable").as_::<Any>()
    }

    /// Setter of the `onconnectionavailable` attribute.
    /// [`PresentationConnectionList.onconnectionavailable`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/onconnectionavailable)
    pub fn set_onconnectionavailable(&mut self, value: &Any) {
        self.inner.set("onconnectionavailable", value);
    }
}
