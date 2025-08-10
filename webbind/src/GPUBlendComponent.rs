use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBlendComponent {
    inner: Any,
}
impl FromVal for GPUBlendComponent {
    fn from_val(v: &Any) -> Self {
        GPUBlendComponent { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUBlendComponent {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBlendComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUBlendComponent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUBlendComponent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUBlendComponent> for Any {
    fn from(s: GPUBlendComponent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUBlendComponent> for Any {
    fn from(s: &GPUBlendComponent) -> Any {
        s.inner.clone()
    }
}

impl GPUBlendComponent {
    pub fn operation(&self) -> GPUBlendOperation {
        self.inner.get("operation").as_::<GPUBlendOperation>()
    }

    pub fn set_operation(&mut self, value: &GPUBlendOperation) {
        self.inner.set("operation", value);
    }
}
impl GPUBlendComponent {
    pub fn src_factor(&self) -> GPUBlendFactor {
        self.inner.get("srcFactor").as_::<GPUBlendFactor>()
    }

    pub fn set_src_factor(&mut self, value: &GPUBlendFactor) {
        self.inner.set("srcFactor", value);
    }
}
impl GPUBlendComponent {
    pub fn dst_factor(&self) -> GPUBlendFactor {
        self.inner.get("dstFactor").as_::<GPUBlendFactor>()
    }

    pub fn set_dst_factor(&mut self, value: &GPUBlendFactor) {
        self.inner.set("dstFactor", value);
    }
}
