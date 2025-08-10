use super::*;

/// The EffectTiming dictionary.
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
    /// Getter of the `fill` attribute.
    pub fn fill(&self) -> FillMode {
        self.inner.get("fill").as_::<FillMode>()
    }

    /// Setter of the `fill` attribute.
    pub fn set_fill(&mut self, value: &FillMode) {
        self.inner.set("fill", value);
    }
}
impl EffectTiming {
    /// Getter of the `iterationStart` attribute.
    pub fn iteration_start(&self) -> f64 {
        self.inner.get("iterationStart").as_::<f64>()
    }

    /// Setter of the `iterationStart` attribute.
    pub fn set_iteration_start(&mut self, value: f64) {
        self.inner.set("iterationStart", value);
    }
}
impl EffectTiming {
    /// Getter of the `iterations` attribute.
    pub fn iterations(&self) -> f64 {
        self.inner.get("iterations").as_::<f64>()
    }

    /// Setter of the `iterations` attribute.
    pub fn set_iterations(&mut self, value: f64) {
        self.inner.set("iterations", value);
    }
}
impl EffectTiming {
    /// Getter of the `direction` attribute.
    pub fn direction(&self) -> PlaybackDirection {
        self.inner.get("direction").as_::<PlaybackDirection>()
    }

    /// Setter of the `direction` attribute.
    pub fn set_direction(&mut self, value: &PlaybackDirection) {
        self.inner.set("direction", value);
    }
}
impl EffectTiming {
    /// Getter of the `easing` attribute.
    pub fn easing(&self) -> JsString {
        self.inner.get("easing").as_::<JsString>()
    }

    /// Setter of the `easing` attribute.
    pub fn set_easing(&mut self, value: &JsString) {
        self.inner.set("easing", value);
    }
}
