use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OptionalEffectTiming {
    inner: Any,
}
impl FromVal for OptionalEffectTiming {
    fn from_val(v: &Any) -> Self {
        OptionalEffectTiming { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OptionalEffectTiming {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OptionalEffectTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for OptionalEffectTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OptionalEffectTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OptionalEffectTiming> for Any {
    fn from(s: OptionalEffectTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OptionalEffectTiming> for Any {
    fn from(s: &OptionalEffectTiming) -> Any {
        s.inner.clone()
    }
}

impl OptionalEffectTiming {
    pub fn delay(&self) -> f64 {
        self.inner.get("delay").as_::<f64>()
    }

    pub fn set_delay(&mut self, value: f64) {
        self.inner.set("delay", value);
    }
}
impl OptionalEffectTiming {
    pub fn end_delay(&self) -> f64 {
        self.inner.get("endDelay").as_::<f64>()
    }

    pub fn set_end_delay(&mut self, value: f64) {
        self.inner.set("endDelay", value);
    }
}
impl OptionalEffectTiming {
    pub fn fill(&self) -> FillMode {
        self.inner.get("fill").as_::<FillMode>()
    }

    pub fn set_fill(&mut self, value: &FillMode) {
        self.inner.set("fill", value);
    }
}
impl OptionalEffectTiming {
    pub fn iteration_start(&self) -> f64 {
        self.inner.get("iterationStart").as_::<f64>()
    }

    pub fn set_iteration_start(&mut self, value: f64) {
        self.inner.set("iterationStart", value);
    }
}
impl OptionalEffectTiming {
    pub fn iterations(&self) -> f64 {
        self.inner.get("iterations").as_::<f64>()
    }

    pub fn set_iterations(&mut self, value: f64) {
        self.inner.set("iterations", value);
    }
}
impl OptionalEffectTiming {
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }

    pub fn set_duration(&mut self, value: &Any) {
        self.inner.set("duration", value);
    }
}
impl OptionalEffectTiming {
    pub fn direction(&self) -> PlaybackDirection {
        self.inner.get("direction").as_::<PlaybackDirection>()
    }

    pub fn set_direction(&mut self, value: &PlaybackDirection) {
        self.inner.set("direction", value);
    }
}
impl OptionalEffectTiming {
    pub fn easing(&self) -> DOMString {
        self.inner.get("easing").as_::<DOMString>()
    }

    pub fn set_easing(&mut self, value: &DOMString) {
        self.inner.set("easing", value);
    }
}
/// The AnimationEffect class.
/// [`AnimationEffect`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationEffect {
    inner: Any,
}
impl FromVal for AnimationEffect {
    fn from_val(v: &Any) -> Self {
        AnimationEffect {
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
impl core::ops::Deref for AnimationEffect {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AnimationEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AnimationEffect {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AnimationEffect {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AnimationEffect> for Any {
    fn from(s: AnimationEffect) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AnimationEffect> for Any {
    fn from(s: &AnimationEffect) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AnimationEffect);

impl AnimationEffect {
    /// The getTiming method.
    /// [`AnimationEffect.getTiming`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/getTiming)
    pub fn get_timing(&self) -> EffectTiming {
        self.inner.call("getTiming", &[]).as_::<EffectTiming>()
    }
}
impl AnimationEffect {
    /// The getComputedTiming method.
    /// [`AnimationEffect.getComputedTiming`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/getComputedTiming)
    pub fn get_computed_timing(&self) -> ComputedEffectTiming {
        self.inner
            .call("getComputedTiming", &[])
            .as_::<ComputedEffectTiming>()
    }
}
impl AnimationEffect {
    /// The updateTiming method.
    /// [`AnimationEffect.updateTiming`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/updateTiming)
    pub fn update_timing0(&self) -> Undefined {
        self.inner.call("updateTiming", &[]).as_::<Undefined>()
    }
    /// The updateTiming method.
    /// [`AnimationEffect.updateTiming`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/updateTiming)
    pub fn update_timing1(&self, timing: &OptionalEffectTiming) -> Undefined {
        self.inner
            .call("updateTiming", &[timing.into()])
            .as_::<Undefined>()
    }
}
impl AnimationEffect {
    /// Getter of the `parent` attribute.
    /// [`AnimationEffect.parent`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/parent)
    pub fn parent(&self) -> GroupEffect {
        self.inner.get("parent").as_::<GroupEffect>()
    }
}
impl AnimationEffect {
    /// Getter of the `previousSibling` attribute.
    /// [`AnimationEffect.previousSibling`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/previousSibling)
    pub fn previous_sibling(&self) -> AnimationEffect {
        self.inner.get("previousSibling").as_::<AnimationEffect>()
    }
}
impl AnimationEffect {
    /// Getter of the `nextSibling` attribute.
    /// [`AnimationEffect.nextSibling`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/nextSibling)
    pub fn next_sibling(&self) -> AnimationEffect {
        self.inner.get("nextSibling").as_::<AnimationEffect>()
    }
}
impl AnimationEffect {
    /// The before method.
    /// [`AnimationEffect.before`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/before)
    pub fn before(&self, effects: &AnimationEffect) -> Undefined {
        self.inner
            .call("before", &[effects.into()])
            .as_::<Undefined>()
    }
}
impl AnimationEffect {
    /// The after method.
    /// [`AnimationEffect.after`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/after)
    pub fn after(&self, effects: &AnimationEffect) -> Undefined {
        self.inner
            .call("after", &[effects.into()])
            .as_::<Undefined>()
    }
}
impl AnimationEffect {
    /// The replace method.
    /// [`AnimationEffect.replace`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/replace)
    pub fn replace(&self, effects: &AnimationEffect) -> Undefined {
        self.inner
            .call("replace", &[effects.into()])
            .as_::<Undefined>()
    }
}
impl AnimationEffect {
    /// The remove method.
    /// [`AnimationEffect.remove`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/remove)
    pub fn remove(&self) -> Undefined {
        self.inner.call("remove", &[]).as_::<Undefined>()
    }
}
