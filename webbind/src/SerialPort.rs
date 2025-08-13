use super::*;




/// The SerialPort class.
/// [`SerialPort`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialPort {
    inner: EventTarget,
}

impl FromVal for SerialPort {
    fn from_val(v: &Any) -> Self {
        SerialPort { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SerialPort {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SerialPort {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SerialPort {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SerialPort {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SerialPort> for Any {
    fn from(s: SerialPort) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SerialPort> for Any {
    fn from(s: &SerialPort) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SerialPort);


impl SerialPort {
    /// Getter of the `onconnect` attribute.
    /// [`SerialPort.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/onconnect)
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    /// Setter of the `onconnect` attribute.
    /// [`SerialPort.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/onconnect)
    pub fn set_onconnect(&mut self, value: &Any) {
        self.inner.set("onconnect", value);
    }
}
impl SerialPort {
    /// Getter of the `ondisconnect` attribute.
    /// [`SerialPort.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/ondisconnect)
    pub fn ondisconnect(&self) -> Any {
        self.inner.get("ondisconnect").as_::<Any>()
    }

    /// Setter of the `ondisconnect` attribute.
    /// [`SerialPort.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/ondisconnect)
    pub fn set_ondisconnect(&mut self, value: &Any) {
        self.inner.set("ondisconnect", value);
    }
}
impl SerialPort {
    /// Getter of the `connected` attribute.
    /// [`SerialPort.connected`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/connected)
    pub fn connected(&self) -> bool {
        self.inner.get("connected").as_::<bool>()
    }

}
impl SerialPort {
    /// Getter of the `readable` attribute.
    /// [`SerialPort.readable`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/readable)
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }

}
impl SerialPort {
    /// Getter of the `writable` attribute.
    /// [`SerialPort.writable`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/writable)
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }

}
impl SerialPort {
    /// The getInfo method.
    /// [`SerialPort.getInfo`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/getInfo)
    pub fn get_info(&self, ) -> SerialPortInfo {
        self.inner.call("getInfo", &[]).as_::<SerialPortInfo>()
    }
}
impl SerialPort {
    /// The open method.
    /// [`SerialPort.open`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/open)
    pub fn open(&self, options: &SerialOptions) -> Promise<Undefined> {
        self.inner.call("open", &[options.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl SerialPort {
    /// The setSignals method.
    /// [`SerialPort.setSignals`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/setSignals)
    pub fn set_signals0(&self, ) -> Promise<Undefined> {
        self.inner.call("setSignals", &[]).as_::<Promise<Undefined>>()
    }
    /// The setSignals method.
    /// [`SerialPort.setSignals`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/setSignals)
    pub fn set_signals1(&self, signals: &SerialOutputSignals) -> Promise<Undefined> {
        self.inner.call("setSignals", &[signals.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl SerialPort {
    /// The getSignals method.
    /// [`SerialPort.getSignals`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/getSignals)
    pub fn get_signals(&self, ) -> Promise<SerialInputSignals> {
        self.inner.call("getSignals", &[]).as_::<Promise<SerialInputSignals>>()
    }
}
impl SerialPort {
    /// The close method.
    /// [`SerialPort.close`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/close)
    pub fn close(&self, ) -> Promise<Undefined> {
        self.inner.call("close", &[]).as_::<Promise<Undefined>>()
    }
}
impl SerialPort {
    /// The forget method.
    /// [`SerialPort.forget`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/forget)
    pub fn forget(&self, ) -> Promise<Undefined> {
        self.inner.call("forget", &[]).as_::<Promise<Undefined>>()
    }
}
