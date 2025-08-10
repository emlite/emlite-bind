use super::*;

/// The HIDCollectionInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDCollectionInfo {
    inner: Any,
}

impl FromVal for HIDCollectionInfo {
    fn from_val(v: &Any) -> Self {
        HIDCollectionInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HIDCollectionInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HIDCollectionInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HIDCollectionInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HIDCollectionInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HIDCollectionInfo> for Any {
    fn from(s: HIDCollectionInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HIDCollectionInfo> for Any {
    fn from(s: &HIDCollectionInfo) -> Any {
        s.inner.clone()
    }
}

impl HIDCollectionInfo {
    /// Getter of the `usagePage` attribute.
    pub fn usage_page(&self) -> u16 {
        self.inner.get("usagePage").as_::<u16>()
    }

    /// Setter of the `usagePage` attribute.
    pub fn set_usage_page(&mut self, value: u16) {
        self.inner.set("usagePage", value);
    }
}
impl HIDCollectionInfo {
    /// Getter of the `usage` attribute.
    pub fn usage(&self) -> u16 {
        self.inner.get("usage").as_::<u16>()
    }

    /// Setter of the `usage` attribute.
    pub fn set_usage(&mut self, value: u16) {
        self.inner.set("usage", value);
    }
}
impl HIDCollectionInfo {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> u8 {
        self.inner.get("type").as_::<u8>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: u8) {
        self.inner.set("type", value);
    }
}
impl HIDCollectionInfo {
    /// Getter of the `children` attribute.
    pub fn children(&self) -> TypedArray<HIDCollectionInfo> {
        self.inner
            .get("children")
            .as_::<TypedArray<HIDCollectionInfo>>()
    }

    /// Setter of the `children` attribute.
    pub fn set_children(&mut self, value: &TypedArray<HIDCollectionInfo>) {
        self.inner.set("children", value);
    }
}
impl HIDCollectionInfo {
    /// Getter of the `inputReports` attribute.
    pub fn input_reports(&self) -> TypedArray<HIDReportInfo> {
        self.inner
            .get("inputReports")
            .as_::<TypedArray<HIDReportInfo>>()
    }

    /// Setter of the `inputReports` attribute.
    pub fn set_input_reports(&mut self, value: &TypedArray<HIDReportInfo>) {
        self.inner.set("inputReports", value);
    }
}
impl HIDCollectionInfo {
    /// Getter of the `outputReports` attribute.
    pub fn output_reports(&self) -> TypedArray<HIDReportInfo> {
        self.inner
            .get("outputReports")
            .as_::<TypedArray<HIDReportInfo>>()
    }

    /// Setter of the `outputReports` attribute.
    pub fn set_output_reports(&mut self, value: &TypedArray<HIDReportInfo>) {
        self.inner.set("outputReports", value);
    }
}
impl HIDCollectionInfo {
    /// Getter of the `featureReports` attribute.
    pub fn feature_reports(&self) -> TypedArray<HIDReportInfo> {
        self.inner
            .get("featureReports")
            .as_::<TypedArray<HIDReportInfo>>()
    }

    /// Setter of the `featureReports` attribute.
    pub fn set_feature_reports(&mut self, value: &TypedArray<HIDReportInfo>) {
        self.inner.set("featureReports", value);
    }
}
