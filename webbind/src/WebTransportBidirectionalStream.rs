use super::*;




/// The WebTransportBidirectionalStream class.
/// [`WebTransportBidirectionalStream`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportBidirectionalStream {
    inner: Any,
}

impl FromVal for WebTransportBidirectionalStream {
    fn from_val(v: &Any) -> Self {
        WebTransportBidirectionalStream { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebTransportBidirectionalStream {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportBidirectionalStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportBidirectionalStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportBidirectionalStream {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebTransportBidirectionalStream> for Any {
    fn from(s: WebTransportBidirectionalStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportBidirectionalStream> for Any {
    fn from(s: &WebTransportBidirectionalStream) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebTransportBidirectionalStream);


impl WebTransportBidirectionalStream {
    /// Getter of the `readable` attribute.
    /// [`WebTransportBidirectionalStream.readable`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream/readable)
    pub fn readable(&self) -> WebTransportReceiveStream {
        self.inner.get("readable").as_::<WebTransportReceiveStream>()
    }

}
impl WebTransportBidirectionalStream {
    /// Getter of the `writable` attribute.
    /// [`WebTransportBidirectionalStream.writable`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream/writable)
    pub fn writable(&self) -> WebTransportSendStream {
        self.inner.get("writable").as_::<WebTransportSendStream>()
    }

}
