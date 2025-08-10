use super::*;

/// The HIDReportInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDReportInfo {
    inner: Any,
}

impl FromVal for HIDReportInfo {
    fn from_val(v: &Any) -> Self {
        HIDReportInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HIDReportInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HIDReportInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HIDReportInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HIDReportInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HIDReportInfo> for Any {
    fn from(s: HIDReportInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HIDReportInfo> for Any {
    fn from(s: &HIDReportInfo) -> Any {
        s.inner.clone()
    }
}

impl HIDReportInfo {
    /// Getter of the `reportId` attribute.
    pub fn report_id(&self) -> u8 {
        self.inner.get("reportId").as_::<u8>()
    }

    /// Setter of the `reportId` attribute.
    pub fn set_report_id(&mut self, value: u8) {
        self.inner.set("reportId", value);
    }
}
impl HIDReportInfo {
    /// Getter of the `items` attribute.
    pub fn items(&self) -> TypedArray<HIDReportItem> {
        self.inner.get("items").as_::<TypedArray<HIDReportItem>>()
    }

    /// Setter of the `items` attribute.
    pub fn set_items(&mut self, value: &TypedArray<HIDReportItem>) {
        self.inner.set("items", value);
    }
}
