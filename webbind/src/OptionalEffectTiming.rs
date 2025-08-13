use super::*;




/// The OptionalEffectTiming dictionary.
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
    /// Getter of the `delay` attribute.
    pub fn delay(&self) -> f64 {
        self.inner.get("delay").as_::<f64>()
    }

    /// Setter of the `delay` attribute.
    pub fn set_delay(&mut self, value: f64) {
        self.inner.set("delay", value);
    }
}
impl OptionalEffectTiming {
    /// Getter of the `endDelay` attribute.
    pub fn end_delay(&self) -> f64 {
        self.inner.get("endDelay").as_::<f64>()
    }

    /// Setter of the `endDelay` attribute.
    pub fn set_end_delay(&mut self, value: f64) {
        self.inner.set("endDelay", value);
    }
}
impl OptionalEffectTiming {
    /// Getter of the `fill` attribute.
    pub fn fill(&self) -> FillMode {
        self.inner.get("fill").as_::<FillMode>()
    }

    /// Setter of the `fill` attribute.
    pub fn set_fill(&mut self, value: &FillMode) {
        self.inner.set("fill", value);
    }
}
impl OptionalEffectTiming {
    /// Getter of the `iterationStart` attribute.
    pub fn iteration_start(&self) -> f64 {
        self.inner.get("iterationStart").as_::<f64>()
    }

    /// Setter of the `iterationStart` attribute.
    pub fn set_iteration_start(&mut self, value: f64) {
        self.inner.set("iterationStart", value);
    }
}
impl OptionalEffectTiming {
    /// Getter of the `iterations` attribute.
    pub fn iterations(&self) -> f64 {
        self.inner.get("iterations").as_::<f64>()
    }

    /// Setter of the `iterations` attribute.
    pub fn set_iterations(&mut self, value: f64) {
        self.inner.set("iterations", value);
    }
}
impl OptionalEffectTiming {
    /// Getter of the `duration` attribute.
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }

    /// Setter of the `duration` attribute.
    pub fn set_duration(&mut self, value: &Any) {
        self.inner.set("duration", value);
    }
}
impl OptionalEffectTiming {
    /// Getter of the `direction` attribute.
    pub fn direction(&self) -> PlaybackDirection {
        self.inner.get("direction").as_::<PlaybackDirection>()
    }

    /// Setter of the `direction` attribute.
    pub fn set_direction(&mut self, value: &PlaybackDirection) {
        self.inner.set("direction", value);
    }
}
impl OptionalEffectTiming {
    /// Getter of the `easing` attribute.
    pub fn easing(&self) -> JsString {
        self.inner.get("easing").as_::<JsString>()
    }

    /// Setter of the `easing` attribute.
    pub fn set_easing(&mut self, value: &JsString) {
        self.inner.set("easing", value);
    }
}
