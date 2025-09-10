use super::*;

/// The SchedulerPostTaskOptions dictionary.
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
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl SchedulerPostTaskOptions {
    /// Getter of the `priority` attribute.
    pub fn priority(&self) -> TaskPriority {
        self.inner.get("priority").as_::<TaskPriority>()
    }

    /// Setter of the `priority` attribute.
    pub fn set_priority(&mut self, value: &TaskPriority) {
        self.inner.set("priority", value);
    }
}
impl SchedulerPostTaskOptions {
    /// Getter of the `delay` attribute.
    pub fn delay(&self) -> u64 {
        self.inner.get("delay").as_::<u64>()
    }

    /// Setter of the `delay` attribute.
    pub fn set_delay(&mut self, value: u64) {
        self.inner.set("delay", value);
    }
}
