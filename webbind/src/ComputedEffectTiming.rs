use super::*;

/// The ComputedEffectTiming dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ComputedEffectTiming {
    inner: Any,
}

impl FromVal for ComputedEffectTiming {
    fn from_val(v: &Any) -> Self {
        ComputedEffectTiming { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ComputedEffectTiming {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ComputedEffectTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ComputedEffectTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ComputedEffectTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ComputedEffectTiming> for Any {
    fn from(s: ComputedEffectTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ComputedEffectTiming> for Any {
    fn from(s: &ComputedEffectTiming) -> Any {
        s.inner.clone()
    }
}

impl ComputedEffectTiming {
    /// Getter of the `progress` attribute.
    pub fn progress(&self) -> f64 {
        self.inner.get("progress").as_::<f64>()
    }

    /// Setter of the `progress` attribute.
    pub fn set_progress(&mut self, value: f64) {
        self.inner.set("progress", value);
    }
}
impl ComputedEffectTiming {
    /// Getter of the `currentIteration` attribute.
    pub fn current_iteration(&self) -> f64 {
        self.inner.get("currentIteration").as_::<f64>()
    }

    /// Setter of the `currentIteration` attribute.
    pub fn set_current_iteration(&mut self, value: f64) {
        self.inner.set("currentIteration", value);
    }
}
