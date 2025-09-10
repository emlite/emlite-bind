use super::*;

/// The GPURenderPipeline class.
/// [`GPURenderPipeline`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPipeline)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPipeline {
    inner: Any,
}

impl FromVal for GPURenderPipeline {
    fn from_val(v: &Any) -> Self {
        GPURenderPipeline {
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

impl core::ops::Deref for GPURenderPipeline {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPURenderPipeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPURenderPipeline {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPURenderPipeline {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPURenderPipeline> for Any {
    fn from(s: GPURenderPipeline) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPURenderPipeline> for Any {
    fn from(s: &GPURenderPipeline) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPURenderPipeline);

impl GPURenderPipeline {
    /// Getter of the `label` attribute.
    /// [`GPURenderPipeline.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPipeline/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPURenderPipeline.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPipeline/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl GPURenderPipeline {
    /// The getBindGroupLayout method.
    /// [`GPURenderPipeline.getBindGroupLayout`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPipeline/getBindGroupLayout)
    pub fn get_bind_group_layout(&self, index: u32) -> GPUBindGroupLayout {
        self.inner
            .call("getBindGroupLayout", &[index.into()])
            .as_::<GPUBindGroupLayout>()
    }
}
