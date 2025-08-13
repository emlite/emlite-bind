use super::*;




/// The AnalyserOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnalyserOptions {
    inner: Any,
}

impl FromVal for AnalyserOptions {
    fn from_val(v: &Any) -> Self {
        AnalyserOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AnalyserOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AnalyserOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AnalyserOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AnalyserOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AnalyserOptions> for Any {
    fn from(s: AnalyserOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AnalyserOptions> for Any {
    fn from(s: &AnalyserOptions) -> Any {
        s.inner.clone()
    }
}

impl AnalyserOptions {
    /// Getter of the `fftSize` attribute.
    pub fn fft_size(&self) -> u32 {
        self.inner.get("fftSize").as_::<u32>()
    }

    /// Setter of the `fftSize` attribute.
    pub fn set_fft_size(&mut self, value: u32) {
        self.inner.set("fftSize", value);
    }
}
impl AnalyserOptions {
    /// Getter of the `maxDecibels` attribute.
    pub fn max_decibels(&self) -> f64 {
        self.inner.get("maxDecibels").as_::<f64>()
    }

    /// Setter of the `maxDecibels` attribute.
    pub fn set_max_decibels(&mut self, value: f64) {
        self.inner.set("maxDecibels", value);
    }
}
impl AnalyserOptions {
    /// Getter of the `minDecibels` attribute.
    pub fn min_decibels(&self) -> f64 {
        self.inner.get("minDecibels").as_::<f64>()
    }

    /// Setter of the `minDecibels` attribute.
    pub fn set_min_decibels(&mut self, value: f64) {
        self.inner.set("minDecibels", value);
    }
}
impl AnalyserOptions {
    /// Getter of the `smoothingTimeConstant` attribute.
    pub fn smoothing_time_constant(&self) -> f64 {
        self.inner.get("smoothingTimeConstant").as_::<f64>()
    }

    /// Setter of the `smoothingTimeConstant` attribute.
    pub fn set_smoothing_time_constant(&mut self, value: f64) {
        self.inner.set("smoothingTimeConstant", value);
    }
}
