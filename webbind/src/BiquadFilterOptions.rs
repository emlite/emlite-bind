use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BiquadFilterOptions {
    inner: Any,
}
impl FromVal for BiquadFilterOptions {
    fn from_val(v: &Any) -> Self {
        BiquadFilterOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BiquadFilterOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BiquadFilterOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BiquadFilterOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BiquadFilterOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BiquadFilterOptions> for Any {
    fn from(s: BiquadFilterOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BiquadFilterOptions> for Any {
    fn from(s: &BiquadFilterOptions) -> Any {
        s.inner.clone()
    }
}

impl BiquadFilterOptions {
    pub fn type_(&self) -> BiquadFilterType {
        self.inner.get("type").as_::<BiquadFilterType>()
    }

    pub fn set_type_(&mut self, value: &BiquadFilterType) {
        self.inner.set("type", value);
    }
}
impl BiquadFilterOptions {
    pub fn q(&self) -> f32 {
        self.inner.get("Q").as_::<f32>()
    }

    pub fn set_q(&mut self, value: f32) {
        self.inner.set("Q", value);
    }
}
impl BiquadFilterOptions {
    pub fn detune(&self) -> f32 {
        self.inner.get("detune").as_::<f32>()
    }

    pub fn set_detune(&mut self, value: f32) {
        self.inner.set("detune", value);
    }
}
impl BiquadFilterOptions {
    pub fn frequency(&self) -> f32 {
        self.inner.get("frequency").as_::<f32>()
    }

    pub fn set_frequency(&mut self, value: f32) {
        self.inner.set("frequency", value);
    }
}
impl BiquadFilterOptions {
    pub fn gain(&self) -> f32 {
        self.inner.get("gain").as_::<f32>()
    }

    pub fn set_gain(&mut self, value: f32) {
        self.inner.set("gain", value);
    }
}
