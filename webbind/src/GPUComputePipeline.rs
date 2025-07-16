use super::*;

/// The GPUComputePipeline class.
/// [`GPUComputePipeline`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePipeline)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUComputePipeline {
    inner: Any,
}
impl FromVal for GPUComputePipeline {
    fn from_val(v: &Any) -> Self {
        GPUComputePipeline {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUComputePipeline {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUComputePipeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUComputePipeline {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUComputePipeline {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUComputePipeline> for Any {
    fn from(s: GPUComputePipeline) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUComputePipeline> for Any {
    fn from(s: &GPUComputePipeline) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUComputePipeline);

impl GPUComputePipeline {
    /// Getter of the `label` attribute.
    /// [`GPUComputePipeline.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePipeline/label)
    pub fn label(&self) -> String {
        self.inner.get("label").as_::<String>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUComputePipeline.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePipeline/label)
    pub fn set_label(&mut self, value: &str) {
        self.inner.set("label", value);
    }
}
impl GPUComputePipeline {
    /// The getBindGroupLayout method.
    /// [`GPUComputePipeline.getBindGroupLayout`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePipeline/getBindGroupLayout)
    pub fn get_bind_group_layout(&self, index: u32) -> GPUBindGroupLayout {
        self.inner
            .call("getBindGroupLayout", &[index.into()])
            .as_::<GPUBindGroupLayout>()
    }
}
