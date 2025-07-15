use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceTiming {
    inner: emlite::Val,
}
impl FromVal for PerformanceTiming {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceTiming {
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
impl core::ops::Deref for PerformanceTiming {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformanceTiming {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceTiming {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PerformanceTiming> for emlite::Val {
    fn from(s: PerformanceTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PerformanceTiming> for emlite::Val {
    fn from(s: &PerformanceTiming) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceTiming);

impl PerformanceTiming {
    pub fn navigation_start(&self) -> u64 {
        self.inner.get("navigationStart").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn unload_event_start(&self) -> u64 {
        self.inner.get("unloadEventStart").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn unload_event_end(&self) -> u64 {
        self.inner.get("unloadEventEnd").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn redirect_start(&self) -> u64 {
        self.inner.get("redirectStart").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn redirect_end(&self) -> u64 {
        self.inner.get("redirectEnd").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn fetch_start(&self) -> u64 {
        self.inner.get("fetchStart").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn domain_lookup_start(&self) -> u64 {
        self.inner.get("domainLookupStart").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn domain_lookup_end(&self) -> u64 {
        self.inner.get("domainLookupEnd").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn connect_start(&self) -> u64 {
        self.inner.get("connectStart").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn connect_end(&self) -> u64 {
        self.inner.get("connectEnd").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn secure_connection_start(&self) -> u64 {
        self.inner.get("secureConnectionStart").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn request_start(&self) -> u64 {
        self.inner.get("requestStart").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn response_start(&self) -> u64 {
        self.inner.get("responseStart").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn response_end(&self) -> u64 {
        self.inner.get("responseEnd").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn dom_loading(&self) -> u64 {
        self.inner.get("domLoading").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn dom_interactive(&self) -> u64 {
        self.inner.get("domInteractive").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn dom_content_loaded_event_start(&self) -> u64 {
        self.inner.get("domContentLoadedEventStart").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn dom_content_loaded_event_end(&self) -> u64 {
        self.inner.get("domContentLoadedEventEnd").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn dom_complete(&self) -> u64 {
        self.inner.get("domComplete").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn load_event_start(&self) -> u64 {
        self.inner.get("loadEventStart").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn load_event_end(&self) -> u64 {
        self.inner.get("loadEventEnd").as_::<u64>()
    }
}
impl PerformanceTiming {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
