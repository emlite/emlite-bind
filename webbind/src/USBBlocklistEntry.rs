use super::*;

/// The USBBlocklistEntry dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBBlocklistEntry {
    inner: Any,
}

impl FromVal for USBBlocklistEntry {
    fn from_val(v: &Any) -> Self {
        USBBlocklistEntry { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBBlocklistEntry {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBBlocklistEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBBlocklistEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBBlocklistEntry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<USBBlocklistEntry> for Any {
    fn from(s: USBBlocklistEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBBlocklistEntry> for Any {
    fn from(s: &USBBlocklistEntry) -> Any {
        s.inner.clone()
    }
}

impl USBBlocklistEntry {
    /// Getter of the `idVendor` attribute.
    pub fn id_vendor(&self) -> u16 {
        self.inner.get("idVendor").as_::<u16>()
    }

    /// Setter of the `idVendor` attribute.
    pub fn set_id_vendor(&mut self, value: u16) {
        self.inner.set("idVendor", value);
    }
}
impl USBBlocklistEntry {
    /// Getter of the `idProduct` attribute.
    pub fn id_product(&self) -> u16 {
        self.inner.get("idProduct").as_::<u16>()
    }

    /// Setter of the `idProduct` attribute.
    pub fn set_id_product(&mut self, value: u16) {
        self.inner.set("idProduct", value);
    }
}
impl USBBlocklistEntry {
    /// Getter of the `bcdDevice` attribute.
    pub fn bcd_device(&self) -> u16 {
        self.inner.get("bcdDevice").as_::<u16>()
    }

    /// Setter of the `bcdDevice` attribute.
    pub fn set_bcd_device(&mut self, value: u16) {
        self.inner.set("bcdDevice", value);
    }
}
