use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OptionalEffectTiming {
    inner: emlite::Val,
}
impl FromVal for OptionalEffectTiming {
    fn from_val(v: &emlite::Val) -> Self {
        OptionalEffectTiming { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OptionalEffectTiming {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OptionalEffectTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for OptionalEffectTiming {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for OptionalEffectTiming {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<OptionalEffectTiming> for emlite::Val {
    fn from(s: OptionalEffectTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
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

    pub fn set_fill(&mut self, value: FillMode) {
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
    pub fn duration(&self) -> jsbind::Any {
        self.inner.get("duration").as_::<jsbind::Any>()
    }

    pub fn set_duration(&mut self, value: jsbind::Any) {
        self.inner.set("duration", value);
    }
}
impl OptionalEffectTiming {
    pub fn direction(&self) -> PlaybackDirection {
        self.inner.get("direction").as_::<PlaybackDirection>()
    }

    pub fn set_direction(&mut self, value: PlaybackDirection) {
        self.inner.set("direction", value);
    }
}
impl OptionalEffectTiming {
    pub fn easing(&self) -> jsbind::DOMString {
        self.inner.get("easing").as_::<jsbind::DOMString>()
    }

    pub fn set_easing(&mut self, value: jsbind::DOMString) {
        self.inner.set("easing", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationEffect {
    inner: emlite::Val,
}
impl FromVal for AnimationEffect {
    fn from_val(v: &emlite::Val) -> Self {
        AnimationEffect {
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
impl core::ops::Deref for AnimationEffect {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AnimationEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AnimationEffect {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AnimationEffect {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AnimationEffect> for emlite::Val {
    fn from(s: AnimationEffect) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AnimationEffect);

impl AnimationEffect {
    pub fn get_timing(&self) -> EffectTiming {
        self.inner.call("getTiming", &[]).as_::<EffectTiming>()
    }
}
impl AnimationEffect {
    pub fn get_computed_timing(&self) -> ComputedEffectTiming {
        self.inner
            .call("getComputedTiming", &[])
            .as_::<ComputedEffectTiming>()
    }
}
impl AnimationEffect {
    pub fn update_timing0(&self) -> jsbind::Undefined {
        self.inner
            .call("updateTiming", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn update_timing1(&self, timing: OptionalEffectTiming) -> jsbind::Undefined {
        self.inner
            .call("updateTiming", &[timing.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AnimationEffect {
    pub fn parent(&self) -> GroupEffect {
        self.inner.get("parent").as_::<GroupEffect>()
    }
}
impl AnimationEffect {
    pub fn previous_sibling(&self) -> AnimationEffect {
        self.inner.get("previousSibling").as_::<AnimationEffect>()
    }
}
impl AnimationEffect {
    pub fn next_sibling(&self) -> AnimationEffect {
        self.inner.get("nextSibling").as_::<AnimationEffect>()
    }
}
impl AnimationEffect {
    pub fn before(&self, effects: AnimationEffect) -> jsbind::Undefined {
        self.inner
            .call("before", &[effects.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AnimationEffect {
    pub fn after(&self, effects: AnimationEffect) -> jsbind::Undefined {
        self.inner
            .call("after", &[effects.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AnimationEffect {
    pub fn replace(&self, effects: AnimationEffect) -> jsbind::Undefined {
        self.inner
            .call("replace", &[effects.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AnimationEffect {
    pub fn remove(&self) -> jsbind::Undefined {
        self.inner.call("remove", &[]).as_::<jsbind::Undefined>()
    }
}
