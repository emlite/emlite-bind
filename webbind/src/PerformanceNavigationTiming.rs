use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for PerformanceNavigationTiming {
    type Target = PerformanceResourceTiming;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PerformanceNavigationTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PerformanceNavigationTiming> for emlite::Val {
    fn from(s: PerformanceNavigationTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PerformanceNavigationTiming {
    pub fn unload_event_start(&self) -> jsbind::Any {
        self.inner.get("unloadEventStart").as_::<jsbind::Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn unload_event_end(&self) -> jsbind::Any {
        self.inner.get("unloadEventEnd").as_::<jsbind::Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn dom_interactive(&self) -> jsbind::Any {
        self.inner.get("domInteractive").as_::<jsbind::Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn dom_content_loaded_event_start(&self) -> jsbind::Any {
        self.inner
            .get("domContentLoadedEventStart")
            .as_::<jsbind::Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn dom_content_loaded_event_end(&self) -> jsbind::Any {
        self.inner
            .get("domContentLoadedEventEnd")
            .as_::<jsbind::Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn dom_complete(&self) -> jsbind::Any {
        self.inner.get("domComplete").as_::<jsbind::Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn load_event_start(&self) -> jsbind::Any {
        self.inner.get("loadEventStart").as_::<jsbind::Any>()
    }
}
impl PerformanceNavigationTiming {
    pub fn load_event_end(&self) -> jsbind::Any {
        self.inner.get("loadEventEnd").as_::<jsbind::Any>()
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
    pub fn critical_ch_restart(&self) -> jsbind::Any {
        self.inner.get("criticalCHRestart").as_::<jsbind::Any>()
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
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
impl PerformanceNavigationTiming {
    pub fn activation_start(&self) -> jsbind::Any {
        self.inner.get("activationStart").as_::<jsbind::Any>()
    }
}
