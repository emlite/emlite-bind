use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUPipelineError {
    inner: DOMException,
}
impl FromVal for GPUPipelineError {
    fn from_val(v: &emlite::Val) -> Self {
        GPUPipelineError { inner: DOMException::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUPipelineError {
    type Target = DOMException;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUPipelineError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUPipelineError {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUPipelineError {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<GPUPipelineError> for emlite::Val {
    fn from(s: GPUPipelineError) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GPUPipelineError);



impl GPUPipelineError {
    pub fn new0() -> GPUPipelineError {
        Self {
            inner: emlite::Val::global("GPUPipelineError").new(&[]).as_::<DOMException>(),
        }
    }

    pub fn new1(message: DOMString) -> GPUPipelineError {
        Self {
            inner: emlite::Val::global("GPUPipelineError").new(&[message.into()]).as_::<DOMException>(),
        }
    }

    pub fn new2(message: DOMString, options: Any) -> GPUPipelineError {
        Self {
            inner: emlite::Val::global("GPUPipelineError").new(&[message.into(), options.into()]).as_::<DOMException>(),
        }
    }

}
impl GPUPipelineError {
    pub fn reason(&self) -> GPUPipelineErrorReason {
        self.inner.get("reason").as_::<GPUPipelineErrorReason>()
    }

}
