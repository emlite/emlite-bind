use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDInputReportEvent {
    inner: Event,
}
impl FromVal for HIDInputReportEvent {
    fn from_val(v: &emlite::Val) -> Self {
        HIDInputReportEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for HIDInputReportEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HIDInputReportEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HIDInputReportEvent> for emlite::Val {
    fn from(s: HIDInputReportEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HIDInputReportEvent> for emlite::Val {
    fn from(s: &HIDInputReportEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HIDInputReportEvent);

impl HIDInputReportEvent {
    pub fn new(type_: &str, event_init_dict: &Any) -> HIDInputReportEvent {
        Self {
            inner: emlite::Val::global("HIDInputReportEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl HIDInputReportEvent {
    pub fn device(&self) -> HIDDevice {
        self.inner.get("device").as_::<HIDDevice>()
    }
}
impl HIDInputReportEvent {
    pub fn report_id(&self) -> u8 {
        self.inner.get("reportId").as_::<u8>()
    }
}
impl HIDInputReportEvent {
    pub fn data(&self) -> DataView {
        self.inner.get("data").as_::<DataView>()
    }
}
