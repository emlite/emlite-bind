use super::*;

/// The GPUQueue class.
/// [`GPUQueue`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUQueue {
    inner: Any,
}

impl FromVal for GPUQueue {
    fn from_val(v: &Any) -> Self {
        GPUQueue {
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

impl core::ops::Deref for GPUQueue {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUQueue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUQueue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUQueue> for Any {
    fn from(s: GPUQueue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUQueue> for Any {
    fn from(s: &GPUQueue) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUQueue);

impl GPUQueue {
    /// Getter of the `label` attribute.
    /// [`GPUQueue.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUQueue.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl GPUQueue {
    /// The submit method.
    /// [`GPUQueue.submit`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/submit)
    pub fn submit(&self, command_buffers: &TypedArray<GPUCommandBuffer>) -> Undefined {
        self.inner
            .call("submit", &[command_buffers.into()])
            .as_::<Undefined>()
    }
}
impl GPUQueue {
    /// The onSubmittedWorkDone method.
    /// [`GPUQueue.onSubmittedWorkDone`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/onSubmittedWorkDone)
    pub fn on_submitted_work_done(&self) -> Promise<Undefined> {
        self.inner
            .call("onSubmittedWorkDone", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl GPUQueue {
    /// The writeBuffer method.
    /// [`GPUQueue.writeBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)
    pub fn write_buffer(&self, buffer: &GPUBuffer, buffer_offset: &Any, data: &Any) -> Undefined {
        self.inner
            .call(
                "writeBuffer",
                &[buffer.into(), buffer_offset.into(), data.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPUQueue {
    /// The writeBuffer method.
    /// [`GPUQueue.writeBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)
    pub fn write_buffer_with_data_offset(
        &self,
        buffer: &GPUBuffer,
        buffer_offset: &Any,
        data: &Any,
        data_offset: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "writeBuffer",
                &[
                    buffer.into(),
                    buffer_offset.into(),
                    data.into(),
                    data_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl GPUQueue {
    /// The writeBuffer method.
    /// [`GPUQueue.writeBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)
    pub fn write_buffer_with_data_offset_and_size(
        &self,
        buffer: &GPUBuffer,
        buffer_offset: &Any,
        data: &Any,
        data_offset: &Any,
        size: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "writeBuffer",
                &[
                    buffer.into(),
                    buffer_offset.into(),
                    data.into(),
                    data_offset.into(),
                    size.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl GPUQueue {
    /// The writeTexture method.
    /// [`GPUQueue.writeTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)
    pub fn write_texture(
        &self,
        destination: &GPUTexelCopyTextureInfo,
        data: &Any,
        data_layout: &GPUTexelCopyBufferLayout,
        size: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "writeTexture",
                &[
                    destination.into(),
                    data.into(),
                    data_layout.into(),
                    size.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl GPUQueue {
    /// The copyExternalImageToTexture method.
    /// [`GPUQueue.copyExternalImageToTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/copyExternalImageToTexture)
    pub fn copy_external_image_to_texture(
        &self,
        source: &GPUCopyExternalImageSourceInfo,
        destination: &GPUCopyExternalImageDestInfo,
        copy_size: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "copyExternalImageToTexture",
                &[source.into(), destination.into(), copy_size.into()],
            )
            .as_::<Undefined>()
    }
}
