use super::*;

/// The HIDInputReportEvent class.
/// [`HIDInputReportEvent`](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDInputReportEvent {
    inner: Event,
}
impl FromVal for HIDInputReportEvent {
    fn from_val(v: &Any) -> Self {
        HIDInputReportEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HIDInputReportEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HIDInputReportEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HIDInputReportEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HIDInputReportEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HIDInputReportEvent> for Any {
    fn from(s: HIDInputReportEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HIDInputReportEvent> for Any {
    fn from(s: &HIDInputReportEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HIDInputReportEvent);

impl HIDInputReportEvent {
    /// The `new HIDInputReportEvent(..)` constructor, creating a new HIDInputReportEvent instance
    pub fn new(type_: &JsString, event_init_dict: &Any) -> HIDInputReportEvent {
        Self {
            inner: Any::global("HIDInputReportEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl HIDInputReportEvent {
    /// Getter of the `device` attribute.
    /// [`HIDInputReportEvent.device`](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent/device)
    pub fn device(&self) -> HIDDevice {
        self.inner.get("device").as_::<HIDDevice>()
    }
}
impl HIDInputReportEvent {
    /// Getter of the `reportId` attribute.
    /// [`HIDInputReportEvent.reportId`](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent/reportId)
    pub fn report_id(&self) -> u8 {
        self.inner.get("reportId").as_::<u8>()
    }
}
impl HIDInputReportEvent {
    /// Getter of the `data` attribute.
    /// [`HIDInputReportEvent.data`](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent/data)
    pub fn data(&self) -> DataView {
        self.inner.get("data").as_::<DataView>()
    }
}
