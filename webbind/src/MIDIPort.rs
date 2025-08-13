use super::*;




/// The MIDIPort class.
/// [`MIDIPort`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIPort {
    inner: EventTarget,
}

impl FromVal for MIDIPort {
    fn from_val(v: &Any) -> Self {
        MIDIPort { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MIDIPort {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MIDIPort {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MIDIPort {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MIDIPort {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MIDIPort> for Any {
    fn from(s: MIDIPort) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MIDIPort> for Any {
    fn from(s: &MIDIPort) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MIDIPort);


impl MIDIPort {
    /// Getter of the `id` attribute.
    /// [`MIDIPort.id`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

}
impl MIDIPort {
    /// Getter of the `manufacturer` attribute.
    /// [`MIDIPort.manufacturer`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/manufacturer)
    pub fn manufacturer(&self) -> JsString {
        self.inner.get("manufacturer").as_::<JsString>()
    }

}
impl MIDIPort {
    /// Getter of the `name` attribute.
    /// [`MIDIPort.name`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

}
impl MIDIPort {
    /// Getter of the `type` attribute.
    /// [`MIDIPort.type`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/type)
    pub fn type_(&self) -> MIDIPortType {
        self.inner.get("type").as_::<MIDIPortType>()
    }

}
impl MIDIPort {
    /// Getter of the `version` attribute.
    /// [`MIDIPort.version`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/version)
    pub fn version(&self) -> JsString {
        self.inner.get("version").as_::<JsString>()
    }

}
impl MIDIPort {
    /// Getter of the `state` attribute.
    /// [`MIDIPort.state`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/state)
    pub fn state(&self) -> MIDIPortDeviceState {
        self.inner.get("state").as_::<MIDIPortDeviceState>()
    }

}
impl MIDIPort {
    /// Getter of the `connection` attribute.
    /// [`MIDIPort.connection`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/connection)
    pub fn connection(&self) -> MIDIPortConnectionState {
        self.inner.get("connection").as_::<MIDIPortConnectionState>()
    }

}
impl MIDIPort {
    /// Getter of the `onstatechange` attribute.
    /// [`MIDIPort.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)
    pub fn onstatechange(&self) -> Any {
        self.inner.get("onstatechange").as_::<Any>()
    }

    /// Setter of the `onstatechange` attribute.
    /// [`MIDIPort.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)
    pub fn set_onstatechange(&mut self, value: &Any) {
        self.inner.set("onstatechange", value);
    }
}
impl MIDIPort {
    /// The open method.
    /// [`MIDIPort.open`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/open)
    pub fn open(&self, ) -> Promise<MIDIPort> {
        self.inner.call("open", &[]).as_::<Promise<MIDIPort>>()
    }
}
impl MIDIPort {
    /// The close method.
    /// [`MIDIPort.close`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/close)
    pub fn close(&self, ) -> Promise<MIDIPort> {
        self.inner.call("close", &[]).as_::<Promise<MIDIPort>>()
    }
}
