use super::*;

#[derive(Clone, Debug)]
pub struct PerformanceServerTiming {
    inner: emlite::Val,
}
impl FromVal for PerformanceServerTiming {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceServerTiming {
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
impl std::ops::Deref for PerformanceServerTiming {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PerformanceServerTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PerformanceServerTiming> for emlite::Val {
    fn from(s: PerformanceServerTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PerformanceServerTiming {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl PerformanceServerTiming {
    pub fn duration(&self) -> jsbind::Any {
        self.inner.get("duration").as_::<jsbind::Any>()
    }
}
impl PerformanceServerTiming {
    pub fn description(&self) -> jsbind::DOMString {
        self.inner.get("description").as_::<jsbind::DOMString>()
    }
}
impl PerformanceServerTiming {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
