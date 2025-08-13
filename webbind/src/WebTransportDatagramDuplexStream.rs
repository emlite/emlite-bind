use super::*;




/// The WebTransportDatagramDuplexStream class.
/// [`WebTransportDatagramDuplexStream`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportDatagramDuplexStream {
    inner: Any,
}

impl FromVal for WebTransportDatagramDuplexStream {
    fn from_val(v: &Any) -> Self {
        WebTransportDatagramDuplexStream { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebTransportDatagramDuplexStream {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportDatagramDuplexStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportDatagramDuplexStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportDatagramDuplexStream {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebTransportDatagramDuplexStream> for Any {
    fn from(s: WebTransportDatagramDuplexStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportDatagramDuplexStream> for Any {
    fn from(s: &WebTransportDatagramDuplexStream) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebTransportDatagramDuplexStream);


impl WebTransportDatagramDuplexStream {
    /// The createWritable method.
    /// [`WebTransportDatagramDuplexStream.createWritable`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/createWritable)
    pub fn create_writable0(&self, ) -> WebTransportDatagramsWritable {
        self.inner.call("createWritable", &[]).as_::<WebTransportDatagramsWritable>()
    }
    /// The createWritable method.
    /// [`WebTransportDatagramDuplexStream.createWritable`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/createWritable)
    pub fn create_writable1(&self, options: &WebTransportSendOptions) -> WebTransportDatagramsWritable {
        self.inner.call("createWritable", &[options.into(), ]).as_::<WebTransportDatagramsWritable>()
    }
}
impl WebTransportDatagramDuplexStream {
    /// Getter of the `readable` attribute.
    /// [`WebTransportDatagramDuplexStream.readable`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/readable)
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }

}
impl WebTransportDatagramDuplexStream {
    /// Getter of the `maxDatagramSize` attribute.
    /// [`WebTransportDatagramDuplexStream.maxDatagramSize`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/maxDatagramSize)
    pub fn max_datagram_size(&self) -> u32 {
        self.inner.get("maxDatagramSize").as_::<u32>()
    }

}
impl WebTransportDatagramDuplexStream {
    /// Getter of the `incomingMaxAge` attribute.
    /// [`WebTransportDatagramDuplexStream.incomingMaxAge`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingMaxAge)
    pub fn incoming_max_age(&self) -> f64 {
        self.inner.get("incomingMaxAge").as_::<f64>()
    }

    /// Setter of the `incomingMaxAge` attribute.
    /// [`WebTransportDatagramDuplexStream.incomingMaxAge`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingMaxAge)
    pub fn set_incoming_max_age(&mut self, value: f64) {
        self.inner.set("incomingMaxAge", value);
    }
}
impl WebTransportDatagramDuplexStream {
    /// Getter of the `outgoingMaxAge` attribute.
    /// [`WebTransportDatagramDuplexStream.outgoingMaxAge`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingMaxAge)
    pub fn outgoing_max_age(&self) -> f64 {
        self.inner.get("outgoingMaxAge").as_::<f64>()
    }

    /// Setter of the `outgoingMaxAge` attribute.
    /// [`WebTransportDatagramDuplexStream.outgoingMaxAge`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingMaxAge)
    pub fn set_outgoing_max_age(&mut self, value: f64) {
        self.inner.set("outgoingMaxAge", value);
    }
}
impl WebTransportDatagramDuplexStream {
    /// Getter of the `incomingHighWaterMark` attribute.
    /// [`WebTransportDatagramDuplexStream.incomingHighWaterMark`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingHighWaterMark)
    pub fn incoming_high_water_mark(&self) -> f64 {
        self.inner.get("incomingHighWaterMark").as_::<f64>()
    }

    /// Setter of the `incomingHighWaterMark` attribute.
    /// [`WebTransportDatagramDuplexStream.incomingHighWaterMark`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingHighWaterMark)
    pub fn set_incoming_high_water_mark(&mut self, value: f64) {
        self.inner.set("incomingHighWaterMark", value);
    }
}
impl WebTransportDatagramDuplexStream {
    /// Getter of the `outgoingHighWaterMark` attribute.
    /// [`WebTransportDatagramDuplexStream.outgoingHighWaterMark`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingHighWaterMark)
    pub fn outgoing_high_water_mark(&self) -> f64 {
        self.inner.get("outgoingHighWaterMark").as_::<f64>()
    }

    /// Setter of the `outgoingHighWaterMark` attribute.
    /// [`WebTransportDatagramDuplexStream.outgoingHighWaterMark`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingHighWaterMark)
    pub fn set_outgoing_high_water_mark(&mut self, value: f64) {
        self.inner.set("outgoingHighWaterMark", value);
    }
}
