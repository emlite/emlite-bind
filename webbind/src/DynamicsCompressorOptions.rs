use super::*;




/// The DynamicsCompressorOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DynamicsCompressorOptions {
    inner: Any,
}

impl FromVal for DynamicsCompressorOptions {
    fn from_val(v: &Any) -> Self {
        DynamicsCompressorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DynamicsCompressorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DynamicsCompressorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DynamicsCompressorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DynamicsCompressorOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DynamicsCompressorOptions> for Any {
    fn from(s: DynamicsCompressorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DynamicsCompressorOptions> for Any {
    fn from(s: &DynamicsCompressorOptions) -> Any {
        s.inner.clone()
    }
}

impl DynamicsCompressorOptions {
    /// Getter of the `attack` attribute.
    pub fn attack(&self) -> f32 {
        self.inner.get("attack").as_::<f32>()
    }

    /// Setter of the `attack` attribute.
    pub fn set_attack(&mut self, value: f32) {
        self.inner.set("attack", value);
    }
}
impl DynamicsCompressorOptions {
    /// Getter of the `knee` attribute.
    pub fn knee(&self) -> f32 {
        self.inner.get("knee").as_::<f32>()
    }

    /// Setter of the `knee` attribute.
    pub fn set_knee(&mut self, value: f32) {
        self.inner.set("knee", value);
    }
}
impl DynamicsCompressorOptions {
    /// Getter of the `ratio` attribute.
    pub fn ratio(&self) -> f32 {
        self.inner.get("ratio").as_::<f32>()
    }

    /// Setter of the `ratio` attribute.
    pub fn set_ratio(&mut self, value: f32) {
        self.inner.set("ratio", value);
    }
}
impl DynamicsCompressorOptions {
    /// Getter of the `release` attribute.
    pub fn release(&self) -> f32 {
        self.inner.get("release").as_::<f32>()
    }

    /// Setter of the `release` attribute.
    pub fn set_release(&mut self, value: f32) {
        self.inner.set("release", value);
    }
}
impl DynamicsCompressorOptions {
    /// Getter of the `threshold` attribute.
    pub fn threshold(&self) -> f32 {
        self.inner.get("threshold").as_::<f32>()
    }

    /// Setter of the `threshold` attribute.
    pub fn set_threshold(&mut self, value: f32) {
        self.inner.set("threshold", value);
    }
}
