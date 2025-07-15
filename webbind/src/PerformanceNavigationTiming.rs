use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceNavigationTiming {
    inner: PerformanceResourceTiming,
}
impl FromVal for PerformanceNavigationTiming {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceNavigationTiming {
            inner: PerformanceResourceTiming::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformanceNavigationTiming {
    type Target = PerformanceResourceTiming;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceNavigationTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformanceNavigationTiming {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceNavigationTiming {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PerformanceNavigationTiming> for emlite::Val {
    fn from(s: PerformanceNavigationTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PerformanceNavigationTiming> for emlite::Val {
    fn from(s: &PerformanceNavigationTiming) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceNavigationTiming);

impl PerformanceNavigationTiming {
    pub fn unload_event_start(&self) -> Any {
        self.inner.get("unloadEventStart").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn unload_event_end(&self) -> Any {
        self.inner.get("unloadEventEnd").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn dom_interactive(&self) -> Any {
        self.inner.get("domInteractive").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn dom_content_loaded_event_start(&self) -> Any {
        self.inner.get("domContentLoadedEventStart").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn dom_content_loaded_event_end(&self) -> Any {
        self.inner.get("domContentLoadedEventEnd").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn dom_complete(&self) -> Any {
        self.inner.get("domComplete").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn load_event_start(&self) -> Any {
        self.inner.get("loadEventStart").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn load_event_end(&self) -> Any {
        self.inner.get("loadEventEnd").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn type_(&self) -> NavigationTimingType {
        self.inner.get("type").as_::<NavigationTimingType>()
    }
}
impl PerformanceNavigationTiming {
    pub fn redirect_count(&self) -> u16 {
        self.inner.get("redirectCount").as_::<u16>()
    }
}
impl PerformanceNavigationTiming {
    pub fn critical_ch_restart(&self) -> Any {
        self.inner.get("criticalCHRestart").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn not_restored_reasons(&self) -> NotRestoredReasons {
        self.inner
            .get("notRestoredReasons")
            .as_::<NotRestoredReasons>()
    }
}
impl PerformanceNavigationTiming {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl PerformanceNavigationTiming {
    pub fn activation_start(&self) -> Any {
        self.inner.get("activationStart").as_::<Any>()
    }
}
