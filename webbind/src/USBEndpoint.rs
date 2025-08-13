use super::*;




/// The USBEndpoint class.
/// [`USBEndpoint`](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBEndpoint {
    inner: Any,
}

impl FromVal for USBEndpoint {
    fn from_val(v: &Any) -> Self {
        USBEndpoint { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBEndpoint {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBEndpoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBEndpoint {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBEndpoint {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<USBEndpoint> for Any {
    fn from(s: USBEndpoint) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBEndpoint> for Any {
    fn from(s: &USBEndpoint) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(USBEndpoint);



impl USBEndpoint {
    /// The `new USBEndpoint(..)` constructor, creating a new USBEndpoint instance
    pub fn new(alternate: &USBAlternateInterface, endpoint_number: u8, direction: &USBDirection) -> USBEndpoint {
        Self {
            inner: Any::global("USBEndpoint").new(&[alternate.into(), endpoint_number.into(), direction.into()]).as_::<Any>(),
        }
    }

}
impl USBEndpoint {
    /// Getter of the `endpointNumber` attribute.
    /// [`USBEndpoint.endpointNumber`](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/endpointNumber)
    pub fn endpoint_number(&self) -> u8 {
        self.inner.get("endpointNumber").as_::<u8>()
    }

}
impl USBEndpoint {
    /// Getter of the `direction` attribute.
    /// [`USBEndpoint.direction`](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/direction)
    pub fn direction(&self) -> USBDirection {
        self.inner.get("direction").as_::<USBDirection>()
    }

}
impl USBEndpoint {
    /// Getter of the `type` attribute.
    /// [`USBEndpoint.type`](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/type)
    pub fn type_(&self) -> USBEndpointType {
        self.inner.get("type").as_::<USBEndpointType>()
    }

}
impl USBEndpoint {
    /// Getter of the `packetSize` attribute.
    /// [`USBEndpoint.packetSize`](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/packetSize)
    pub fn packet_size(&self) -> u32 {
        self.inner.get("packetSize").as_::<u32>()
    }

}
