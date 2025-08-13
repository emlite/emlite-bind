use super::*;




/// The PerformanceTiming class.
/// [`PerformanceTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceTiming {
    inner: Any,
}

impl FromVal for PerformanceTiming {
    fn from_val(v: &Any) -> Self {
        PerformanceTiming { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PerformanceTiming {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PerformanceTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PerformanceTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PerformanceTiming {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PerformanceTiming> for Any {
    fn from(s: PerformanceTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PerformanceTiming> for Any {
    fn from(s: &PerformanceTiming) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PerformanceTiming);


impl PerformanceTiming {
    /// Getter of the `navigationStart` attribute.
    /// [`PerformanceTiming.navigationStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/navigationStart)
    pub fn navigation_start(&self) -> u64 {
        self.inner.get("navigationStart").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `unloadEventStart` attribute.
    /// [`PerformanceTiming.unloadEventStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventStart)
    pub fn unload_event_start(&self) -> u64 {
        self.inner.get("unloadEventStart").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `unloadEventEnd` attribute.
    /// [`PerformanceTiming.unloadEventEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventEnd)
    pub fn unload_event_end(&self) -> u64 {
        self.inner.get("unloadEventEnd").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `redirectStart` attribute.
    /// [`PerformanceTiming.redirectStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectStart)
    pub fn redirect_start(&self) -> u64 {
        self.inner.get("redirectStart").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `redirectEnd` attribute.
    /// [`PerformanceTiming.redirectEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectEnd)
    pub fn redirect_end(&self) -> u64 {
        self.inner.get("redirectEnd").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `fetchStart` attribute.
    /// [`PerformanceTiming.fetchStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/fetchStart)
    pub fn fetch_start(&self) -> u64 {
        self.inner.get("fetchStart").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `domainLookupStart` attribute.
    /// [`PerformanceTiming.domainLookupStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupStart)
    pub fn domain_lookup_start(&self) -> u64 {
        self.inner.get("domainLookupStart").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `domainLookupEnd` attribute.
    /// [`PerformanceTiming.domainLookupEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupEnd)
    pub fn domain_lookup_end(&self) -> u64 {
        self.inner.get("domainLookupEnd").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `connectStart` attribute.
    /// [`PerformanceTiming.connectStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectStart)
    pub fn connect_start(&self) -> u64 {
        self.inner.get("connectStart").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `connectEnd` attribute.
    /// [`PerformanceTiming.connectEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectEnd)
    pub fn connect_end(&self) -> u64 {
        self.inner.get("connectEnd").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `secureConnectionStart` attribute.
    /// [`PerformanceTiming.secureConnectionStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/secureConnectionStart)
    pub fn secure_connection_start(&self) -> u64 {
        self.inner.get("secureConnectionStart").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `requestStart` attribute.
    /// [`PerformanceTiming.requestStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/requestStart)
    pub fn request_start(&self) -> u64 {
        self.inner.get("requestStart").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `responseStart` attribute.
    /// [`PerformanceTiming.responseStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseStart)
    pub fn response_start(&self) -> u64 {
        self.inner.get("responseStart").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `responseEnd` attribute.
    /// [`PerformanceTiming.responseEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseEnd)
    pub fn response_end(&self) -> u64 {
        self.inner.get("responseEnd").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `domLoading` attribute.
    /// [`PerformanceTiming.domLoading`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domLoading)
    pub fn dom_loading(&self) -> u64 {
        self.inner.get("domLoading").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `domInteractive` attribute.
    /// [`PerformanceTiming.domInteractive`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domInteractive)
    pub fn dom_interactive(&self) -> u64 {
        self.inner.get("domInteractive").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `domContentLoadedEventStart` attribute.
    /// [`PerformanceTiming.domContentLoadedEventStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventStart)
    pub fn dom_content_loaded_event_start(&self) -> u64 {
        self.inner.get("domContentLoadedEventStart").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `domContentLoadedEventEnd` attribute.
    /// [`PerformanceTiming.domContentLoadedEventEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventEnd)
    pub fn dom_content_loaded_event_end(&self) -> u64 {
        self.inner.get("domContentLoadedEventEnd").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `domComplete` attribute.
    /// [`PerformanceTiming.domComplete`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domComplete)
    pub fn dom_complete(&self) -> u64 {
        self.inner.get("domComplete").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `loadEventStart` attribute.
    /// [`PerformanceTiming.loadEventStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventStart)
    pub fn load_event_start(&self) -> u64 {
        self.inner.get("loadEventStart").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// Getter of the `loadEventEnd` attribute.
    /// [`PerformanceTiming.loadEventEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventEnd)
    pub fn load_event_end(&self) -> u64 {
        self.inner.get("loadEventEnd").as_::<u64>()
    }

}
impl PerformanceTiming {
    /// The toJSON method.
    /// [`PerformanceTiming.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/toJSON)
    pub fn to_json(&self, ) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
