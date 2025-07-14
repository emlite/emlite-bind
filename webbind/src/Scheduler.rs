use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SchedulerPostTaskOptions {
    inner: emlite::Val,
}
impl FromVal for SchedulerPostTaskOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SchedulerPostTaskOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SchedulerPostTaskOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SchedulerPostTaskOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SchedulerPostTaskOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SchedulerPostTaskOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SchedulerPostTaskOptions> for emlite::Val {
    fn from(s: SchedulerPostTaskOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SchedulerPostTaskOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl SchedulerPostTaskOptions {
    pub fn priority(&self) -> TaskPriority {
        self.inner.get("priority").as_::<TaskPriority>()
    }

    pub fn set_priority(&mut self, value: TaskPriority) {
        self.inner.set("priority", value);
    }
}
impl SchedulerPostTaskOptions {
    pub fn delay(&self) -> u64 {
        self.inner.get("delay").as_::<u64>()
    }

    pub fn set_delay(&mut self, value: u64) {
        self.inner.set("delay", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Scheduler {
    inner: emlite::Val,
}
impl FromVal for Scheduler {
    fn from_val(v: &emlite::Val) -> Self {
        Scheduler {
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
impl core::ops::Deref for Scheduler {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Scheduler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Scheduler {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Scheduler {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Scheduler> for emlite::Val {
    fn from(s: Scheduler) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Scheduler);

impl Scheduler {
    pub fn post_task0(&self, callback: jsbind::Function) -> jsbind::Promise {
        self.inner
            .call("postTask", &[callback.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn post_task1(
        &self,
        callback: jsbind::Function,
        options: SchedulerPostTaskOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("postTask", &[callback.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Scheduler {
    pub fn yield_(&self) -> jsbind::Promise {
        self.inner.call("yield", &[]).as_::<jsbind::Promise>()
    }
}
