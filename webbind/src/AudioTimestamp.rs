use super::*;

/// The AudioTimestamp dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioTimestamp {
    inner: Any,
}

impl FromVal for AudioTimestamp {
    fn from_val(v: &Any) -> Self {
        AudioTimestamp { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioTimestamp {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioTimestamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioTimestamp {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioTimestamp {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioTimestamp> for Any {
    fn from(s: AudioTimestamp) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioTimestamp> for Any {
    fn from(s: &AudioTimestamp) -> Any {
        s.inner.clone()
    }
}

impl AudioTimestamp {
    /// Getter of the `contextTime` attribute.
    pub fn context_time(&self) -> f64 {
        self.inner.get("contextTime").as_::<f64>()
    }

    /// Setter of the `contextTime` attribute.
    pub fn set_context_time(&mut self, value: f64) {
        self.inner.set("contextTime", value);
    }
}
impl AudioTimestamp {
    /// Getter of the `performanceTime` attribute.
    pub fn performance_time(&self) -> Any {
        self.inner.get("performanceTime").as_::<Any>()
    }

    /// Setter of the `performanceTime` attribute.
    pub fn set_performance_time(&mut self, value: &Any) {
        self.inner.set("performanceTime", value);
    }
}
