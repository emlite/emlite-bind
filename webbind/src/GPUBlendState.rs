use super::*;




/// The GPUBlendState dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBlendState {
    inner: Any,
}

impl FromVal for GPUBlendState {
    fn from_val(v: &Any) -> Self {
        GPUBlendState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUBlendState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUBlendState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUBlendState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUBlendState {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUBlendState> for Any {
    fn from(s: GPUBlendState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUBlendState> for Any {
    fn from(s: &GPUBlendState) -> Any {
        s.inner.clone()
    }
}

impl GPUBlendState {
    /// Getter of the `color` attribute.
    pub fn color(&self) -> GPUBlendComponent {
        self.inner.get("color").as_::<GPUBlendComponent>()
    }

    /// Setter of the `color` attribute.
    pub fn set_color(&mut self, value: &GPUBlendComponent) {
        self.inner.set("color", value);
    }
}
impl GPUBlendState {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> GPUBlendComponent {
        self.inner.get("alpha").as_::<GPUBlendComponent>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: &GPUBlendComponent) {
        self.inner.set("alpha", value);
    }
}
