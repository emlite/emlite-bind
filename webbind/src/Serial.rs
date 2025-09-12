use super::*;

/// The Serial class.
/// [`Serial`](https://developer.mozilla.org/en-US/docs/Web/API/Serial)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Serial {
    inner: EventTarget,
}

impl FromVal for Serial {
    fn from_val(v: &Any) -> Self {
        Serial {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Serial {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Serial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Serial {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Serial {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Serial> for Any {
    fn from(s: Serial) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Serial> for Any {
    fn from(s: &Serial) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Serial);

impl Serial {
    /// Getter of the `onconnect` attribute.
    /// [`Serial.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/Serial/onconnect)
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    /// Setter of the `onconnect` attribute.
    /// [`Serial.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/Serial/onconnect)
    pub fn set_onconnect(&mut self, value: &Any) {
        self.inner.set("onconnect", value);
    }
}
impl Serial {
    /// Getter of the `ondisconnect` attribute.
    /// [`Serial.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/Serial/ondisconnect)
    pub fn ondisconnect(&self) -> Any {
        self.inner.get("ondisconnect").as_::<Any>()
    }

    /// Setter of the `ondisconnect` attribute.
    /// [`Serial.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/Serial/ondisconnect)
    pub fn set_ondisconnect(&mut self, value: &Any) {
        self.inner.set("ondisconnect", value);
    }
}
impl Serial {
    /// The getPorts method.
    /// [`Serial.getPorts`](https://developer.mozilla.org/en-US/docs/Web/API/Serial/getPorts)
    pub fn get_ports(&self) -> Promise<TypedArray<SerialPort>> {
        self.inner
            .call("getPorts", &[])
            .as_::<Promise<TypedArray<SerialPort>>>()
    }
}
impl Serial {
    /// The requestPort method.
    /// [`Serial.requestPort`](https://developer.mozilla.org/en-US/docs/Web/API/Serial/requestPort)
    pub fn request_port(&self) -> Promise<SerialPort> {
        self.inner
            .call("requestPort", &[])
            .as_::<Promise<SerialPort>>()
    }
}
impl Serial {
    /// The requestPort method.
    /// [`Serial.requestPort`](https://developer.mozilla.org/en-US/docs/Web/API/Serial/requestPort)
    pub fn request_port_with_options(
        &self,
        options: &SerialPortRequestOptions,
    ) -> Promise<SerialPort> {
        self.inner
            .call("requestPort", &[options.into()])
            .as_::<Promise<SerialPort>>()
    }
}
