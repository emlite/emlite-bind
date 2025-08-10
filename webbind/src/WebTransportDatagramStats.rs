use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportDatagramStats {
    inner: Any,
}
impl FromVal for WebTransportDatagramStats {
    fn from_val(v: &Any) -> Self {
        WebTransportDatagramStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportDatagramStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportDatagramStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebTransportDatagramStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebTransportDatagramStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebTransportDatagramStats> for Any {
    fn from(s: WebTransportDatagramStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebTransportDatagramStats> for Any {
    fn from(s: &WebTransportDatagramStats) -> Any {
        s.inner.clone()
    }
}

impl WebTransportDatagramStats {
    pub fn dropped_incoming(&self) -> u64 {
        self.inner.get("droppedIncoming").as_::<u64>()
    }

    pub fn set_dropped_incoming(&mut self, value: u64) {
        self.inner.set("droppedIncoming", value);
    }
}
impl WebTransportDatagramStats {
    pub fn expired_incoming(&self) -> u64 {
        self.inner.get("expiredIncoming").as_::<u64>()
    }

    pub fn set_expired_incoming(&mut self, value: u64) {
        self.inner.set("expiredIncoming", value);
    }
}
impl WebTransportDatagramStats {
    pub fn expired_outgoing(&self) -> u64 {
        self.inner.get("expiredOutgoing").as_::<u64>()
    }

    pub fn set_expired_outgoing(&mut self, value: u64) {
        self.inner.set("expiredOutgoing", value);
    }
}
impl WebTransportDatagramStats {
    pub fn lost_outgoing(&self) -> u64 {
        self.inner.get("lostOutgoing").as_::<u64>()
    }

    pub fn set_lost_outgoing(&mut self, value: u64) {
        self.inner.set("lostOutgoing", value);
    }
}
