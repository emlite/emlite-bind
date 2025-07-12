use super::*;

#[derive(Clone, Debug)]
pub struct WebTransportBidirectionalStream {
    inner: emlite::Val,
}
impl FromVal for WebTransportBidirectionalStream {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportBidirectionalStream {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WebTransportBidirectionalStream {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WebTransportBidirectionalStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebTransportBidirectionalStream> for emlite::Val {
    fn from(s: WebTransportBidirectionalStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebTransportBidirectionalStream {
    pub fn readable(&self) -> WebTransportReceiveStream {
        self.inner
            .get("readable")
            .as_::<WebTransportReceiveStream>()
    }
}
impl WebTransportBidirectionalStream {
    pub fn writable(&self) -> WebTransportSendStream {
        self.inner.get("writable").as_::<WebTransportSendStream>()
    }
}
