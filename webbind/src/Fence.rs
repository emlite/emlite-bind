use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FenceEvent {
    inner: emlite::Val,
}
impl FromVal for FenceEvent {
    fn from_val(v: &emlite::Val) -> Self {
        FenceEvent { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FenceEvent {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FenceEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FenceEvent> for emlite::Val {
    fn from(s: FenceEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FenceEvent {
    pub fn event_type(&self) -> jsbind::DOMString {
        self.inner.get("eventType").as_::<jsbind::DOMString>()
    }

    pub fn set_event_type(&mut self, value: jsbind::DOMString) {
        self.inner.set("eventType", value);
    }
}
impl FenceEvent {
    pub fn event_data(&self) -> jsbind::DOMString {
        self.inner.get("eventData").as_::<jsbind::DOMString>()
    }

    pub fn set_event_data(&mut self, value: jsbind::DOMString) {
        self.inner.set("eventData", value);
    }
}
impl FenceEvent {
    pub fn destination(&self) -> jsbind::Sequence<FenceReportingDestination> {
        self.inner
            .get("destination")
            .as_::<jsbind::Sequence<FenceReportingDestination>>()
    }

    pub fn set_destination(&mut self, value: jsbind::Sequence<FenceReportingDestination>) {
        self.inner.set("destination", value);
    }
}
impl FenceEvent {
    pub fn cross_origin_exposed(&self) -> bool {
        self.inner.get("crossOriginExposed").as_::<bool>()
    }

    pub fn set_cross_origin_exposed(&mut self, value: bool) {
        self.inner.set("crossOriginExposed", value);
    }
}
impl FenceEvent {
    pub fn once(&self) -> bool {
        self.inner.get("once").as_::<bool>()
    }

    pub fn set_once(&mut self, value: bool) {
        self.inner.set("once", value);
    }
}
impl FenceEvent {
    pub fn destination_url(&self) -> jsbind::USVString {
        self.inner.get("destinationURL").as_::<jsbind::USVString>()
    }

    pub fn set_destination_url(&mut self, value: jsbind::USVString) {
        self.inner.set("destinationURL", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Fence {
    inner: emlite::Val,
}
impl FromVal for Fence {
    fn from_val(v: &emlite::Val) -> Self {
        Fence {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Fence {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Fence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Fence> for emlite::Val {
    fn from(s: Fence) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Fence {
    pub fn report_event0(&self) -> jsbind::Undefined {
        self.inner
            .call("reportEvent", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn report_event1(&self, event: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("reportEvent", &[event.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Fence {
    pub fn set_report_event_data_for_automatic_beacons0(&self) -> jsbind::Undefined {
        self.inner
            .call("setReportEventDataForAutomaticBeacons", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn set_report_event_data_for_automatic_beacons1(
        &self,
        event: FenceEvent,
    ) -> jsbind::Undefined {
        self.inner
            .call("setReportEventDataForAutomaticBeacons", &[event.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Fence {
    pub fn get_nested_configs(&self) -> jsbind::Sequence<FencedFrameConfig> {
        self.inner
            .call("getNestedConfigs", &[])
            .as_::<jsbind::Sequence<FencedFrameConfig>>()
    }
}
impl Fence {
    pub fn disable_untrusted_network(&self) -> jsbind::Promise {
        self.inner
            .call("disableUntrustedNetwork", &[])
            .as_::<jsbind::Promise>()
    }
}
impl Fence {
    pub fn notify_event(&self, event: Event) -> jsbind::Undefined {
        self.inner
            .call("notifyEvent", &[event.into()])
            .as_::<jsbind::Undefined>()
    }
}
