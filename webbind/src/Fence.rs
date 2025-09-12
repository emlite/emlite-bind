use super::*;

/// The Fence class.
/// [`Fence`](https://developer.mozilla.org/en-US/docs/Web/API/Fence)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Fence {
    inner: Any,
}

impl FromVal for Fence {
    fn from_val(v: &Any) -> Self {
        Fence {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Fence {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Fence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Fence {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Fence {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Fence> for Any {
    fn from(s: Fence) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Fence> for Any {
    fn from(s: &Fence) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Fence);

impl Fence {
    /// The reportEvent method.
    /// [`Fence.reportEvent`](https://developer.mozilla.org/en-US/docs/Web/API/Fence/reportEvent)
    pub fn report_event(&self) -> Undefined {
        self.inner.call("reportEvent", &[]).as_::<Undefined>()
    }
}
impl Fence {
    /// The reportEvent method.
    /// [`Fence.reportEvent`](https://developer.mozilla.org/en-US/docs/Web/API/Fence/reportEvent)
    pub fn report_event_with_event(&self, event: &Any) -> Undefined {
        self.inner
            .call("reportEvent", &[event.into()])
            .as_::<Undefined>()
    }
}
impl Fence {
    /// The setReportEventDataForAutomaticBeacons method.
    /// [`Fence.setReportEventDataForAutomaticBeacons`](https://developer.mozilla.org/en-US/docs/Web/API/Fence/setReportEventDataForAutomaticBeacons)
    pub fn set_report_event_data_for_automatic_beacons(&self) -> Undefined {
        self.inner
            .call("setReportEventDataForAutomaticBeacons", &[])
            .as_::<Undefined>()
    }
}
impl Fence {
    /// The setReportEventDataForAutomaticBeacons method.
    /// [`Fence.setReportEventDataForAutomaticBeacons`](https://developer.mozilla.org/en-US/docs/Web/API/Fence/setReportEventDataForAutomaticBeacons)
    pub fn set_report_event_data_for_automatic_beacons_with_event(
        &self,
        event: &FenceEvent,
    ) -> Undefined {
        self.inner
            .call("setReportEventDataForAutomaticBeacons", &[event.into()])
            .as_::<Undefined>()
    }
}
impl Fence {
    /// The getNestedConfigs method.
    /// [`Fence.getNestedConfigs`](https://developer.mozilla.org/en-US/docs/Web/API/Fence/getNestedConfigs)
    pub fn get_nested_configs(&self) -> TypedArray<FencedFrameConfig> {
        self.inner
            .call("getNestedConfigs", &[])
            .as_::<TypedArray<FencedFrameConfig>>()
    }
}
impl Fence {
    /// The disableUntrustedNetwork method.
    /// [`Fence.disableUntrustedNetwork`](https://developer.mozilla.org/en-US/docs/Web/API/Fence/disableUntrustedNetwork)
    pub fn disable_untrusted_network(&self) -> Promise<Undefined> {
        self.inner
            .call("disableUntrustedNetwork", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl Fence {
    /// The notifyEvent method.
    /// [`Fence.notifyEvent`](https://developer.mozilla.org/en-US/docs/Web/API/Fence/notifyEvent)
    pub fn notify_event(&self, event: &Event) -> Undefined {
        self.inner
            .call("notifyEvent", &[event.into()])
            .as_::<Undefined>()
    }
}
