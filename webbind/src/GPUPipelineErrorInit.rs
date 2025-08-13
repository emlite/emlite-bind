use super::*;




/// The GPUPipelineErrorInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUPipelineErrorInit {
    inner: Any,
}

impl FromVal for GPUPipelineErrorInit {
    fn from_val(v: &Any) -> Self {
        GPUPipelineErrorInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUPipelineErrorInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUPipelineErrorInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUPipelineErrorInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUPipelineErrorInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUPipelineErrorInit> for Any {
    fn from(s: GPUPipelineErrorInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUPipelineErrorInit> for Any {
    fn from(s: &GPUPipelineErrorInit) -> Any {
        s.inner.clone()
    }
}

impl GPUPipelineErrorInit {
    /// Getter of the `reason` attribute.
    pub fn reason(&self) -> GPUPipelineErrorReason {
        self.inner.get("reason").as_::<GPUPipelineErrorReason>()
    }

    /// Setter of the `reason` attribute.
    pub fn set_reason(&mut self, value: &GPUPipelineErrorReason) {
        self.inner.set("reason", value);
    }
}
