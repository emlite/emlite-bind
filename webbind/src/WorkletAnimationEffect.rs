use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EffectTiming {
    inner: Any,
}
impl FromVal for EffectTiming {
    fn from_val(v: &Any) -> Self {
        EffectTiming { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for EffectTiming {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EffectTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for EffectTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for EffectTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<EffectTiming> for Any {
    fn from(s: EffectTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&EffectTiming> for Any {
    fn from(s: &EffectTiming) -> Any {
        s.inner.clone()
    }
}

impl EffectTiming {
    pub fn fill(&self) -> FillMode {
        self.inner.get("fill").as_::<FillMode>()
    }

    pub fn set_fill(&mut self, value: &FillMode) {
        self.inner.set("fill", value);
    }
}
impl EffectTiming {
    pub fn iteration_start(&self) -> f64 {
        self.inner.get("iterationStart").as_::<f64>()
    }

    pub fn set_iteration_start(&mut self, value: f64) {
        self.inner.set("iterationStart", value);
    }
}
impl EffectTiming {
    pub fn iterations(&self) -> f64 {
        self.inner.get("iterations").as_::<f64>()
    }

    pub fn set_iterations(&mut self, value: f64) {
        self.inner.set("iterations", value);
    }
}
impl EffectTiming {
    pub fn direction(&self) -> PlaybackDirection {
        self.inner.get("direction").as_::<PlaybackDirection>()
    }

    pub fn set_direction(&mut self, value: &PlaybackDirection) {
        self.inner.set("direction", value);
    }
}
impl EffectTiming {
    pub fn easing(&self) -> DOMString {
        self.inner.get("easing").as_::<DOMString>()
    }

    pub fn set_easing(&mut self, value: &DOMString) {
        self.inner.set("easing", value);
    }
}
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
    pub fn progress(&self) -> f64 {
        self.inner.get("progress").as_::<f64>()
    }

    pub fn set_progress(&mut self, value: f64) {
        self.inner.set("progress", value);
    }
}
impl ComputedEffectTiming {
    pub fn current_iteration(&self) -> f64 {
        self.inner.get("currentIteration").as_::<f64>()
    }

    pub fn set_current_iteration(&mut self, value: f64) {
        self.inner.set("currentIteration", value);
    }
}
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
