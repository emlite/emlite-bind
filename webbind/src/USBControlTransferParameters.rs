use super::*;




/// The USBControlTransferParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBControlTransferParameters {
    inner: Any,
}

impl FromVal for USBControlTransferParameters {
    fn from_val(v: &Any) -> Self {
        USBControlTransferParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBControlTransferParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBControlTransferParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBControlTransferParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBControlTransferParameters {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<USBControlTransferParameters> for Any {
    fn from(s: USBControlTransferParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBControlTransferParameters> for Any {
    fn from(s: &USBControlTransferParameters) -> Any {
        s.inner.clone()
    }
}

impl USBControlTransferParameters {
    /// Getter of the `requestType` attribute.
    pub fn request_type(&self) -> USBRequestType {
        self.inner.get("requestType").as_::<USBRequestType>()
    }

    /// Setter of the `requestType` attribute.
    pub fn set_request_type(&mut self, value: &USBRequestType) {
        self.inner.set("requestType", value);
    }
}
impl USBControlTransferParameters {
    /// Getter of the `recipient` attribute.
    pub fn recipient(&self) -> USBRecipient {
        self.inner.get("recipient").as_::<USBRecipient>()
    }

    /// Setter of the `recipient` attribute.
    pub fn set_recipient(&mut self, value: &USBRecipient) {
        self.inner.set("recipient", value);
    }
}
impl USBControlTransferParameters {
    /// Getter of the `request` attribute.
    pub fn request(&self) -> u8 {
        self.inner.get("request").as_::<u8>()
    }

    /// Setter of the `request` attribute.
    pub fn set_request(&mut self, value: u8) {
        self.inner.set("request", value);
    }
}
impl USBControlTransferParameters {
    /// Getter of the `value` attribute.
    pub fn value(&self) -> u16 {
        self.inner.get("value").as_::<u16>()
    }

    /// Setter of the `value` attribute.
    pub fn set_value(&mut self, value: u16) {
        self.inner.set("value", value);
    }
}
impl USBControlTransferParameters {
    /// Getter of the `index` attribute.
    pub fn index(&self) -> u16 {
        self.inner.get("index").as_::<u16>()
    }

    /// Setter of the `index` attribute.
    pub fn set_index(&mut self, value: u16) {
        self.inner.set("index", value);
    }
}
