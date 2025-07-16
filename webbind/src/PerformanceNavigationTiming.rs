use super::*;

/// The PerformanceNavigationTiming class.
/// [`PerformanceNavigationTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceNavigationTiming {
    inner: PerformanceResourceTiming,
}
impl FromVal for PerformanceNavigationTiming {
    fn from_val(v: &Any) -> Self {
        PerformanceNavigationTiming {
            inner: PerformanceResourceTiming::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for PerformanceNavigationTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PerformanceNavigationTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PerformanceNavigationTiming> for Any {
    fn from(s: PerformanceNavigationTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PerformanceNavigationTiming> for Any {
    fn from(s: &PerformanceNavigationTiming) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceNavigationTiming);

impl PerformanceNavigationTiming {
    /// Getter of the `unloadEventStart` attribute.
    /// [`PerformanceNavigationTiming.unloadEventStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/unloadEventStart)
    pub fn unload_event_start(&self) -> Any {
        self.inner.get("unloadEventStart").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `unloadEventEnd` attribute.
    /// [`PerformanceNavigationTiming.unloadEventEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/unloadEventEnd)
    pub fn unload_event_end(&self) -> Any {
        self.inner.get("unloadEventEnd").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `domInteractive` attribute.
    /// [`PerformanceNavigationTiming.domInteractive`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domInteractive)
    pub fn dom_interactive(&self) -> Any {
        self.inner.get("domInteractive").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `domContentLoadedEventStart` attribute.
    /// [`PerformanceNavigationTiming.domContentLoadedEventStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domContentLoadedEventStart)
    pub fn dom_content_loaded_event_start(&self) -> Any {
        self.inner.get("domContentLoadedEventStart").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `domContentLoadedEventEnd` attribute.
    /// [`PerformanceNavigationTiming.domContentLoadedEventEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domContentLoadedEventEnd)
    pub fn dom_content_loaded_event_end(&self) -> Any {
        self.inner.get("domContentLoadedEventEnd").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `domComplete` attribute.
    /// [`PerformanceNavigationTiming.domComplete`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domComplete)
    pub fn dom_complete(&self) -> Any {
        self.inner.get("domComplete").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `loadEventStart` attribute.
    /// [`PerformanceNavigationTiming.loadEventStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/loadEventStart)
    pub fn load_event_start(&self) -> Any {
        self.inner.get("loadEventStart").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `loadEventEnd` attribute.
    /// [`PerformanceNavigationTiming.loadEventEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/loadEventEnd)
    pub fn load_event_end(&self) -> Any {
        self.inner.get("loadEventEnd").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `type` attribute.
    /// [`PerformanceNavigationTiming.type`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/type)
    pub fn type_(&self) -> NavigationTimingType {
        self.inner.get("type").as_::<NavigationTimingType>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `redirectCount` attribute.
    /// [`PerformanceNavigationTiming.redirectCount`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/redirectCount)
    pub fn redirect_count(&self) -> u16 {
        self.inner.get("redirectCount").as_::<u16>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `criticalCHRestart` attribute.
    /// [`PerformanceNavigationTiming.criticalCHRestart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/criticalCHRestart)
    pub fn critical_ch_restart(&self) -> Any {
        self.inner.get("criticalCHRestart").as_::<Any>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `notRestoredReasons` attribute.
    /// [`PerformanceNavigationTiming.notRestoredReasons`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/notRestoredReasons)
    pub fn not_restored_reasons(&self) -> NotRestoredReasons {
        self.inner
            .get("notRestoredReasons")
            .as_::<NotRestoredReasons>()
    }
}
impl PerformanceNavigationTiming {
    /// The toJSON method.
    /// [`PerformanceNavigationTiming.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl PerformanceNavigationTiming {
    /// Getter of the `activationStart` attribute.
    /// [`PerformanceNavigationTiming.activationStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/activationStart)
    pub fn activation_start(&self) -> Any {
        self.inner.get("activationStart").as_::<Any>()
    }
}
