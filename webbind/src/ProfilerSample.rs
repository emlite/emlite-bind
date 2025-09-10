use super::*;

/// The ProfilerSample dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProfilerSample {
    inner: Any,
}

impl FromVal for ProfilerSample {
    fn from_val(v: &Any) -> Self {
        ProfilerSample { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ProfilerSample {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ProfilerSample {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ProfilerSample {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ProfilerSample {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ProfilerSample> for Any {
    fn from(s: ProfilerSample) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ProfilerSample> for Any {
    fn from(s: &ProfilerSample) -> Any {
        s.inner.clone()
    }
}

impl ProfilerSample {
    /// Getter of the `timestamp` attribute.
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }

    /// Setter of the `timestamp` attribute.
    pub fn set_timestamp(&mut self, value: &Any) {
        self.inner.set("timestamp", value);
    }
}
impl ProfilerSample {
    /// Getter of the `stackId` attribute.
    pub fn stack_id(&self) -> u64 {
        self.inner.get("stackId").as_::<u64>()
    }

    /// Setter of the `stackId` attribute.
    pub fn set_stack_id(&mut self, value: u64) {
        self.inner.set("stackId", value);
    }
}
