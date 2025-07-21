use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SchedulerPostTaskOptions {
    inner: Any,
}
impl FromVal for SchedulerPostTaskOptions {
    fn from_val(v: &Any) -> Self {
        SchedulerPostTaskOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SchedulerPostTaskOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SchedulerPostTaskOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SchedulerPostTaskOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SchedulerPostTaskOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SchedulerPostTaskOptions> for Any {
    fn from(s: SchedulerPostTaskOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SchedulerPostTaskOptions> for Any {
    fn from(s: &SchedulerPostTaskOptions) -> Any {
        s.inner.clone()
    }
}

impl SchedulerPostTaskOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl SchedulerPostTaskOptions {
    pub fn priority(&self) -> TaskPriority {
        self.inner.get("priority").as_::<TaskPriority>()
    }

    pub fn set_priority(&mut self, value: &TaskPriority) {
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
/// The Scheduler class.
/// [`Scheduler`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Scheduler {
    inner: Any,
}
impl FromVal for Scheduler {
    fn from_val(v: &Any) -> Self {
        Scheduler {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Scheduler {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Scheduler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Scheduler {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Scheduler {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Scheduler> for Any {
    fn from(s: Scheduler) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Scheduler> for Any {
    fn from(s: &Scheduler) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Scheduler);

impl Scheduler {
    /// The postTask method.
    /// [`Scheduler.postTask`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask)
    pub fn post_task0(&self, callback: &Function) -> Promise<Any> {
        self.inner
            .call("postTask", &[callback.into()])
            .as_::<Promise<Any>>()
    }
    /// The postTask method.
    /// [`Scheduler.postTask`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask)
    pub fn post_task1(
        &self,
        callback: &Function,
        options: &SchedulerPostTaskOptions,
    ) -> Promise<Any> {
        self.inner
            .call("postTask", &[callback.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
impl Scheduler {
    /// The yield method.
    /// [`Scheduler.yield`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/yield)
    pub fn yield_(&self) -> Promise<Undefined> {
        self.inner.call("yield", &[]).as_::<Promise<Undefined>>()
    }
}
