use super::*;




/// The GPUPipelineLayout class.
/// [`GPUPipelineLayout`](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineLayout)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUPipelineLayout {
    inner: Any,
}

impl FromVal for GPUPipelineLayout {
    fn from_val(v: &Any) -> Self {
        GPUPipelineLayout { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUPipelineLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUPipelineLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUPipelineLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUPipelineLayout {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUPipelineLayout> for Any {
    fn from(s: GPUPipelineLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUPipelineLayout> for Any {
    fn from(s: &GPUPipelineLayout) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUPipelineLayout);


impl GPUPipelineLayout {
    /// Getter of the `label` attribute.
    /// [`GPUPipelineLayout.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineLayout/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUPipelineLayout.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineLayout/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
