use super::*;

/// The WebTransportSendGroup class.
/// [`WebTransportSendGroup`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendGroup)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportSendGroup {
    inner: Any,
}

impl FromVal for WebTransportSendGroup {
    fn from_val(v: &Any) -> Self {
        WebTransportSendGroup {
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

impl core::ops::Deref for WebTransportSendGroup {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportSendGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportSendGroup {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportSendGroup {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebTransportSendGroup> for Any {
    fn from(s: WebTransportSendGroup) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportSendGroup> for Any {
    fn from(s: &WebTransportSendGroup) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebTransportSendGroup);

impl WebTransportSendGroup {
    /// The getStats method.
    /// [`WebTransportSendGroup.getStats`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendGroup/getStats)
    pub fn get_stats(&self) -> Promise<WebTransportSendStreamStats> {
        self.inner
            .call("getStats", &[])
            .as_::<Promise<WebTransportSendStreamStats>>()
    }
}
