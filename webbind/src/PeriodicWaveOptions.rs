use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PeriodicWaveOptions {
    inner: Any,
}
impl FromVal for PeriodicWaveOptions {
    fn from_val(v: &Any) -> Self {
        PeriodicWaveOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PeriodicWaveOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PeriodicWaveOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PeriodicWaveOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PeriodicWaveOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PeriodicWaveOptions> for Any {
    fn from(s: PeriodicWaveOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PeriodicWaveOptions> for Any {
    fn from(s: &PeriodicWaveOptions) -> Any {
        s.inner.clone()
    }
}

impl PeriodicWaveOptions {
    pub fn real(&self) -> TypedArray<f32> {
        self.inner.get("real").as_::<TypedArray<f32>>()
    }

    pub fn set_real(&mut self, value: TypedArray<f32>) {
        self.inner.set("real", value);
    }
}
impl PeriodicWaveOptions {
    pub fn imag(&self) -> TypedArray<f32> {
        self.inner.get("imag").as_::<TypedArray<f32>>()
    }

    pub fn set_imag(&mut self, value: TypedArray<f32>) {
        self.inner.set("imag", value);
    }
}
