use super::*;

/// The PresentationReceiver class.
/// [`PresentationReceiver`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationReceiver {
    inner: Any,
}

impl FromVal for PresentationReceiver {
    fn from_val(v: &Any) -> Self {
        PresentationReceiver {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PresentationReceiver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PresentationReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PresentationReceiver {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PresentationReceiver {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PresentationReceiver> for Any {
    fn from(s: PresentationReceiver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PresentationReceiver> for Any {
    fn from(s: &PresentationReceiver) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PresentationReceiver);

impl PresentationReceiver {
    /// Getter of the `connectionList` attribute.
    /// [`PresentationReceiver.connectionList`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver/connectionList)
    pub fn connection_list(&self) -> Promise<PresentationConnectionList> {
        self.inner
            .get("connectionList")
            .as_::<Promise<PresentationConnectionList>>()
    }
}
