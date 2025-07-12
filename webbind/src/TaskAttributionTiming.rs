use super::*;

#[derive(Clone, Debug)]
pub struct TaskAttributionTiming {
    inner: PerformanceEntry,
}
impl FromVal for TaskAttributionTiming {
    fn from_val(v: &emlite::Val) -> Self {
        TaskAttributionTiming {
            inner: PerformanceEntry::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TaskAttributionTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TaskAttributionTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TaskAttributionTiming> for emlite::Val {
    fn from(s: TaskAttributionTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TaskAttributionTiming {
    pub fn start_time(&self) -> jsbind::Any {
        self.inner.get("startTime").as_::<jsbind::Any>()
    }
}
impl TaskAttributionTiming {
    pub fn duration(&self) -> jsbind::Any {
        self.inner.get("duration").as_::<jsbind::Any>()
    }
}
impl TaskAttributionTiming {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl TaskAttributionTiming {
    pub fn entry_type(&self) -> jsbind::DOMString {
        self.inner.get("entryType").as_::<jsbind::DOMString>()
    }
}
impl TaskAttributionTiming {
    pub fn container_type(&self) -> jsbind::DOMString {
        self.inner.get("containerType").as_::<jsbind::DOMString>()
    }
}
impl TaskAttributionTiming {
    pub fn container_src(&self) -> jsbind::DOMString {
        self.inner.get("containerSrc").as_::<jsbind::DOMString>()
    }
}
impl TaskAttributionTiming {
    pub fn container_id(&self) -> jsbind::DOMString {
        self.inner.get("containerId").as_::<jsbind::DOMString>()
    }
}
impl TaskAttributionTiming {
    pub fn container_name(&self) -> jsbind::DOMString {
        self.inner.get("containerName").as_::<jsbind::DOMString>()
    }
}
impl TaskAttributionTiming {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
