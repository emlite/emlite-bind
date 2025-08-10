use super::*;

/// The DelayOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DelayOptions {
    inner: Any,
}

impl FromVal for DelayOptions {
    fn from_val(v: &Any) -> Self {
        DelayOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DelayOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DelayOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DelayOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DelayOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DelayOptions> for Any {
    fn from(s: DelayOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DelayOptions> for Any {
    fn from(s: &DelayOptions) -> Any {
        s.inner.clone()
    }
}

impl DelayOptions {
    /// Getter of the `maxDelayTime` attribute.
    pub fn max_delay_time(&self) -> f64 {
        self.inner.get("maxDelayTime").as_::<f64>()
    }

    /// Setter of the `maxDelayTime` attribute.
    pub fn set_max_delay_time(&mut self, value: f64) {
        self.inner.set("maxDelayTime", value);
    }
}
impl DelayOptions {
    /// Getter of the `delayTime` attribute.
    pub fn delay_time(&self) -> f64 {
        self.inner.get("delayTime").as_::<f64>()
    }

    /// Setter of the `delayTime` attribute.
    pub fn set_delay_time(&mut self, value: f64) {
        self.inner.set("delayTime", value);
    }
}
