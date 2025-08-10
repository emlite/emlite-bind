use super::*;

/// The QueuingStrategy dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct QueuingStrategy {
    inner: Any,
}

impl FromVal for QueuingStrategy {
    fn from_val(v: &Any) -> Self {
        QueuingStrategy { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for QueuingStrategy {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for QueuingStrategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for QueuingStrategy {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for QueuingStrategy {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<QueuingStrategy> for Any {
    fn from(s: QueuingStrategy) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&QueuingStrategy> for Any {
    fn from(s: &QueuingStrategy) -> Any {
        s.inner.clone()
    }
}

impl QueuingStrategy {
    /// Getter of the `highWaterMark` attribute.
    pub fn high_water_mark(&self) -> f64 {
        self.inner.get("highWaterMark").as_::<f64>()
    }

    /// Setter of the `highWaterMark` attribute.
    pub fn set_high_water_mark(&mut self, value: f64) {
        self.inner.set("highWaterMark", value);
    }
}
impl QueuingStrategy {
    /// Getter of the `size` attribute.
    pub fn size(&self) -> Function {
        self.inner.get("size").as_::<Function>()
    }

    /// Setter of the `size` attribute.
    pub fn set_size(&mut self, value: &Function) {
        self.inner.set("size", value);
    }
}
