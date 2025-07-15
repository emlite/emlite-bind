use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for TaskAttributionTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TaskAttributionTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TaskAttributionTiming {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TaskAttributionTiming {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TaskAttributionTiming> for emlite::Val {
    fn from(s: TaskAttributionTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&TaskAttributionTiming> for emlite::Val {
    fn from(s: &TaskAttributionTiming) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TaskAttributionTiming);

impl TaskAttributionTiming {
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }
}
impl TaskAttributionTiming {
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }
}
impl TaskAttributionTiming {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl TaskAttributionTiming {
    pub fn entry_type(&self) -> String {
        self.inner.get("entryType").as_::<String>()
    }
}
impl TaskAttributionTiming {
    pub fn container_type(&self) -> String {
        self.inner.get("containerType").as_::<String>()
    }
}
impl TaskAttributionTiming {
    pub fn container_src(&self) -> String {
        self.inner.get("containerSrc").as_::<String>()
    }
}
impl TaskAttributionTiming {
    pub fn container_id(&self) -> String {
        self.inner.get("containerId").as_::<String>()
    }
}
impl TaskAttributionTiming {
    pub fn container_name(&self) -> String {
        self.inner.get("containerName").as_::<String>()
    }
}
impl TaskAttributionTiming {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
