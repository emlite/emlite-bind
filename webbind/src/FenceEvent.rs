use super::*;

/// The FenceEvent dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FenceEvent {
    inner: Any,
}

impl FromVal for FenceEvent {
    fn from_val(v: &Any) -> Self {
        FenceEvent { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FenceEvent {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FenceEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FenceEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FenceEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FenceEvent> for Any {
    fn from(s: FenceEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FenceEvent> for Any {
    fn from(s: &FenceEvent) -> Any {
        s.inner.clone()
    }
}

impl FenceEvent {
    /// Getter of the `eventType` attribute.
    pub fn event_type(&self) -> JsString {
        self.inner.get("eventType").as_::<JsString>()
    }

    /// Setter of the `eventType` attribute.
    pub fn set_event_type(&mut self, value: &JsString) {
        self.inner.set("eventType", value);
    }
}
impl FenceEvent {
    /// Getter of the `eventData` attribute.
    pub fn event_data(&self) -> JsString {
        self.inner.get("eventData").as_::<JsString>()
    }

    /// Setter of the `eventData` attribute.
    pub fn set_event_data(&mut self, value: &JsString) {
        self.inner.set("eventData", value);
    }
}
impl FenceEvent {
    /// Getter of the `destination` attribute.
    pub fn destination(&self) -> TypedArray<FenceReportingDestination> {
        self.inner
            .get("destination")
            .as_::<TypedArray<FenceReportingDestination>>()
    }

    /// Setter of the `destination` attribute.
    pub fn set_destination(&mut self, value: &TypedArray<FenceReportingDestination>) {
        self.inner.set("destination", value);
    }
}
impl FenceEvent {
    /// Getter of the `crossOriginExposed` attribute.
    pub fn cross_origin_exposed(&self) -> bool {
        self.inner.get("crossOriginExposed").as_::<bool>()
    }

    /// Setter of the `crossOriginExposed` attribute.
    pub fn set_cross_origin_exposed(&mut self, value: bool) {
        self.inner.set("crossOriginExposed", value);
    }
}
impl FenceEvent {
    /// Getter of the `once` attribute.
    pub fn once(&self) -> bool {
        self.inner.get("once").as_::<bool>()
    }

    /// Setter of the `once` attribute.
    pub fn set_once(&mut self, value: bool) {
        self.inner.set("once", value);
    }
}
impl FenceEvent {
    /// Getter of the `destinationURL` attribute.
    pub fn destination_url(&self) -> JsString {
        self.inner.get("destinationURL").as_::<JsString>()
    }

    /// Setter of the `destinationURL` attribute.
    pub fn set_destination_url(&mut self, value: &JsString) {
        self.inner.set("destinationURL", value);
    }
}
