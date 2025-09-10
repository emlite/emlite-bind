use super::*;

/// The HIDInputReportEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDInputReportEventInit {
    inner: Any,
}

impl FromVal for HIDInputReportEventInit {
    fn from_val(v: &Any) -> Self {
        HIDInputReportEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HIDInputReportEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HIDInputReportEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HIDInputReportEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HIDInputReportEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HIDInputReportEventInit> for Any {
    fn from(s: HIDInputReportEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HIDInputReportEventInit> for Any {
    fn from(s: &HIDInputReportEventInit) -> Any {
        s.inner.clone()
    }
}

impl HIDInputReportEventInit {
    /// Getter of the `device` attribute.
    pub fn device(&self) -> HIDDevice {
        self.inner.get("device").as_::<HIDDevice>()
    }

    /// Setter of the `device` attribute.
    pub fn set_device(&mut self, value: &HIDDevice) {
        self.inner.set("device", value);
    }
}
impl HIDInputReportEventInit {
    /// Getter of the `reportId` attribute.
    pub fn report_id(&self) -> u8 {
        self.inner.get("reportId").as_::<u8>()
    }

    /// Setter of the `reportId` attribute.
    pub fn set_report_id(&mut self, value: u8) {
        self.inner.set("reportId", value);
    }
}
impl HIDInputReportEventInit {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> DataView {
        self.inner.get("data").as_::<DataView>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &DataView) {
        self.inner.set("data", value);
    }
}
