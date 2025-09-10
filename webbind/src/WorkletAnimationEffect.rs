use super::*;

/// The WorkletAnimationEffect class.
/// [`WorkletAnimationEffect`](https://developer.mozilla.org/en-US/docs/Web/API/WorkletAnimationEffect)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkletAnimationEffect {
    inner: Any,
}

impl FromVal for WorkletAnimationEffect {
    fn from_val(v: &Any) -> Self {
        WorkletAnimationEffect {
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

impl core::ops::Deref for WorkletAnimationEffect {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WorkletAnimationEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WorkletAnimationEffect {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WorkletAnimationEffect {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WorkletAnimationEffect> for Any {
    fn from(s: WorkletAnimationEffect) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WorkletAnimationEffect> for Any {
    fn from(s: &WorkletAnimationEffect) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WorkletAnimationEffect);

impl WorkletAnimationEffect {
    /// The getTiming method.
    /// [`WorkletAnimationEffect.getTiming`](https://developer.mozilla.org/en-US/docs/Web/API/WorkletAnimationEffect/getTiming)
    pub fn get_timing(&self) -> EffectTiming {
        self.inner.call("getTiming", &[]).as_::<EffectTiming>()
    }
}
impl WorkletAnimationEffect {
    /// The getComputedTiming method.
    /// [`WorkletAnimationEffect.getComputedTiming`](https://developer.mozilla.org/en-US/docs/Web/API/WorkletAnimationEffect/getComputedTiming)
    pub fn get_computed_timing(&self) -> ComputedEffectTiming {
        self.inner
            .call("getComputedTiming", &[])
            .as_::<ComputedEffectTiming>()
    }
}
impl WorkletAnimationEffect {
    /// Getter of the `localTime` attribute.
    /// [`WorkletAnimationEffect.localTime`](https://developer.mozilla.org/en-US/docs/Web/API/WorkletAnimationEffect/localTime)
    pub fn local_time(&self) -> f64 {
        self.inner.get("localTime").as_::<f64>()
    }

    /// Setter of the `localTime` attribute.
    /// [`WorkletAnimationEffect.localTime`](https://developer.mozilla.org/en-US/docs/Web/API/WorkletAnimationEffect/localTime)
    pub fn set_local_time(&mut self, value: f64) {
        self.inner.set("localTime", value);
    }
}
