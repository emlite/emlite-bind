use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TaskSignalAnyInit {
    inner: emlite::Val,
}
impl FromVal for TaskSignalAnyInit {
    fn from_val(v: &emlite::Val) -> Self {
        TaskSignalAnyInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TaskSignalAnyInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TaskSignalAnyInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TaskSignalAnyInit> for emlite::Val {
    fn from(s: TaskSignalAnyInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TaskSignalAnyInit {
    pub fn priority(&self) -> jsbind::Any {
        self.inner.get("priority").as_::<jsbind::Any>()
    }

    pub fn set_priority(&mut self, value: jsbind::Any) {
        self.inner.set("priority", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TaskSignal {
    inner: AbortSignal,
}
impl FromVal for TaskSignal {
    fn from_val(v: &emlite::Val) -> Self {
        TaskSignal {
            inner: AbortSignal::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TaskSignal {
    type Target = AbortSignal;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TaskSignal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TaskSignal> for emlite::Val {
    fn from(s: TaskSignal) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TaskSignal {
    pub fn any0(signals: jsbind::Sequence<AbortSignal>) -> TaskSignal {
        emlite::Val::global("tasksignal")
            .call("any", &[signals.into()])
            .as_::<TaskSignal>()
    }

    pub fn any1(signals: jsbind::Sequence<AbortSignal>, init: TaskSignalAnyInit) -> TaskSignal {
        emlite::Val::global("tasksignal")
            .call("any", &[signals.into(), init.into()])
            .as_::<TaskSignal>()
    }
}
impl TaskSignal {
    pub fn priority(&self) -> TaskPriority {
        self.inner.get("priority").as_::<TaskPriority>()
    }
}
impl TaskSignal {
    pub fn onprioritychange(&self) -> jsbind::Any {
        self.inner.get("onprioritychange").as_::<jsbind::Any>()
    }

    pub fn set_onprioritychange(&mut self, value: jsbind::Any) {
        self.inner.set("onprioritychange", value);
    }
}
